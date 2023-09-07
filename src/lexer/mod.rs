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
            Some(self.data[self.ptr])
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
            Err(_) => Err(lex_err!(0, self.ptr, v_deformed_literal!(potential_num))),
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
            peek.is_ascii_alphabetic()
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

    fn tok_single_chr(&mut self, ch: char) -> Option<LexerResult<Token>> {
        let tok = match ch {
            '+' => {
                self.ptr += 1;
                Some(Token::Math(Math::Add))
            }
            '-' => {
                self.ptr += 1;
                Some(Token::Math(Math::Sub))
            }
            '*' => {
                self.ptr += 1;
                Some(Token::Math(Math::Mul))
            }
            '/' => {
                self.ptr += 1;
                Some(Token::Math(Math::Div))
            }
            '.' => {
                self.ptr += 1;
                Some(Token::Symbol(Character::Output))
            }
            ':' => {
                self.ptr += 1;
                Some(Token::Symbol(Character::BeginWord))
            }
            ';' => {
                self.ptr += 1;
                Some(Token::Symbol(Character::EndWord))
            }
            '>' => {
                self.ptr += 1;
                Some(Token::Symbol(Character::Gt))
            }
            '<' => {
                self.ptr += 1;
                Some(Token::Symbol(Character::Lt))
            }
            '=' => {
                self.ptr += 1;
                Some(Token::Symbol(Character::Equal))
            }
            _ => None,
        };

        macro_rules! ret {
            ($t:expr) => {
                if let Some(t) = $t {
                    println!("ret some");
                    Some(Ok(t))
                } else {
                    println!("ret none");
                    None
                }
            };
        }

        let peek_pos = if self.ptr > 2 {
            self.ptr - 2
        } else {
            return ret!(tok);
        };

        let peek_res = unwrap_to_eof_option!(self.get(peek_pos));

        if peek_res != ' ' && peek_pos > 5 {
            println!("err");
            return Some(Err(lex_err!(0, peek_pos, v_unexpected_tok!(peek_res))));
        } else {
            return ret!(tok);
        }
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
            while {
                let pk = unwrap_to_eof_option!(self.peek());
                dbg!(&pk);
                pk != '\n'
            } {
                self.peek += 1;

                if self.peek >= self.data_len {
                    break;
                }
            }

            self.ptr = self.peek;
            return None;
        }

        // Numbers
        if ch.is_ascii_digit() {
            return Some(self.tokenize_number());
        }

        // Words
        if ch.is_ascii_alphabetic() {
            if let Some(w) = self.tokenize_word() {
                return Some(w);
            }
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
