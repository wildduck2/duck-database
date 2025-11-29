use std::{
  collections::HashMap,
  fs::{self, File, OpenOptions},
  io::{self, Write},
  os::unix::fs::FileExt,
};

use chrono::{TimeZone, Utc};
use serde;
use ttlog::ttlog_macros::{error, info, trace};

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
  total_size: u64,
}

#[derive(Debug)]
pub struct LogFile {
  index: HashMap<String, Index>,
  byte_offset: u64,
  current_file_id: u64,
  path: String,
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
      byte_offset: 0,
      current_file_id: 0x1,
      index: HashMap::new(),
    }
  }

  pub fn create(&mut self, path: String) -> Result<(), std::io::Error> {
    fs::create_dir_all("tmp")?;
    OpenOptions::new().create(true).append(true).open(&path)?;
    self.path = path;

    trace!("Log file has been created successfully.");
    Ok(())
  }

  pub fn append(&mut self, key: &str, value: &'static str) -> Result<(), io::Error> {
    if key.is_empty() {
      error!("The index length should be at least 1 character");
      return Err(io::Error::other(""));
    }

    let data_size = (value.len() + key.len() + 8 * 2) as u64;
    let index_value = Index {
      offset: self.byte_offset,
      file_id: self.current_file_id,
      total_size: data_size,
    };

    self.index.insert(key.to_string(), index_value);
    self.byte_offset += data_size;

    let timestamp = Utc::now().timestamp();

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
    if !self.index.contains_key(id) {
      return Err(io::Error::other("This key does not exist in the index"));
    }

    let index = self.get_index_value(id)?;
    // let timestamp = Utc.timestamp_opt(index.timestamp, 0);
    // let timestamp = timestamp.unwrap().to_string();
    // let index_key_value = String::from_utf8(index.key_buf).unwrap().to_string();
    let index_value_value = String::from_utf8(index.value_buf).unwrap().to_string();
    info!("[READ]", index_value = index_value_value);
    Ok(index_value_value)
  }

  pub fn delete(&mut self, id: &str) -> Result<String, io::Error> {
    let mut index = self.get_index_value(id)?;
    let value = String::from_utf8(index.value_buf.clone())
      .unwrap()
      .to_string();
    index.value_buf.clear();
    self.insert_index_value(index)?;
    self.index.remove(id);

    info!("[DELETE]", index_value = value);
    Ok("".to_string())
  }

  pub fn compact(&self) {
    println!("compacting data");
  }

  pub fn split(&self) {
    println!("splitting data");
  }

  fn insert_index_value(&mut self, meta: MetaIndex) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().append(true).open(&self.path)?;
    file.write_all(&meta.timestamp.to_le_bytes())?;
    file.write_all(&meta.key_size.to_le_bytes())?;
    file.write_all(&meta.key_buf)?;
    file.write_all(&meta.value_size.to_le_bytes())?;
    file.write_all(&meta.value_buf)?;

    Ok(())
  }

  fn get_index_value(&mut self, id: &str) -> Result<MetaIndex, io::Error> {
    let index = self.index.get(id).unwrap();
    let file = File::open(&self.path)?;
    let mut offset = index.offset;

    let mut ts_buff = [0u8; 8];
    file.read_exact_at(&mut ts_buff, offset)?;
    let timestamp = i64::from_le_bytes(ts_buff);
    offset += 8;

    let mut key_size_buf = [0u8; 8];
    file.read_exact_at(&mut key_size_buf, offset)?;
    let key_size = u64::from_le_bytes(key_size_buf) as usize;
    offset += 8;

    let mut key_buf = vec![0u8; key_size];
    file.read_exact_at(&mut key_buf, offset)?;
    offset += key_size as u64;

    let mut value_size_buf = [0u8; 8];
    file.read_exact_at(&mut value_size_buf, offset)?;
    let value_size = u64::from_le_bytes(value_size_buf) as usize;
    offset += 8;

    let mut value_buf = vec![0u8; value_size];
    file.read_exact_at(&mut value_buf, offset)?;

    Ok(MetaIndex {
      timestamp,
      key_size,
      key_buf,
      value_size,
      value_buf,
    })
  }
}
