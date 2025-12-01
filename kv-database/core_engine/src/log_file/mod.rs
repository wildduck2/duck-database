use std::{
  collections::HashMap,
  fs::{self, File, OpenOptions},
  io::{self, Write},
  os::unix::fs::{FileExt, MetadataExt},
};

use chrono::Utc;
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
  byte_offset: u64,
  current_file_id: u64,
  path: String,
  data_index: HashMap<String, Index>,
  pub file_index: HashMap<u64, String>,
}

impl Default for LogFile {
  fn default() -> Self {
    Self::new()
  }
}

impl LogFile {
  pub fn new() -> Self {
    Self {
      path: "".to_string(),
      byte_offset: 0x1,
      current_file_id: 0x1,
      data_index: HashMap::new(),
      file_index: HashMap::new(),
    }
  }

  pub fn start(&mut self) -> Result<(), std::io::Error> {
    let files = fs::read_dir("./tmp")?
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

    for file_path in files {
      let file = File::open(&file_path)?;
      let metadata = fs::metadata(file_path)?;
      let mut offset = 0;

      loop {
        if metadata.size() <= offset {
          break;
        }

        let index = Index {
          offset,
          file_id: self.current_file_id,
        };

        let meta = self.get_index_from_file(&mut offset, &file)?;
        let key = String::from_utf8(meta.key_buf.clone()).unwrap();

        if meta.value_buf.is_empty() {
          self.data_index.remove(&key);
          continue;
        }

        self.data_index.insert(key, index);
      }
    }
    println!("{:#?}", self.data_index);

    Ok(())
  }

  pub fn create(&mut self) -> Result<(), std::io::Error> {
    fs::create_dir_all("tmp")?;
    let path = format!("./tmp/log-file-{}", self.current_file_id);

    OpenOptions::new().create(true).append(true).open(&path)?;
    self.path = path;
    self
      .file_index
      .insert(self.current_file_id, self.path.clone());
    self.byte_offset = 0;

    trace!(
      "[LOGFILE] Log file has been created successfully.",
      file_id = self.current_file_id
    );
    Ok(())
  }

  pub fn append(&mut self, key: &str, value: &'static str) -> Result<(), io::Error> {
    if key.is_empty() {
      error!("The index length should be at least 1 character");
      return Err(io::Error::other(""));
    }

    let data_size = (value.len() + key.len() + 8 * 3) as u64;
    let index_value = Index {
      offset: self.byte_offset,
      file_id: self.current_file_id,
    };

    self.data_index.insert(key.to_string(), index_value);
    self.byte_offset += data_size;

    let timestamp = Utc::now().timestamp_nanos_opt().unwrap();

    self.insert_index_value(MetaIndex {
      timestamp,
      key_size: key.len(),
      key_buf: key.as_bytes().to_vec(),
      value_size: value.len(),
      value_buf: value.as_bytes().to_vec(),
    })?;

    info!("[WRITE]", index_value = value);
    Ok(())
  }

  pub fn read(&mut self, id: &str) -> Result<String, io::Error> {
    if !self.data_index.contains_key(id) {
      return Err(io::Error::other("This key does not exist in the index"));
    }

    let index = self.get_index_value(id)?;

    // let timestamp = Utc.timestamp_opt(index.timestamp, 0);
    // let timestamp = timestamp.unwrap().to_string();
    // let index_key_value = String::from_utf8(index.key_buf).unwrap().to_string();
    let index_value_value = String::from_utf8(index.value_buf).unwrap().to_string();
    info!("[READ]", value = index_value_value);
    Ok(index_value_value)
  }

