use std::{
  collections::HashMap,
  fs::{self, File, OpenOptions},
  io::{self, Read, Write},
  os::unix::fs::{FileExt, MetadataExt},
  sync::{Arc, Mutex},
};

use chrono::{DateTime, NaiveDateTime, Utc};
use serde;
use ttlog::ttlog_macros::{error, info, trace};

const FILE_THRESHOLD: u64 = 1024; // 1KB

#[derive(Debug)]
struct MetaIndex {
  timestamp: i64,
  key_size: usize,
  key_buf: Vec<u8>,
  value_size: usize,
  value_buf: Vec<u8>,
}

#[derive(Debug)]
struct Index {
  file_id: u64,
  offset: u64,
}

#[derive(Debug)]
pub struct LogFile {
  inner: Arc<Mutex<Inner>>,
}

#[derive(Debug)]
struct Inner {
  byte_offset: u64,
  current_file_id: u64,
  path: String,
  data_index: HashMap<String, Index>,
  file_index: HashMap<u64, String>,
}

impl Default for LogFile {
  fn default() -> Self {
    Self::new()
  }
}

impl LogFile {
  pub fn new() -> Self {
    Self {
      inner: Arc::new(Mutex::new(Inner {
        path: "".to_string(),
        byte_offset: 0x1,
        current_file_id: 0x1,
        data_index: HashMap::new(),
        file_index: HashMap::new(),
      })),
    }
  }

  fn read_hint_file(&self) -> Result<(), std::io::Error> {
    let mut inner = self.inner.lock().unwrap();
    let path = format!("./tmp/hint-{}.log", inner.current_file_id);
    if !fs::exists(&path)? {
      return Ok(());
    }

    let hint_file = OpenOptions::new().read(true).open(&path)?;
    let mut offset = 0;

    loop {
      if fs::metadata(&path)?.size() <= offset {
        break;
      }

      let mut key_size_buf = [0u8; 8];
      hint_file.read_exact_at(&mut key_size_buf, offset)?;
      let key_size = u64::from_le_bytes(key_size_buf);
      offset += 8;

      let mut key_buf = vec![0u8; key_size as usize];
      hint_file.read_exact_at(&mut key_buf, offset)?;
      let key_value = String::from_utf8(key_buf.clone()).unwrap();
      offset += key_size;

      // adding because here we read the timestamp
      offset += 8;

      let mut file_id_buf = [0u8; 8];
      hint_file.read_exact_at(&mut file_id_buf, offset)?;
      let file_id = u64::from_le_bytes(file_id_buf);
      offset += 8;

      let mut offset_buf = [0u8; 8];
      hint_file.read_exact_at(&mut offset_buf, offset)?;
      let offset_value = u64::from_le_bytes(offset_buf);
      offset += 8;

      inner.data_index.insert(
        key_value,
        Index {
          offset: offset_value,
          file_id,
        },
      );
    }

    Ok(())
  }

  pub fn start(&self) -> Result<(), std::io::Error> {
    fs::create_dir_all("tmp")?;

    // regenrate the index from the hint file
    self.read_hint_file()?;

    let mut inner = self.inner.lock().unwrap();

    // regenrate the index from the file list
    let mut files = fs::read_dir("./tmp")?
      .filter_map(|entry| entry.ok())
      .filter_map(|entry| {
        let path = entry.path();
        let file_name = path.file_name()?.to_str()?;

        // check the prefix
        if let Some(number_str) = file_name.strip_prefix("log-file-") {
          // check that the rest is a number
          if number_str.parse::<u64>().is_ok() {
            return Some(path);
          }
        }

        None
      })
      .collect::<Vec<_>>();

    files.sort_by_key(|path| {
      path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_prefix("log-file-")
        .unwrap()
        .parse::<u64>()
        .unwrap()
    });

    for file_path in &files {
      let file = File::open(file_path)?;
      let file_id = file_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_prefix("log-file-")
        .unwrap()
        .parse::<u64>()
        .unwrap();
      let metadata = fs::metadata(file_path)?;

      inner
        .file_index
        .insert(file_id, file_path.to_str().unwrap().to_string());

      let mut offset = 0;
      loop {
        if metadata.size() <= offset {
          break;
        }

        let index = Index { offset, file_id };

        let meta = match self.get_index_from_file(&mut offset, &file) {
          Ok(meta) => meta,
          Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
          Err(e) => return Err(e),
        };
        let key = String::from_utf8(meta.key_buf.clone()).unwrap();

        if meta.value_buf.is_empty() {
          inner.data_index.remove(&key);
          continue;
        }

        inner.data_index.insert(key, index);
      }
    }

    let id = files
      .last()
      .map(|path| {
        path
          .file_name()
          .unwrap()
          .to_str()
          .unwrap()
          .strip_prefix("log-file-")
          .unwrap()
          .parse::<u64>()
          .unwrap()
      })
      .unwrap_or(0x1);
    inner.current_file_id = id + 1;

    drop(inner);

    self.create()?;
    Ok(())
  }

