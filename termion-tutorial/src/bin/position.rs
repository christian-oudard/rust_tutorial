use termion::raw::IntoRawMode;
use std::io::{Write, stdout};

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();


    // Draw Xs in all the corners and the center.
    let (width, height) = termion::terminal_size().unwrap();
    write!(stdout, "{}", termion::clear::All).unwrap();
    write!(stdout, "{}X", termion::cursor::Goto(1, 1)).unwrap();
    write!(stdout, "{}X", termion::cursor::Goto(width, 1)).unwrap();
    write!(stdout, "{}X", termion::cursor::Goto(1, height)).unwrap();
    write!(stdout, "{}X", termion::cursor::Goto(width, height)).unwrap();
    write!(stdout, "{}O", termion::cursor::Goto(width / 2, height / 2)).unwrap();
    print!("{}", termion::cursor::Hide);

    std::io::stdout().flush().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));

    write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
    print!("{}", termion::cursor::Show);
    print!("{}", termion::clear::All);
}
