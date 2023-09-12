mod types;

use std::rc::Rc;

use crate::lexer::types::{Character, Token};
use types::*;

pub struct Parser {
    tokens: Rc<[Token]>,
    ptr: usize,
    peek: usize,
}

impl Parser {
    pub fn new(data: Vec<Token>) -> Self {
        let data: Rc<[Token]> = data.into();
        Parser {
            tokens: data,
            peek: 0,
            ptr: 0,
        }
    }

    pub fn get(&self, pos: usize) -> &Token {
        &self.tokens[pos]
    }

    pub fn peek(&self) -> &Token {
        self.get(self.ptr)
    }

    pub fn at(&self) -> &Token {
        self.get(self.peek)
    }

    pub fn parse_word(&self) -> &Token {}

    pub fn parse_token(&mut self, tok: &Token) -> Node {
        match tok {
            // Basics
            Token::Word(wd_s) => Node::WordCall(wd_s.clone()),
            Token::Math(math) => Node::Math(MathOp::from(math.clone())),
            Token::Literal(n) => Node::Push(n),

            // Word definitions
            Token::Symbol(Character::BeginWord) => self.parse_word(),
            _ => Node::NotImplemented,
        }
    }

    pub fn parse(&mut self) -> Node {
        let mut code: Vec<Node> = Vec::new();
        let tokens = self.tokens.clone();
        while self.ptr < self.tokens.len() {
            let node = self.parse_token(self.at());
            self.ptr += 1;
            code.push(node)
        }
        Node::Toplevel(code) // FIXME: should allow for partial parses
    }
}
