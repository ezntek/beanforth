#[derive(Debug)]
pub struct ErrorLocation {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum Math {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Character {
    Output, // Period
    Equal,
    Gt,
    Lt,
    EndWord,   // Semicolon
    BeginWord, // Colon
}

#[derive(Debug)]
pub enum Token {
    Math(Math),
    Symbol(Character),
    Word(String),
    Literal(i32),
    Eof,
}
