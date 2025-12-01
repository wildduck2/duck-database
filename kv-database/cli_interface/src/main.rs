use std::{
  fs::{self},
  sync::Arc,
};

// ✖ You do not rebuild the index on startup.
// Right now it only exists in memory during runtime.

// ✖ You write value_size after key bytes
// ✖ Bitcask writes key_size then value_size before data
// ✖ You use 8 bytes for key size and value size
// Bitcask uses 4 bytes by default

// On startup:
// load all data files ordered by file_id
// scan each record sequentially
// rebuild index by keeping only latest entry for each key
// skip tombstoned keys
// record all file sizes for compaction
// Your code:
// ✖ does not implement this
// Right now you assume the index exists in memory.

// ✖ does not call fsync
// If process dies mid-write, partial record may corrupt future reads.
// ✖ not thread safe
// ✖ uses mutable HashMap
// ✖ no locks

use core_engine::log_file;
use ttlog::{file_listener::FileListener, stdout_listener::StdoutListener, trace::Trace};

const PERIODIC_COMPACTION_INTERVAL: u64 = 60 * 10;

fn main() -> Result<(), std::io::Error> {
  let trace = Trace::init(2, 64, "test", Some("./tmp"));
  trace.add_listener(Arc::new(FileListener::new("./tmp/ttlog.log")?));
  trace.add_listener(Arc::new(StdoutListener::new()));
  trace.set_level(ttlog::event::LogLevel::TRACE);

  let mut log_file = log_file::LogFile::new();

  log_file.start()?;

  for i in 0..400 {
    log_file.append(
      &format!("123:{}", i + 1),
      &format!("\"name\":\"wildduck\",\"age\":{}", i + 1),
    )?;
  }
  log_file.append("123:5", "{\"name\":\"wildduck\",\"age\":25}")?;
  // log_file.delete("123:1")?;
  // log_file.update("123:5", "{\"name\":\"wildduck\",\"age\":28}")?;
  log_file.read("123:400")?;

  // trace!("[LOGFILE]", file_size = log_file.get_file_size());

  let handle = std::thread::spawn(move || loop {
    let _ = log_file.compact();
    std::thread::sleep(std::time::Duration::from_secs(PERIODIC_COMPACTION_INTERVAL));
  });

  let _ = handle.join();
  Ok(())
}
