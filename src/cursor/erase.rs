use crate::cursor::{CSI, ESC};
use std::fmt::Display;

#[allow(dead_code)]
pub enum Erase {
    ToScreenEnd,
    ToScreenBegin,
    EntireScreen,
    ToLineEnd,
    ToLineBegin,
    EntireLine,
}

impl Display for Erase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ToScreenEnd => write!(f, "{}{}0J", ESC, CSI),
            Self::ToScreenBegin => write!(f, "{}{}1J", ESC, CSI),
            Self::EntireScreen => write!(f, "{}{}2J", ESC, CSI),
            Self::ToLineEnd => write!(f, "{}{}0K", ESC, CSI),
            Self::ToLineBegin => write!(f, "{}{}1K", ESC, CSI),
            Self::EntireLine => write!(f, "{}{}2K", ESC, CSI),
        }
    }
}
