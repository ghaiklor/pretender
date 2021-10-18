use self::{color::Color, position::Position, style::Style};
use std::fmt::Display;

pub mod color;
pub mod erase;
pub mod position;
pub mod style;

pub const ESC: char = '\x1B';
pub const CSI: char = '[';

#[derive(PartialEq)]
pub struct Cursor {
    string: String,
}

impl Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl From<&str> for Cursor {
    fn from(string: &str) -> Self {
        Cursor {
            string: string.to_string(),
        }
    }
}

impl From<String> for Cursor {
    fn from(string: String) -> Self {
        Cursor { string }
    }
}

impl Cursor {
    fn prefix(mut self, prefix: impl Display) -> Self {
        self.string = format!("{}{}", prefix, self.string);
        self
    }

    fn suffix(mut self, suffix: impl Display) -> Self {
        self.string = format!("{}{}", self.string, suffix);
        self
    }

    pub fn wrap(self, prefix: impl Display, suffix: impl Display) -> Self {
        self.prefix(prefix).suffix(suffix)
    }

    pub fn with_style(self, style: Style) -> Self {
        self.wrap(style, Style::Normal)
    }

    pub fn with_color(self, color: Color) -> Self {
        self.with_style(Style::ForegroundColor(color))
    }

    pub fn with_background(self, color: Color) -> Self {
        self.with_style(Style::BackgroundColor(color))
    }

    pub fn at_previous_line(self) -> Self {
        self.prefix(Position::PreviousLine(1))
    }

    pub fn at_next_line(self) -> Self {
        self.prefix(Position::NextLine(1))
    }

    pub fn at_absolute_position(self, row: usize, column: usize) -> Self {
        self.prefix(Position::Absolute(row, column))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_string_with_style() {
        let cursor = Cursor::from("Hello").with_style(Style::Bold);
        assert_eq!(cursor.string, String::from("\x1B[1mHello\x1B[0m"));
    }

    #[test]
    fn it_creates_colorized_string() {
        let cursor = Cursor::from("Hello").with_color(Color::Green);
        assert_eq!(cursor.string, String::from("\x1B[32mHello\x1B[0m"));
    }

    #[test]
    fn it_creates_string_with_background() {
        let cursor = Cursor::from("Hello").with_background(Color::Magenta);
        assert_eq!(cursor.string, String::from("\x1B[45mHello\x1B[0m"));
    }

    #[test]
    fn it_creates_string_at_previous_line() {
        let cursor = Cursor::from("Hello").at_previous_line();
        assert_eq!(cursor.string, String::from("\x1B[1FHello"));
    }

    #[test]
    fn it_creates_string_at_next_line() {
        let cursor = Cursor::from("Hello").at_next_line();
        assert_eq!(cursor.string, String::from("\x1B[1EHello"));
    }

    #[test]
    fn it_creates_string_at_absolute_position() {
        let cursor = Cursor::from("Hello").at_absolute_position(10, 25);
        assert_eq!(cursor.string, String::from("\x1B[10;25HHello"))
    }

    #[test]
    fn it_allows_chainable_calls() {
        let cursor = Cursor::from("Hello")
            .with_color(Color::Green)
            .with_background(Color::White)
            .at_previous_line()
            .at_previous_line();

        assert_eq!(
            cursor.string,
            String::from("\x1B[1F\x1B[1F\x1B[47m\x1B[32mHello\x1B[0m\x1B[0m")
        )
    }
}