  pub fn create(&self) -> Result<(), std::io::Error> {
    let mut inner = self.inner.lock().unwrap();
    let path = format!("./tmp/log-file-{}", inner.current_file_id);

    OpenOptions::new().create(true).append(true).open(&path)?;
    inner.path = path;
    let path = inner.path.clone();
    let id = inner.current_file_id;
    inner.file_index.insert(id, path);
    inner.byte_offset = 0;

    trace!(
      "[LOGFILE] Log file has been created successfully.",
      file_id = inner.current_file_id
    );
    Ok(())
  }

  pub fn append(&self, key: &str, value: &str) -> Result<(), io::Error> {
    let mut inner = self.inner.lock().unwrap();
    if key.is_empty() {
      error!("The index length should be at least 1 character");
      return Err(io::Error::other(""));
    }

    let data_size = (value.len() + key.len() + 8 * 3) as u64;
    let index_value = Index {
      offset: inner.byte_offset,
      file_id: inner.current_file_id,
    };

    inner.data_index.insert(key.to_string(), index_value);
    inner.byte_offset += data_size;

    let timestamp = Utc::now().timestamp_nanos_opt().unwrap();

    drop(inner);
    self.insert_index_value(MetaIndex {
      timestamp,
      key_size: key.len(),
      key_buf: key.as_bytes().to_vec(),
      value_size: value.len(),
      value_buf: value.as_bytes().to_vec(),
    })?;

    info!("[WRITE]", index_value = value.to_string());
    Ok(())
  }

  pub fn read(&self, id: &str) -> Result<String, io::Error> {
    if !self.inner.lock().unwrap().data_index.contains_key(id) {
      return Err(io::Error::other("This key does not exist in the index"));
    }

    let index = self.get_index_value(id)?;

    // let timestamp = Utc.timestamp_opt(index.timestamp, 0);
    // let timestamp = timestamp.unwrap().to_string();
    // let index_key_value = String::from_utf8(index.key_buf).unwrap().to_string();
    let value = String::from_utf8(index.value_buf).unwrap().to_string();
    info!("[READ]", key = id.to_string(), value = value);
    Ok(value)
  }

  pub fn update(&self, key: &str, value: &'static str) -> Result<(), io::Error> {
    let mut inner = self.inner.lock().unwrap();
    if key.is_empty() {
      error!("The index length should be at least 1 character");
      return Err(io::Error::other(""));
    }

    if !inner.data_index.contains_key(key) {
      return Err(io::Error::other("This key does not exist in the index"));
    }

    let index_value = Index {
      offset: inner.byte_offset,
      file_id: inner.current_file_id,
    };

    let data_size = (value.len() + key.len() + 8 * 2) as u64;

    inner.data_index.insert(key.to_string(), index_value);
    inner.byte_offset += data_size;

    let timestamp = Utc::now().timestamp();

    drop(inner);
    self.insert_index_value(MetaIndex {
      timestamp,
      key_size: key.len(),
      key_buf: key.as_bytes().to_vec(),
      value_size: value.len(),
      value_buf: value.as_bytes().to_vec(),
    })?;

    info!("[UPDATE]", key = key.to_string(), value = value);

    Ok(())
  }

  pub fn delete(&self, id: &str) -> Result<(), io::Error> {
    let mut index = self.get_index_value(id)?;
    let value = String::from_utf8(index.value_buf.clone())
      .unwrap()
      .to_string();
    index.value_size = 0;
    index.value_buf.clear();
    self.insert_index_value(index)?;
    self.inner.lock().unwrap().data_index.remove(id);

    info!("[DELETE]", key = id.to_string(), value = value);
    Ok(())
  }

  pub fn compact(&self) -> Result<(), io::Error> {
    let new_hash = std::mem::take(&mut self.inner.lock().unwrap().file_index);
    let mut end_file = HashMap::<String, MetaIndex>::new();
    let mut sorted_file_ids = new_hash.keys().collect::<Vec<_>>();
    sorted_file_ids.sort();

    for &file_id in sorted_file_ids {
      let file_idx = new_hash.get(&file_id).unwrap();
      self.compact_file(&mut end_file, file_idx)?
    }
    let _ = core::mem::replace(&mut self.inner.lock().unwrap().file_index, new_hash);

    let temp_file_path = format!(
      "./tmp/temp-log-file-{}",
      Utc::now().timestamp_nanos_opt().unwrap()
    );
    let mut temp_file = File::create(&temp_file_path)?;

    // Keep record layout identical to append: ts, key_size, value_size, key, value.
    for (_, value) in end_file.iter() {
      temp_file.write_all(&value.timestamp.to_le_bytes())?;
      temp_file.write_all(&value.key_size.to_le_bytes())?;
      temp_file.write_all(&value.value_size.to_le_bytes())?;
      temp_file.write_all(&value.key_buf)?;
      temp_file.write_all(&value.value_buf)?;

      // CRASH SAFETY HERE
      temp_file.sync_all()?; // durability guarantee
    }

    temp_file.flush()?;

    self.inner.lock().unwrap().current_file_id = 1;
    let path = format!(
      "./tmp/log-file-{}.log",
      self.inner.lock().unwrap().current_file_id
    );
    for (_, path) in self.inner.lock().unwrap().file_index.iter() {
      fs::remove_file(path)?;
    }

    drop(temp_file);
    fs::rename(&temp_file_path, &path)?;

    let mut inner = self.inner.lock().unwrap();
    let current_file_id = inner.current_file_id;
    inner.path = path.clone();
    inner.file_index.insert(current_file_id, path);
    info!("[COMPACT] Compaction has been completed successfully.");

    drop(inner);
    self.write_hint_file()?;
    Ok(())
  }

