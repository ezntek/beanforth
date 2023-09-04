#[derive(Debug)]
pub struct ErrorLocation {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum MathToken {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Token {
    Math(MathToken),
    Literal(i32),
    Invalid,
    Nothing,
    Eof,
}
