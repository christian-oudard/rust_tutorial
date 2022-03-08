use termion::{color, style};

fn main() {
    println!("{}red", color::Fg(color::Red));
    println!("{}blue", color::Fg(color::Blue));
    println!("{}{}yellow and bold{}", color::Fg(color::Yellow), style::Bold, style::Reset);
    println!("{}italic{}", style::Italic, style::Reset);
    println!("{}underlined{}", style::Underline, style::Reset);
    // println!("{}Blink ew make it stop{}", style::Blink, style::Reset);
    // println!("{}faint, weird, just use a different color{}", style::Faint, style::Reset);
    // println!("{}invert, gross, just set the bg{}", style::Invert, style::Reset);
    // println!("{}framed not supported{}", style::Framed, style::Reset);
    // println!("{}strikethrough, not supported in tmux{}", style::CrossedOut, style::Reset);
}