  pub fn update(&mut self, key: &str, value: &'static str) -> Result<(), io::Error> {
    if key.is_empty() {
      error!("The index length should be at least 1 character");
      return Err(io::Error::other(""));
    }

    if !self.data_index.contains_key(key) {
      return Err(io::Error::other("This key does not exist in the index"));
    }

    let index_value = Index {
      offset: self.byte_offset,
      file_id: self.current_file_id,
    };

    self.data_index.insert(key.to_string(), index_value);

    let data_size = (value.len() + key.len() + 8 * 2) as u64;
    let index_value = Index {
      offset: self.byte_offset,
      file_id: self.current_file_id,
    };

    self.data_index.insert(key.to_string(), index_value);
    self.byte_offset += data_size;

    let timestamp = Utc::now().timestamp();

    self.insert_index_value(MetaIndex {
      timestamp,
      key_size: key.len(),
      key_buf: key.as_bytes().to_vec(),
      value_size: value.len(),
      value_buf: value.as_bytes().to_vec(),
    })?;

    info!("[UPDATE]", index_value = value);

    Ok(())
  }

  pub fn delete(&mut self, id: &str) -> Result<String, io::Error> {
    let mut index = self.get_index_value(id)?;
    let value = String::from_utf8(index.value_buf.clone())
      .unwrap()
      .to_string();
    index.value_size = 0;
    index.value_buf.clear();
    self.insert_index_value(index)?;
    self.data_index.remove(id);

    info!("[DELETE]", index_value = value);
    Ok("".to_string())
  }

  pub fn compact(&mut self) -> Result<(), io::Error> {
    let new_hash = std::mem::take(&mut self.file_index);
    let mut end_file = HashMap::<String, MetaIndex>::new();
    let mut sorted_file_ids = new_hash.keys().collect::<Vec<_>>();
    sorted_file_ids.sort();

    for &file_id in sorted_file_ids {
      let file_idx = new_hash.get(&file_id).unwrap();
      self.compact_file(&mut end_file, file_idx)?
    }
    let _ = core::mem::replace(&mut self.file_index, new_hash);

    let temp_file_path = format!(
      "./tmp/temp-log-file-{}",
      Utc::now().timestamp_nanos_opt().unwrap()
    );
    let mut temp_file = File::create(&temp_file_path)?;

    for (_, value) in end_file.iter() {
      temp_file.write_all(&value.timestamp.to_le_bytes())?;
      temp_file.write_all(&value.key_size.to_le_bytes())?;
      temp_file.write_all(&value.key_buf)?;
      temp_file.write_all(&value.value_size.to_le_bytes())?;
      temp_file.write_all(&value.value_buf)?;
    }

    temp_file.flush()?;
    let path = format!("./tmp/log-file-{}", self.current_file_id + 1);

    drop(temp_file);
    fs::rename(&temp_file_path, &path)?;

    for (_, path) in self.file_index.iter() {
      fs::remove_file(path)?;
    }

    self.current_file_id += 1;
    self.file_index.insert(self.current_file_id, path);

    info!("[COMPACT] Compaction has been completed successfully.");
    Ok(())
  }

  fn compact_file(
    &mut self,
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

  fn insert_index_value(&mut self, meta: MetaIndex) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().append(true).open(&self.path)?;

    file.write_all(&meta.timestamp.to_le_bytes())?;
    file.write_all(&meta.key_size.to_le_bytes())?;
    file.write_all(&meta.value_size.to_le_bytes())?;
    file.write_all(&meta.key_buf)?;
    file.write_all(&meta.value_buf)?;
    self.split()?;

    Ok(())
  }

  fn get_index_value(&mut self, id: &str) -> Result<MetaIndex, io::Error> {
    if !self.data_index.contains_key(id) {
      return Err(io::Error::other(""));
    }

    let index = self.data_index.get(id).unwrap();
    let file = File::open(self.file_index.get(&index.file_id).unwrap())?;
    let mut offset = index.offset;
    self.get_index_from_file(&mut offset, &file)
  }

  fn get_index_from_file(&mut self, offset: &mut u64, file: &File) -> Result<MetaIndex, io::Error> {
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

  fn split(&mut self) -> Result<(), io::Error> {
    let metadata = fs::metadata(&self.path)?;

    if metadata.size() > FILE_THRESHOLD {
      trace!(
        "[LOGFILE] File has exceeded the threshold",
        threshold = FILE_THRESHOLD,
        file_size = metadata.size()
      );

      self.current_file_id += 1;
      self.create()?;
    }
    Ok(())
  }
}
