#![allow(dead_code)]
//! this Module intends to colorize the output of ongoing process

/// enum Col
pub enum Col {
    Green,
    Red,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

/// the implementation of Col enum
impl Col {
    /// # print_col:
    /// method that print text within the console with specific color.
    /// The method can be written in two ways so far see the example
    /// ## Arguments
    /// * `str` - text to be printed
    /// * `self` - enum Col
    /// ## Returns
    /// String - colorized text
    /// ## Example:
    /// ```rust
    /// use commandcrafter::color::Col;
    /// let text1 = Col::print_col(&Col::Magenta, "hello");
    /// let text2 = Col::Green.print_col("hello");
    /// assert_eq!(text1, "\x1b[35mhello\x1b[0m");
    /// assert_eq!(text2, "\x1b[32mhello\x1b[0m");
    /// ```
    pub fn print_col(&self, text: &str) -> String {
        match self {
            Col::Green => format!("\x1b[32m{}\x1b[0m", text),
            Col::Red => format!("\x1b[31m{}\x1b[0m", text),
            Col::Yellow => format!("\x1b[33m{}\x1b[0m", text),
            Col::Blue => format!("\x1b[34m{}\x1b[0m", text),
            Col::Magenta => format!("\x1b[35m{}\x1b[0m", text),
            Col::Cyan => format!("\x1b[36m{}\x1b[0m", text),
            Col::White => format!("\x1b[37m{}\x1b[0m", text),
        }
    }
}
