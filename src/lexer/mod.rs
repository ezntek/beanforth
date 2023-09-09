pub mod types;

#[macro_use]
mod error;

#[macro_use]
mod macrodefs;

use std::num::IntErrorKind;

use error::*;
#[allow(unused_imports)]
use macrodefs::*;
use types::*;

#[derive(Debug)]
pub struct Lexer {
    data: Vec<char>,
    data_len: usize,
    ptr: usize,
    peek: usize,
}

type LexerResult<T> = Result<T, LexerError>;

macro_rules! unwrap_to_eof {
    ($optchar:expr) => {
        match $optchar {
            Some(ch) => ch,
            None => return Ok(Token::Eof),
        }
    };
}

macro_rules! unwrap_to_eof_option {
    ($optchar:expr) => {
        match $optchar {
            Some(ch) => ch,
            None => return Some(Ok(Token::Eof)),
        }
    };
}

impl Lexer {
    pub fn new(data: String) -> Self {
        dbg!(&data);
        dbg!(data.len());
        Lexer {
            data: data.chars().collect::<Vec<char>>(),
            data_len: data.len(),
            ptr: 0,
            peek: 0,
        }
    }

    fn get(&self, pos: usize) -> Option<char> {
        if pos >= self.data_len {
            None
        } else {
            Some(self.data[pos])
        }
    }

    fn at(&self) -> Option<char> {
        dbg!(self.ptr);
        self.get(self.ptr)
    }

    fn peek(&self) -> Option<char> {
        dbg!(self.peek);
        self.get(self.peek)
    }

    fn tokenize_number(&mut self) -> LexerResult<Token> {
        let mut curr_num_ch: char = unwrap_to_eof!(self.at());
        let mut potential_num = String::from(curr_num_ch);

        self.peek = self.ptr + 1;
        'outerloop: while curr_num_ch.is_ascii_digit() {
            curr_num_ch = match self.peek() {
                Some(n) => n,
                None => {
                    self.peek -= 1;
                    potential_num.push(self.peek().unwrap());
                    self.peek += 1;
                    break 'outerloop;
                }
            };
            if !curr_num_ch.is_ascii_digit() {
                break;
            }
            potential_num.push(curr_num_ch);
            self.peek += 1;
        }

        self.ptr = self.peek;

        match potential_num.parse::<i32>() {
            Ok(v) => Ok(Token::Literal(v)),
            Err(e) => match e.kind() {
                IntErrorKind::PosOverflow => Err(note_lex_err!(
                    0,
                    self.ptr,
                    v_deformed_literal!(potential_num),
                    "Number is too big to be a signed 32bit integer!"
                )),
                IntErrorKind::NegOverflow => Err(note_lex_err!(
                    0,
                    self.ptr,
                    v_deformed_literal!(potential_num),
                    "Number is too small to be a signed 32bit integer!"
                )),
                _ => Err(lex_err!(0, self.ptr, v_deformed_literal!(potential_num))),
            },
        }
    }

    fn read_word(&mut self) -> Option<String> {
        let mut s = String::new();

        self.peek = self.ptr;

        'outerloop: while {
            let p = self.peek();
            dbg!(&p);
            let peek = match p {
                Some(pk) => pk,
                None => break 'outerloop,
            };
            !peek.is_ascii_whitespace()
        } {
            let peek = match self.peek() {
                Some(p) => p,
                None => {
                    s.push(self.get(self.peek - 1).unwrap());
                    break 'outerloop;
                }
            };

            s.push(peek);
            self.peek += 1;
        }

        self.ptr = self.peek;

        println!("{}", s);

        dbg!(&s);

        return Some(s);
    }

    fn tokenize_word(&mut self) -> Option<LexerResult<Token>> {
        let word_string = unwrap_to_eof_option!(self.read_word());
        dbg!(&word_string);
        Some(Ok(Token::Word(word_string))) // TODO: try to make it better
    }

    fn tokenize_at_ptr(&mut self) -> Option<LexerResult<Token>> {
        let ch = unwrap_to_eof_option!(self.at());
        dbg!(&ch);

        // Ignore whitespaces
        if ch.is_ascii_whitespace() {
            self.ptr += 1;
            return None;
        }

        // comments
        if ch == '\\' {
            self.peek = self.ptr + 1;
            while unwrap_to_eof_option!(self.peek()) != '\n' {
                self.peek += 1;

                if self.peek >= self.data_len {
                    break;
                }
            }

            self.ptr = self.peek;
            return None;
        }

        // multiline comments
        if ch == '(' {
            self.peek = self.ptr + 1;
            while unwrap_to_eof_option!(self.peek()) != ')' {
                self.peek += 1;

                if self.peek >= self.data_len {
                    break;
                }
            }

            self.ptr = self.peek + 1;
            return None;
        }

        // Numbers
        if ch.is_ascii_digit() {
            println!("num");
            return Some(self.tokenize_number());
        }

        // Words
        if let Some(w) = self.tokenize_word() {
            return Some(w);
        }

        // Fallback
        Some(Err(lex_err!(0, self.ptr, v_invalid_tok!(ch))))
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut res: Vec<Token> = Vec::new();

        while self.ptr < self.data_len {
            if let Some(r) = self.tokenize_at_ptr() {
                let tok = match r {
                    Ok(t) => t,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                };

                if let Token::Eof = tok {
                    break;
                }

                res.push(tok);
            }
        }

        res.push(eof!());
        res
    }
}
