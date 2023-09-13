#[allow(unused_imports)]
use super::error::*;

macro_rules! err_loc {
    ($line:expr, $col:expr) => {
        ErrorLocation {
            line: $line,
            column: $col,
        }
    };
}
macro_rules! err {
    ($line:expr, $col:expr, $variant:expr) => {
        Error {
            pos: err_loc!($line, $col),
            note: String::new(),
            variant: $variant,
        }
    };
}

macro_rules! err_with_note {
    ($line:expr, $col:expr, $variant:expr, $note:expr) => {
        Error {
            pos: err_loc!($line, $col),
            note: $note.to_owned(),
            variant: $variant,
        }
    };
}

pub(crate) use err;
pub(crate) use err_loc;
pub(crate) use err_with_note;
