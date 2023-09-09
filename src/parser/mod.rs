mod types;

use crate::lexer::types::Token;
use types::*;

pub struct Parser {
    tokens: Vec<Token>,
    ptr: usize,
    peek: usize,
}

impl Parser {
    pub fn new(data: Vec<Token>) -> Self {
        Parser {
            tokens: data,
            peek: 0,
            ptr: 0,
        }
    }

    pub fn get(&self, pos: usize) -> &Token {
        &self.tokens[pos]
    }

    pub fn at(&self) -> &Token {
        self.get(self.ptr)
    }

    pub fn peek(&self) -> &Token {
        self.get(self.ptr)
    }

    pub fn parse_token(&mut self, tok: &Token) -> Node {
        match tok {
            Token::Word(wd_s) => Node::WordCall(wd_s.clone()),
            Token::Math(math) => Node::Math(MathOp::from(math.clone())),
            _ => Node::NotImplemented,
        }
    }

    pub fn parse(&mut self) -> Node {
        let mut code: Vec<Node> = Vec::new();
        for tok in self.tokens.iter() {
            let node = self.parse_token(&tok);
            self.ptr += 1;
            code.push(node)
        }
        Node::Toplevel(code) // FIXME: should allow for partial parses
    }
}
