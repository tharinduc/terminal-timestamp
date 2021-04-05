extern crate termion;
extern crate chrono;


use std::time::SystemTime;
use std::io::stdout;
use chrono::{DateTime, Utc};
use termion::{color, terminal_size, cursor, raw::IntoRawMode, cursor::DetectCursorPos};

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let cursor_pos = stdout.cursor_pos().expect("Could not get the cursor position");
    let time_now = SystemTime::now();
    let datetime = DateTime::<Utc>::from(time_now);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let size = terminal_size().expect("Could not get the terminal size");
    let x = size.0 - (timestamp_str.len() - 1) as u16;
    let y = cursor_pos.1 - 1;
    print!("{}{red}{}{reset}",
        cursor::Goto(x, y),
        timestamp_str,
        red = color::Fg(color::Red),
        reset = color::Fg(color::Reset));
}
