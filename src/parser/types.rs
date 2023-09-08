pub type Literal = i32;

pub enum MathStmt {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Statement {
    Push(Literal),
    Math(MathStmt),
    Word { name: String, code: Vec<Statement> },
}
