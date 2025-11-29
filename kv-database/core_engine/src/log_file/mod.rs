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
struct Index {
  file_id: u64,
  offset: u64,
  total_size: u64,
}

#[derive(Debug)]
pub struct LogFile {
  index: HashMap<String, Index>,
  current_byte_offset: u64,
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
      current_byte_offset: 0,
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

  pub fn append(&mut self, index_key: &str, data: &'static str) -> Result<(), io::Error> {
    if index_key.is_empty() {
      error!("The index length should be at least 1 character");
      return Err(io::Error::other(""));
    }

    let data_size = (data.len() + 8 * 3) as u64;
    let index_value = Index {
      offset: self.current_byte_offset,
      file_id: self.current_file_id,
      total_size: data_size,
    };

    self.index.insert(index_key.to_string(), index_value);
    self.current_byte_offset += data_size;

    let ts = Utc::now().timestamp();

    let mut file = OpenOptions::new().write(true).open(&self.path)?;
    file.write_all(&ts.to_le_bytes())?;
    file.write_all(&index_key.len().to_le_bytes())?;
    file.write_all(index_key.as_bytes())?;
    file.write_all(&data.len().to_le_bytes())?;
    file.write_all(data.as_bytes())?;

    info!("[WRITE]", index_value = data);
    Ok(())
  }

  pub fn read(&mut self, id: &str) -> Result<String, io::Error> {
    let index = self.index.get(id).unwrap();
    let file = File::open(&self.path)?;
    let mut offset = index.offset;

    let mut ts_buff = [0u8; 8];
    file.read_exact_at(&mut ts_buff, offset)?;
    let seconds_i64 = i64::from_le_bytes(ts_buff);
    let timestamp = Utc.timestamp_opt(seconds_i64, 0);
    offset += 8;

    let mut index_key_size_buf = [0u8; 8];
    file.read_exact_at(&mut index_key_size_buf, offset)?;
    let index_key_zize = u64::from_le_bytes(index_key_size_buf) as usize;
    offset += 8;

    let mut index_key_buf = vec![0; index_key_zize];
    file.read_exact_at(&mut index_key_buf, offset)?;
    offset += index_key_zize as u64;

    let mut index_value_size_buf = [0u8; 8];
    file.read_exact_at(&mut index_value_size_buf, offset)?;
    let index_value_size = u64::from_le_bytes(index_value_size_buf) as usize;
    offset += 8;

    let mut index_value_buf = vec![0; index_value_size];
    file.read_exact_at(&mut index_value_buf, offset)?;

    let _ts = timestamp.unwrap().to_string();
    let _key = String::from_utf8(index_key_buf).unwrap().to_string();
    let value = String::from_utf8(index_value_buf).unwrap().to_string();

    info!("[READ]", index_value = value);

    Ok(value)
  }

  pub fn delete(&mut self) {
    println!("deleting data");
  }

  pub fn compact(&self) {
    println!("compacting data");
  }

  pub fn split(&self) {
    println!("splitting data");
  }
}
