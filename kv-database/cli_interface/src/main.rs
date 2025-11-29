use std::{fs, sync::Arc};

use core_engine::log_file;
use ttlog::{
  file_listener::FileListener, stdout_listener::StdoutListener, trace::Trace, ttlog_macros::debug,
};

fn main() -> Result<(), std::io::Error> {
  if fs::exists("./tmp")? {
    fs::remove_dir_all("./tmp")?;
  }

  let trace = Trace::init(2, 64, "test", Some("./tmp"));
  trace.add_listener(Arc::new(FileListener::new("./tmp/ttlog.log")?));
  trace.add_listener(Arc::new(StdoutListener::new()));
  trace.set_level(ttlog::event::LogLevel::TRACE);

  let mut log_file = log_file::LogFile::new();
  log_file.create("./tmp/log_file".to_string()).unwrap_or(());

  log_file.append("123", "{\"name\":\"wilddcuk\",\"age\":25}")?;
  log_file.read("123")?;
  log_file.delete("123")?;
  // log_file.read("123")?;

  let handle = std::thread::spawn(|| loop {
    debug!("Waiting for compaction");
    std::thread::sleep(std::time::Duration::from_secs(10));
  });

  handle.join();
  Ok(())
}
