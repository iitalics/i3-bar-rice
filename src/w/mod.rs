use std;
use render;
use super::{Result, Error};

mod date;
mod battery;
mod memory;

pub use self::date::DateTimeW;
pub use self::battery::BatteryW;
pub use self::memory::MemoryW;

// trait for different components of the status bar
pub trait Widget {
    fn cooldown(&self) -> i32 { 1 }
    fn blit(&self) -> Result<Vec<render::Elem>>;
}


// --- utilities for widgets ---

fn read_program<S>(prgm: S) -> Result<String>
    where S: AsRef<std::ffi::OsStr> {

    std::process::Command::new(prgm)
        .output()
        .map_err(|_| Error::CmdFailed)
        .and_then(|out| {
            String::from_utf8(out.stdout)
                .map_err(|_| Error::Utf8Error)
        })
}
