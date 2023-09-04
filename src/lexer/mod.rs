mod types;

#[macro_use]
mod error;

#[macro_use]
mod macrodefs;

use error::*;
#[allow(unused_imports)]
use macrodefs::*;
use types::*;

#[derive(Debug)]
pub struct Lexer {
    data: Vec<char>,
    ptr: usize,
    peek: usize,
}

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

    fn analyze(&mut self) -> Result<Token, LexerError> {
        match char::from(self.at()) {
            // Maths Operators
            '+' => {
                self.peek = self.ptr - 1;
                if self.peek() != ' ' {
                    let invalid_ch = self.peek();
                    let variant = v_unexpected_tok!(invalid_ch);
                    self.peek = self.ptr;
                    return Err(note_lex_err!(
                        0,
                        self.ptr,
                        variant,
                        format!("`{}` not expected before `+`", invalid_ch)
                    ));
                }

                self.ptr += 1;
                let res = Token::Math(MathToken::Add);
                Ok(res)
            }
            '-' => {
                self.peek = self.ptr - 1;
                if self.peek() != ' ' {
                    let invalid_ch = self.peek();
                    let variant = v_unexpected_tok!(invalid_ch);
                    self.peek = self.ptr;
                    return Err(note_lex_err!(
                        0,
                        self.ptr,
                        variant,
                        format!("`{}` not expected before `-`", invalid_ch)
                    ));
                }

                self.ptr += 1;
                let res = Token::Math(MathToken::Sub);
                Ok(res)
            }
            '*' => {
                self.peek = self.ptr - 1;
                if self.peek() != ' ' {
                    let invalid_ch = self.peek();
                    let variant = v_unexpected_tok!(invalid_ch);
                    self.peek = self.ptr;
                    return Err(note_lex_err!(
                        0,
                        self.ptr,
                        variant,
                        format!("`{}` not expected before `*`", invalid_ch)
                    ));
                }

                self.ptr += 1;
                let res = Token::Math(MathToken::Mul);
                Ok(res)
            }
            '/' => {
                self.peek = self.ptr - 1;
                if self.peek() != ' ' {
                    let invalid_ch = self.peek();
                    let variant = v_unexpected_tok!(invalid_ch);
                    self.peek = self.ptr;
                    return Err(note_lex_err!(
                        0,
                        self.ptr,
                        variant,
                        format!("`{}` not expected before `/`", invalid_ch)
                    ));
                }

                self.ptr += 1;
                let res = Token::Math(MathToken::Div);
                Ok(res)
            }

            // Comments
            '\\' => {
                self.peek = self.ptr - 1;
                if self.peek() != ' ' {
                    let invalid_ch = self.peek();
                    let variant = v_unexpected_tok!(invalid_ch);
                    self.peek = self.ptr;
                    return Err(note_lex_err!(
                        0,
                        self.ptr,
                        variant,
                        format!("`{}` not expected before `\\`", invalid_ch)
                    ));
                }

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

                Ok(Token::Nothing)
            }

            // Constants
            ch => {
                let numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

                if numbers.contains(&ch) {
                    let mut curr_num_ch: char = self.at();
                    let mut potential_num = String::from(curr_num_ch);

                    self.peek = self.ptr + 1;
                    while numbers.contains(&curr_num_ch) {
                        curr_num_ch = self.peek();
                        if !numbers.contains(&curr_num_ch) {
                            break;
                        }
                        potential_num.push(curr_num_ch);
                        self.peek += 1;
                    }

                    self.ptr = self.peek;

                    return match potential_num.parse::<i32>() {
                        Ok(v) => Ok(Token::Literal(v)),
                        Err(_) => Err(lex_err!(0, self.ptr, v_deformed_literal!(potential_num))),
                    };
                }

                // ignore spaces
                if ch == ' ' {
                    self.ptr += 1;
                    return Ok(nothing!());
                }

                // Actual fallback case
                self.ptr += 1;
                Err(lex_err!(0, self.ptr, v_invalid_tok!(ch)))
            }
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut res: Vec<Token> = Vec::new();

        while self.ptr < self.data.len() {
            let tok = match self.analyze() {
                Ok(t) => t,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };

            res.push(tok);
        }

        res.push(eof!());
        res
    }
}
