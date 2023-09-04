mod types;

#[macro_use]
mod error;

#[macro_use]
mod macrodefs;

use error::*;
#[allow(unused_imports)]
use macrodefs::*;
use types::*;

const NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

#[derive(Debug)]
pub struct Lexer {
    data: Vec<char>,
    ptr: usize,
    peek: usize,
}

type LexerResult = Result<Token, LexerError>;

impl Lexer {
    pub fn new(data: String) -> Self {
        Lexer {
            data: data.chars().collect::<Vec<char>>(),
            ptr: 0,
            peek: 0,
        }
    }

    fn at(&self) -> char {
        self.data[self.ptr]
    }

    fn peek(&self) -> char {
        self.data[self.peek]
    }

    fn tokenize_number(&mut self) -> LexerResult {
        let mut curr_num_ch: char = self.at();
        let mut potential_num = String::from(curr_num_ch);

        self.peek = self.ptr + 1;
        while NUMBERS.contains(&curr_num_ch) {
            curr_num_ch = self.peek();
            if !NUMBERS.contains(&curr_num_ch) {
                break;
            }
            potential_num.push(curr_num_ch);
            self.peek += 1;
        }

        self.ptr = self.peek;

        let r = match potential_num.parse::<i32>() {
            Ok(v) => Ok(Token::Literal(v)),
            Err(_) => Err(lex_err!(0, self.ptr, v_deformed_literal!(potential_num))),
        };

        return r;
    }

    fn tok_single_chr(&mut self, ch: char) -> Option<LexerResult> {
        match ch {
            '+' => {
                self.ptr += 1;
                let res = Token::Math(Math::Add);
                Some(Ok(res))
            }
            '-' => {
                self.ptr += 1;
                let res = Token::Math(Math::Sub);
                Some(Ok(res))
            }
            '*' => {
                self.ptr += 1;
                let res = Token::Math(Math::Mul);
                Some(Ok(res))
            }
            '/' => {
                self.ptr += 1;
                let res = Token::Math(Math::Div);
                Some(Ok(res))
            }
            _ => None,
        }
    }

    fn tokenize_at_ptr(&mut self) -> Option<LexerResult> {
        let ch = self.at();

        // Ignore whitespaces
        if ['\t', ' '].contains(&ch) {
            self.ptr += 1;
            return None;
        }

        // Numbers
        if NUMBERS.contains(&ch) {
            return Some(self.tokenize_number());
        }

        // comments
        if ch == '\\' {
            let mut curr_chr = '\\';

            self.peek = self.ptr + 1;
            while curr_chr != '\n' {
                curr_chr = self.peek();
                self.peek += 1;

                if self.peek >= self.data.len() {
                    break;
                }
            }

            self.ptr = self.peek;
        }

        // Single character symbols
        if let Some(r) = self.tok_single_chr(ch) {
            return Some(r);
        }

        // Fallback
        Some(Err(lex_err!(0, self.ptr, v_invalid_tok!(ch))))
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut res: Vec<Token> = Vec::new();

        while self.ptr < self.data.len() {
            if let Some(r) = self.tokenize_at_ptr() {
                let tok = match r {
                    Ok(t) => t,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                };

                res.push(tok);
            }
        }

        res.push(eof!());
        res
    }
}
