use crate::lexer::types::Math;

pub type Literal = i32;

#[derive(Debug)]
pub enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Node {
    Push(Literal),
    Math(MathOp),
    WordCall(String),
    WordDef {
        name: String,
        code: Vec<Node>,
    },
    Loop(Vec<Node>),
    Conditional {
        if_br: Vec<Node>,
        else_br: Vec<Node>,
    },
    Toplevel(Vec<Node>),
    NotImplemented, // TODO: get rid of it asap
}

impl From<Math> for MathOp {
    fn from(value: Math) -> Self {
        use Math as M;
        match value {
            M::Add => MathOp::Add,
            M::Sub => MathOp::Sub,
            M::Mul => MathOp::Mul,
            M::Div => MathOp::Div,
        }
    }
}
