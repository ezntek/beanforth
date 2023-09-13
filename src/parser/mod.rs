mod types;

use crate::error::*;
use std::rc::Rc;

use crate::{
    error::{err_with_note, loc, v_unexpected_char},
    lexer::types::{Character, Token, TokenVariant},
};
use types::*;

// Rust Function Pointers
//
// Owned Trait Object:  Box<dyn Fn(char, i32, String)>
// Generic:             T where T: Fn(char, i32, String)
// Shorthand Generic:   impl Fn(char, i32, String)

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

    pub fn parse_word(&mut self) -> ParserResult<Node> {
        self.peek = self.ptr + 1;
        let word_tok = self.peek();
        let word_name = match &word_tok.variant {
            TokenVariant::Word(wd) => wd.clone(),
            _ => {
                return Err(err_with_note!(
                    loc!(0, 0),
                    v_unexpected_tok!(word_tok.clone()),
                    "aorysudhoyauwnfdkoyawumt"
                ))
            }
        };
        self.peek += 1;

        let mut code: Vec<Node> = Vec::new();
        let tokens = self.tokens.clone();
        let tok = &tokens[self.peek];

        let invalid_word_chrs = [TokenVariant::Symbol(Character::BeginWord)];

        while tok.variant != TokenVariant::Symbol(Character::EndWord)
            && self.peek < self.tokens.len()
        {
            let tok = &tokens[self.peek].clone();
            if invalid_word_chrs.contains(&tok.variant) {
                return Err(err_with_note!(
                    loc!(0, 0),
                    v_unexpected_tok!(tok.clone()),
                    format!("{} not expected within a word", &tok)
                ));
            } else {
                let node = self.parse_token(&tok.variant).unwrap();
            }
            self.peek += 1;
        }

        Ok(Node::WordDef {
            name: word_name,
            code,
        })
    }

    pub fn parse_token(&mut self, tok: &TokenVariant) -> ParserResult<Node> {
        match tok {
            // Basics
            TokenVariant::Word(wd_s) => Ok(Node::WordCall(wd_s.clone())),
            TokenVariant::Math(math) => Ok(Node::Math(MathOp::from(math.clone()))),
            TokenVariant::Literal(n) => Ok(Node::Push(n.clone())),

            // Word definitions
            TokenVariant::Symbol(Character::BeginWord) => self.parse_word(),
            _ => Ok(Node::NotImplemented),
        }
    }

    pub(super) fn set_data(&mut self, data: impl Into<Rc<[Token]>>) {
        self.tokens = data.into();
    }

    pub fn parse(&mut self) -> Node {
        let mut code: Vec<Node> = Vec::new();

        let is_whole_file = self.get(0).variant == TokenVariant::Begin
            && self.get(self.tokens.len() - 1).variant == TokenVariant::End;

        let tokens = self.tokens.clone();
        if !is_whole_file {
            return self.parse_token(&tokens[0].variant).unwrap_or_else(|e| {
                println!("{}", e);
                std::process::exit(1)
            });
        }

        while self.ptr < tokens.len() {
            let node = self
                .parse_token(&tokens[self.ptr].variant)
                .unwrap_or_else(|e| {
                    println!("{}", e);
                    std::process::exit(1)
                });
            self.ptr += 1;
            code.push(node);
        }

        Node::Toplevel(code)
    }
}