  fn write_hint_file(&self) -> Result<(), io::Error> {
    let inner = self.inner.lock().unwrap();
    let path = format!("./tmp/hint-{}.log", inner.current_file_id);
    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;

    for (key, value) in inner.data_index.iter() {
      let timestamp = Utc::now().timestamp();
      file.write_all(&key.len().to_le_bytes())?;
      file.write_all(key.as_bytes())?;
      file.write_all(&timestamp.to_le_bytes())?;
      file.write_all(&value.file_id.to_le_bytes())?;
      file.write_all(&value.offset.to_le_bytes())?;
    }

    info!("[HINT] Hint file has been written successfully.");

    Ok(())
  }

  fn compact_file(
    &self,
    end_file: &mut HashMap<String, MetaIndex>,
    file_idx: &String,
  ) -> Result<(), io::Error> {
    let mut offset = 0;
    let file = File::open(file_idx)?;
    let meta_data = fs::metadata(file_idx)?;

    loop {
      if meta_data.size() <= offset {
        break;
      }

      let meta = self.get_index_from_file(&mut offset, &file)?;
      let key = String::from_utf8(meta.key_buf.clone()).unwrap();

      if meta.value_buf.is_empty() {
        end_file.remove(&key);
        continue;
      }

      end_file.insert(key, meta);
    }

    Ok(())
  }

  fn insert_index_value(&self, meta: MetaIndex) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
      .append(true)
      .open(&self.inner.lock().unwrap().path)?;

    file.write_all(&meta.timestamp.to_le_bytes())?;
    file.write_all(&meta.key_size.to_le_bytes())?;
    file.write_all(&meta.value_size.to_le_bytes())?;
    file.write_all(&meta.key_buf)?;
    file.write_all(&meta.value_buf)?;

    // CRASH SAFETY HERE
    file.sync_all()?; // durability guarantee

    // FILE SEGMENTATION HERE
    self.split()?;

    Ok(())
  }

  fn get_index_value(&self, id: &str) -> Result<MetaIndex, io::Error> {
    let inner = self.inner.lock().unwrap();
    if !inner.data_index.contains_key(id) {
      return Err(io::Error::other(""));
    }

    let index = inner.data_index.get(id).unwrap();
    let file = File::open(inner.file_index.get(&index.file_id).unwrap())?;
    let mut offset = index.offset;

    drop(inner);
    self.get_index_from_file(&mut offset, &file)
  }

  fn get_index_from_file(&self, offset: &mut u64, file: &File) -> Result<MetaIndex, io::Error> {
    let mut ts_buff = [0u8; 8];
    file.read_exact_at(&mut ts_buff, *offset)?;
    let timestamp = i64::from_le_bytes(ts_buff);
    *offset += 8;

    let mut key_size_buf = [0u8; 8];
    file.read_exact_at(&mut key_size_buf, *offset)?;
    let key_size = u64::from_le_bytes(key_size_buf) as usize;
    *offset += 8;

    let mut value_size_buf = [0u8; 8];
    file.read_exact_at(&mut value_size_buf, *offset)?;
    let value_size = u64::from_le_bytes(value_size_buf) as usize;
    *offset += 8;

    let file_size = file.metadata()?.size();
    if *offset + key_size as u64 + value_size as u64 > file_size {
      return Err(io::Error::new(
        io::ErrorKind::UnexpectedEof,
        "Corrupted record: claimed size exceeds file",
      ));
    }

    let mut key_buf = vec![0u8; key_size];
    file.read_exact_at(&mut key_buf, *offset)?;
    *offset += key_size as u64;

    let mut value_buf = vec![0u8; value_size];
    file.read_exact_at(&mut value_buf, *offset)?;
    *offset += value_size as u64;

    Ok(MetaIndex {
      timestamp,
      key_size,
      key_buf,
      value_size,
      value_buf,
    })
  }

  fn split(&self) -> Result<(), io::Error> {
    let metadata = fs::metadata(&self.inner.lock().unwrap().path)?;

    if metadata.size() > FILE_THRESHOLD {
      trace!(
        "[LOGFILE] File has exceeded the threshold",
        threshold = FILE_THRESHOLD,
        file_size = metadata.size()
      );

      self.inner.lock().unwrap().current_file_id += 1;
      self.create()?;
    }
    Ok(())
  }
}
