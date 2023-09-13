use crate::error::{loc, Error, Location};

pub(super) type LexerResult<T> = Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Math {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Character {
    Output, // Period
    Equal,
    Gt,
    Lt,
    EndWord,   // Semicolon
    BeginWord, // Colon
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReservedWord {
    If,
    Then,
    Else,
    Do,
    Loop,
}

impl ReservedWord {
    pub fn try_to_string<S: AsRef<str>>(s: S) -> Option<Self> {
        use ReservedWord as R;
        match s.as_ref() {
            "if" => Some(R::If),
            "then" => Some(R::Then),
            "else" => Some(R::Else),
            "do" => Some(R::Do),
            "loop" => Some(R::Loop),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenVariant {
    Math(Math),
    Symbol(Character),
    Word(String),
    ReservedWord(ReservedWord),
    Literal(i32),
    Begin,
    End,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub variant: TokenVariant,
    location: Option<Location>,
}

impl Token {
    pub fn new(variant: TokenVariant) -> Self {
        Token {
            variant,
            location: None,
        }
    }

    pub fn with_location(variant: TokenVariant, location: Location) -> Self {
        Token {
            variant,
            location: Some(location),
        }
    }

    pub fn get_location(&self) -> Location {
        match self.location {
            Some(loc) => loc,
            None => loc!(0, 0),
        }
    }
}
