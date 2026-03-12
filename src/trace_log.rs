// trace_log.rs — Human-readable trace log for DiffPlayerQC (one file per run)

use chrono::{Datelike, Timelike};
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

static TRACE: Mutex<Option<File>> = Mutex::new(None);

/// Initialize the trace log. Creates a file named `yyyy_mm_dd_hh_mm_ss_Diff_start.log`
/// in the given directory (e.g. CARGO_MANIFEST_DIR or logs/). Call once at startup.
pub fn init(log_dir: &std::path::Path) -> std::io::Result<()> {
    let now = chrono::Local::now();
    let name = format!(
        "{:04}_{:02}_{:02}_{:02}_{:02}_{:02}_Diff_start.log",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
    let path = log_dir.join(name);
    let file = File::create(path)?;
    *TRACE.lock().unwrap() = Some(file);
    Ok(())
}

/// Write a line to the trace log: `[yyyy-mm-dd HH:MM:SS.mmm] msg`
pub fn log(msg: &str) {
    if let Ok(mut guard) = TRACE.lock() {
        if let Some(ref mut f) = *guard {
            let now = chrono::Local::now();
            let line = format!("[{}] {}\n", now.format("%Y-%m-%d %H:%M:%S%.3f"), msg);
            let _ = f.write_all(line.as_bytes());
            let _ = f.flush();
        }
    }
}
