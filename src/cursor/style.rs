use super::color::Color;
use crate::cursor::{CSI, ESC};
use std::fmt::Display;

#[allow(dead_code)]
pub enum Style {
    Normal,
    Bold,
    Dim,
    Italic,
    Underline,
    SlowBlink,
    RapidBlink,
    Invert,
    CrossedOut,
    ForegroundColor(Color),
    BackgroundColor(Color),
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "{}{}0m", ESC, CSI),
            Self::Bold => write!(f, "{}{}1m", ESC, CSI),
            Self::Dim => write!(f, "{}{}2m", ESC, CSI),
            Self::Italic => write!(f, "{}{}3m", ESC, CSI),
            Self::Underline => write!(f, "{}{}4m", ESC, CSI),
            Self::SlowBlink => write!(f, "{}{}5m", ESC, CSI),
            Self::RapidBlink => write!(f, "{}{}6m", ESC, CSI),
            Self::Invert => write!(f, "{}{}7m", ESC, CSI),
            Self::CrossedOut => write!(f, "{}{}9m", ESC, CSI),
            Self::ForegroundColor(color) => match color {
                Color::Black => write!(f, "{}{}30m", ESC, CSI),
                Color::Red => write!(f, "{}{}31m", ESC, CSI),
                Color::Green => write!(f, "{}{}32m", ESC, CSI),
                Color::Yellow => write!(f, "{}{}33m", ESC, CSI),
                Color::Blue => write!(f, "{}{}34m", ESC, CSI),
                Color::Magenta => write!(f, "{}{}35m", ESC, CSI),
                Color::Cyan => write!(f, "{}{}36m", ESC, CSI),
                Color::White => write!(f, "{}{}37m", ESC, CSI),
            },
            Self::BackgroundColor(color) => match color {
                Color::Black => write!(f, "{}{}40m", ESC, CSI),
                Color::Red => write!(f, "{}{}41m", ESC, CSI),
                Color::Green => write!(f, "{}{}42m", ESC, CSI),
                Color::Yellow => write!(f, "{}{}43m", ESC, CSI),
                Color::Blue => write!(f, "{}{}44m", ESC, CSI),
                Color::Magenta => write!(f, "{}{}45m", ESC, CSI),
                Color::Cyan => write!(f, "{}{}46m", ESC, CSI),
                Color::White => write!(f, "{}{}47m", ESC, CSI),
            },
        }
    }
}
