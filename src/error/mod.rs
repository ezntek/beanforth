#[macro_use]
mod macros;
mod error;

pub use error::*;

#[allow(unused_imports)]
pub use macros::*;

/*
macro_rules! v_invalid_tok {
    ($param:expr) => {
        ErrorVariant::InvalidToken($param)
    };
}
        */

macro_rules! v_deformed_literal {
    ($param:expr) => {
        ErrorVariant::DeformedLiteral($param)
    };
}

macro_rules! v_unexpected_char {
    ($param:expr) => {
        ErrorVariant::UnexpectedChar($param)
    };
}

macro_rules! v_unexpected_tok {
    ($param:expr) => {
        ErrorVariant::UnexpectedToken($param)
    };
}

pub(crate) use v_deformed_literal;
// pub(crate) use v_invalid_tok;
pub(crate) use v_unexpected_char;
pub(crate) use v_unexpected_tok;

/*#[macro_export]
macro_rules! v_generic {
    () => {
        error::LexerErrorVariant::GenericPlaceholder
    };
}*/
