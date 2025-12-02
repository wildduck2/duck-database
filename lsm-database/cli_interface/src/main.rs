use std::sync::Arc;

use core_engine::log_file::{self, PERIODIC_COMPACTION_INTERVAL};
use ttlog::{file_listener::FileListener, stdout_listener::StdoutListener, trace::Trace};

fn main() -> Result<(), std::io::Error> {
  let trace = Trace::init(2, 64, "test", Some("./tmp"));
  trace.add_listener(Arc::new(FileListener::new("./tmp/ttlog.log")?));
  trace.add_listener(Arc::new(StdoutListener::new()));
  trace.set_level(ttlog::event::LogLevel::TRACE);
  //
  // let log_file = log_file::LogFile::new()?;
  // log_file.start()?;
  //
  // for i in 0..4 {
  //   log_file.append(
  //     &format!("123:{}", 1),
  //     &format!("\"name\":\"wildduck\",\"age\":{}", i + 1),
  //   )?;
  // }
  // log_file.append("123:5", "{\"name\":\"wildduck\",\"age\":25}")?;
  // // log_file.delete("123:1")?;
  // log_file.update("123:5", "{\"name\":\"wildduck\",\"age\":28}")?;
  // // log_file.read("123:400")?;
  // // log_file.read("123:1")?;
  // // log_file.read("123:5")?;
  //
  // let handle = std::thread::spawn(move || loop {
  //   let _ = log_file.compact();
  //
  //   // log_file.read("123:1");
  //
  //   std::thread::sleep(std::time::Duration::from_secs(PERIODIC_COMPACTION_INTERVAL));
  // });
  //
  // let _ = handle.join();
  Ok(())
}
