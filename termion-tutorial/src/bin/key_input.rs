#[macro_use]
extern crate log;

use std::cmp::min;

use simplelog::*;
use std::fs::File;

use std::io::{stdin, stdout, Write};
use termion::{
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
};

fn main() {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("log").unwrap(),
    ).unwrap();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let (width, height) = termion::terminal_size().unwrap();

    let mut x: u16 = 0;
    let mut y: u16 = 0;

    write!(stdout, "{}", termion::clear::All).unwrap();
    print!("{}", termion::cursor::Hide);
    write!(stdout, "{}", termion::clear::All).unwrap();
    write!(stdout, "{}X", termion::cursor::Goto(1, 1)).unwrap();
    std::io::stdout().flush().unwrap();

    for c in stdin.keys() {
        let evt = c.unwrap();
        match evt {
            Key::Char('q') => break,
            Key::Up => y = y.saturating_sub(1),
            Key::Down => y = min(height - 1, y + 1),
            Key::Left => x = x.saturating_sub(1),
            Key::Right => x = min(width - 1, x + 1),
            _ => {},
        }
        write!(stdout, "{}", termion::clear::All).unwrap();
        write!(stdout, "{}X", termion::cursor::Goto(x+1, y+1)).unwrap();
        std::io::stdout().flush().unwrap();
    }

    print!("{}", termion::cursor::Show);
    print!("{}", termion::clear::All);
}
