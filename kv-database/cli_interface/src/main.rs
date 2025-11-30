use std::{
  fs::{self, OpenOptions},
  sync::Arc,
};

use core_engine::log_file;
use ttlog::{
  file_listener::FileListener,
  stdout_listener::StdoutListener,
  trace::Trace,
  ttlog_macros::{debug, trace},
};

const PERIODIC_COMPACTION_INTERVAL: u64 = 60 * 10;

fn main() -> Result<(), std::io::Error> {
  if fs::exists("./tmp")? {
    fs::remove_dir_all("./tmp")?;
  }

  let trace = Trace::init(2, 64, "test", Some("./tmp"));
  trace.add_listener(Arc::new(FileListener::new("./tmp/ttlog.log")?));
  trace.add_listener(Arc::new(StdoutListener::new()));
  trace.set_level(ttlog::event::LogLevel::TRACE);

  let mut log_file = log_file::LogFile::new();
  log_file.create().unwrap_or(());

  for i in 0..5 {
    log_file.append(&format!("123:{}", i), "{\"name\":\"wildduck\",\"age\":25}")?;
  }
  log_file.append("123:5", "{\"name\":\"wildduck\",\"age\":25}")?;
  log_file.delete("123:1")?;
  log_file.update("123:5", "{\"name\":\"wildduck\",\"age\":28}")?;
  log_file.read("123:4")?;

  // trace!("[LOGFILE]", file_size = log_file.get_file_size());

  let handle = std::thread::spawn(move || loop {
    let _ = log_file.compact();
    std::thread::sleep(std::time::Duration::from_secs(PERIODIC_COMPACTION_INTERVAL));
  });

  let _ = handle.join();
  Ok(())
}
