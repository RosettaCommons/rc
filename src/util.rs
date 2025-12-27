#![allow(unused_imports)]

mod command;
mod dir_guard;
mod yansi;

use std::io::{self, Write};
use std::time::Duration;

pub use command::Command;
pub use dir_guard::ensure_dir_signature;
pub use yansi::{Paint, PaintExt};

#[allow(dead_code)]
/// Fancy sleep function with a countdown message.
/// Prints: "{message}... sleeping... {time_left}"
pub fn sleep(message: impl AsRef<str>, seconds: usize) {
    let mut max_len = 0;

    for i in (1..=seconds).rev() {
        let msg = format!("{}sleeping... {i}", message.as_ref());
        print!("{msg:max_len$}\r");
        io::stdout().flush().unwrap();
        max_len = max_len.max(msg.len());
        std::thread::sleep(Duration::from_secs(1));
    }

    // Erase the message line after sleeping
    print!("\r{:width$}\r", "", width = max_len);
    io::stdout().flush().unwrap();
}
