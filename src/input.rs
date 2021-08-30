use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::{error, fmt};

#[macro_export]
macro_rules! clear {
    () => {{
        print!("{}[2J", 27 as char);
        cursor!(0, 0)?
    }};
}

#[macro_export]
macro_rules! cursor {
    ($ln: expr, $col: expr) => {{
        use std::io::stdout;
        crossterm::execute!(stdout(), crossterm::cursor::MoveTo($ln, $col))
    }};
}

#[macro_export]
macro_rules! read {
    () => {{
        use std::io::{stdin, Read};
        let mut buf = String::new();
        let _ = stdin().read_line(&mut buf);
        buf
    }};
    ($e: expr) => {{
        use std::io::{stdin, Read};
        let mut buf: [u8; $e] = [0; $e];
        let _ = stdin().read_exact(&mut buf);
        buf
    }};
}

#[derive(Debug)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Number(u8),
    Letter(char),
    Escape,
    Enter,
    None,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Key::None => String::from("None"),
                Key::Enter => String::from("⬅"),
                Key::Escape => String::from("⤴"),
                Key::Up => String::from("∧"),
                Key::Down => String::from("∨"),
                Key::Left => String::from("<"),
                Key::Right => String::from(">"),
                Key::Letter(char) => char.to_string(),
                Key::Number(num) => num.to_string(),
            }
        )
    }
}

pub fn read_key() -> Result<Key, Box<dyn error::Error>> {
    enable_raw_mode()?;
    let buf: [u8; 1] = crate::read!(1);
    disable_raw_mode()?;
    match match buf[0] {
        digit @ 48..=58 => Key::Number(digit - 48),
        char @ 65..=90 => Key::Letter(char as char),
        char @ 97..=122 => Key::Letter(char as char),
        27 => Key::Escape,
        13 => Key::Enter,
        _ => Key::None,
    } {
        Key::Escape => read_arrow(),
        key @ _ => Ok(key),
    }
}

fn read_arrow() -> Result<Key, Box<dyn error::Error>> {
    enable_raw_mode()?;
    let buf: [u8; 2] = crate::read!(2);
    disable_raw_mode()?;
    Ok(match buf[1] {
        65 => Key::Up,
        66 => Key::Down,
        67 => Key::Right,
        68 => Key::Left,
        _ => Key::None,
    })
}
