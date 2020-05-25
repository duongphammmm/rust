extern crate termion;

use termion::event::*;
use termion::input::{TermRead, MouseTerminal};
use termion::{color, cursor, clear, style};
use termion::raw::IntoRawMode;
use std::io::{self, Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    writeln!(stdout, "{}{}q to exit. Type stuff, use alt, click around and so on...",
        termion::clear::All,
        termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Char(c)) => print!("{}", c),
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, a, b) |
                    MouseEvent::Release(a, b) |
                    MouseEvent::Hold(a, b) => {
                        write!(stdout, "{}", cursor::Goto(a,b)).unwrap();
                    }
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}