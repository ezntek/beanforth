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
    pub fn new(data: impl Into<Rc<[Token]>>) -> Self {
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
        self.get(self.peek)
    }

    pub fn offset_peek(&self, offset: usize) -> &Token {
        self.get(self.peek + offset)
    }

    pub fn parse_word(&mut self) -> Node {
        self.peek = self.ptr + 1;
        let word_name = self.peek();
        self.peek += 1;

        let mut code: Vec<Node> = Vec::new();
        let tok = self.peek();

        let invalid_word_chrs = [
            &Token::Symbol(Character::BeginWord),
            &Token::Symbol(Character::EndWord),
        ];

        while tok != &Token::Symbol(Character::EndWord) {
            let tok = self.peek();
            if invalid_word_chrs.contains(&tok) {
                unreachable!() // BUG: is reachable
            } else {
                unreachable!() // BUG: is reachable
            }
        }

        Node::NotImplemented // FIXME:
    }

    pub fn parse_token(&mut self, tok: &Token) -> Node {
        match tok {
            // Basics
            Token::Word(wd_s) => Node::WordCall(wd_s.clone()),
            Token::Math(math) => Node::Math(MathOp::from(math.clone())),
            Token::Literal(n) => Node::Push(*n),

            // Word definitions
            Token::Symbol(Character::BeginWord) => self.parse_word(),
            _ => Node::NotImplemented,
        }
    }

    pub(super) fn set_data(&mut self, data: impl Into<Rc<[Token]>>) {
        self.tokens = data.into();
    }

    pub fn parse(&mut self) -> Node {
        let mut code: Vec<Node> = Vec::new();

        let is_whole_file =
            self.get(0) == &Token::Begin && self.get(self.tokens.len() - 1) == &Token::End;

        let tokens = self.tokens.clone();
        if !is_whole_file {
            return self.parse_token(&tokens[0]);
        }

        while self.ptr < tokens.len() {
            let node = self.parse_token(&tokens[self.ptr]);
            self.ptr += 1;
            code.push(node);
        }

        Node::Toplevel(code)
    }
}
