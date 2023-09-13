macro_rules! token {
    ($variant:expr) => {
        Token::new($variant)
    };

    ($variant:expr, $loc:expr) => {
        Token::with_location($variant, $loc)
    };

    ($variant:expr, $line:expr, $col:expr) => {
        Token::with_location($variant, location!($line, $col))
    };
}

macro_rules! eof {
    () => {
        token!(TokenVariant::End)
    };
}

pub(super) use eof;
pub(crate) use token;
