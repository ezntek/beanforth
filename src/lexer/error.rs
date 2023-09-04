use super::types::ErrorLocation;
use std::{error, fmt};

#[derive(Debug)]
pub enum LexerErrorVariant {
    InvalidToken(char),
    UnexpectedToken(char),
    DeformedLiteral(String),
}

#[derive(Debug)]
pub struct LexerError {
    pub pos: ErrorLocation, // line, column
    pub note: String,
    pub variant: LexerErrorVariant,
}

impl fmt::Display for LexerErrorVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LexerErrorVariant as V;

        match self {
            V::InvalidToken(ch) => write!(f, "Invalid character `{}`", ch),
            V::UnexpectedToken(ch) => write!(f, "Unexpected character `{}`", ch),
            V::DeformedLiteral(s) => write!(f, "Deformed literal `{}`", s),
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error at line {} column {}: {} ({})",
            self.pos.line, self.pos.column, self.variant, self.note
        )
    }
}

impl error::Error for LexerError {}

#[macro_export]
macro_rules! v_invalid_tok {
    ($param:expr) => {
        error::LexerErrorVariant::InvalidToken($param)
    };
}

#[macro_export]
macro_rules! v_unexpected_tok {
    ($param:expr) => {
        error::LexerErrorVariant::UnexpectedToken($param)
    };
}

#[macro_export]
macro_rules! v_deformed_literal {
    ($param:expr) => {
        error::LexerErrorVariant::DeformedLiteral($param)
    };
}
