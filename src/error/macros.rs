#[allow(unused_imports)]
use super::error::*;

macro_rules! loc {
    ($line:expr, $col:expr) => {
        Location {
            line: $line,
            column: $col,
        }
    };
}
macro_rules! err {
    ($pos:expr, $variant:expr) => {
        Error {
            pos: $pos,
            note: String::new(),
            variant: $variant,
        }
    };
}

macro_rules! err_with_note {
    ($pos:expr, $variant:expr, $note:expr) => {
        Error {
            pos: $pos,
            note: $note.to_owned(),
            variant: $variant,
        }
    };
}

pub(crate) use err;
pub(crate) use err_with_note;
pub(crate) use loc;
