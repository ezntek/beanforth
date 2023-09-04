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
pub enum Word {
    Dup,
    Drop,
    Swap,
    Over,
    Rot,
    Emit,
    Cr,

    // operations
    Mod,
    And,
    Or,
    Invert, // `Not`
}

#[derive(Debug)]
pub enum Symbol {
    Period,
    PeriodDoubleQuotes,
    Equal,
    Gt,
    Lt,
}

#[derive(Debug)]
pub enum Token {
    Math(Math),
    Symbol(Symbol),
    BuiltinWord(Word),
    Literal(i32),
    Eof,
}
