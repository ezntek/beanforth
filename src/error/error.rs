use std::{error, fmt};

use crate::lexer::types::Token;

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum ErrorVariant {
    UnexpectedChar(char),
    UnexpectedToken(Token),
    DeformedLiteral(String),
    //GenericPlaceholder,
}

#[derive(Debug)]
pub struct Error {
    pub pos: Location, // line, column
    pub note: String,
    pub variant: ErrorVariant,
}

impl fmt::Display for ErrorVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorVariant as V;

        match self {
            //V::InvalidToken(ch) => write!(f, "Invalid character `{}`", ch),
            V::UnexpectedChar(ch) => write!(f, "Unexpected character `{}`", ch),
            V::DeformedLiteral(s) => write!(f, "Deformed literal `{}`", s),
            V::UnexpectedToken(tok) => write!(f, "Unexpected Token `{:?}`", tok),
            //V::GenericPlaceholder => write!(f, "Generic placeholder error"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error at line {} column {}: {} ({})",
            self.pos.line, self.pos.column, self.variant, self.note
        )
    }
}

impl error::Error for Error {}
