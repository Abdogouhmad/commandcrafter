#![allow(dead_code)]
//! this Module intends to colorize the output of ongoing process

/// enum fmt
pub enum Fmt {
    Green,
    Red,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Fmt {
    pub fn coprint(&self, text: &str) -> String {
        match self {
            Fmt::Green => format!("\x1b[32m{}\x1b[0m", text),
            Fmt::Red => format!("\x1b[31m{}\x1b[0m", text),
            Fmt::Yellow => format!("\x1b[33m{}\x1b[0m", text),
            Fmt::Blue => format!("\x1b[34m{}\x1b[0m", text),
            Fmt::Magenta => format!("\x1b[35m{}\x1b[0m", text),
            Fmt::Cyan => format!("\x1b[36m{}\x1b[0m", text),
            Fmt::White => format!("\x1b[37m{}\x1b[0m", text),
        }
    }
}
