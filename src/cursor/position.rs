use crate::cursor::{CSI, ESC};
use std::fmt::Display;

#[allow(dead_code)]
pub enum Position {
    Up(usize),
    Down(usize),
    Forward(usize),
    Backward(usize),
    NextLine(usize),
    PreviousLine(usize),
    Absolute(usize, usize),
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up(cells) => write!(f, "{}{}{}A", ESC, CSI, cells),
            Self::Down(cells) => write!(f, "{}{}{}B", ESC, CSI, cells),
            Self::Forward(cells) => write!(f, "{}{}{}C", ESC, CSI, cells),
            Self::Backward(cells) => write!(f, "{}{}{}D", ESC, CSI, cells),
            Self::NextLine(lines) => write!(f, "{}{}{}E", ESC, CSI, lines),
            Self::PreviousLine(lines) => write!(f, "{}{}{}F", ESC, CSI, lines),
            Self::Absolute(row, columns) => write!(f, "{}{}{};{}H", ESC, CSI, row, columns),
        }
    }
}
