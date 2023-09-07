#[macro_export]
macro_rules! eof {
    () => {
        Token::Eof
    };
}

#[macro_export]
macro_rules! nothing {
    () => {
        Token::Nothing
    };
}

#[macro_export]
macro_rules! err_loc {
    ($line:expr, $col:expr) => {
        types::ErrorLocation {
            line: $line,
            column: $col,
        }
    };
}

#[macro_export]
macro_rules! lex_err {
    ($line:expr, $col:expr, $variant:expr) => {
        error::LexerError {
            pos: err_loc!($line, $col),
            note: String::new(),
            variant: $variant,
        }
    };
}

#[macro_export]
macro_rules! note_lex_err {
    ($line:expr, $col:expr, $variant:expr, $note:expr) => {
        error::LexerError {
            pos: err_loc!($line, $col),
            note: $note.to_owned(),
            variant: $variant,
        }
    };
}
