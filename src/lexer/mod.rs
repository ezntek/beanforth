pub mod types;
#[macro_use]
mod macrodefs;

use super::error::*;
use std::num::IntErrorKind;

#[allow(unused_imports)]
use macrodefs::*;
use types::*;

#[derive(Debug)]
pub struct Lexer {
    data: Vec<char>,
    data_len: usize,
    ptr: usize,
    peek: usize,
    newlines: Box<[usize]>,
}

macro_rules! unwrap_to_eof_option {
    ($optchar:expr) => {
        match $optchar {
            Some(ch) => ch,
            None => return Some(Ok(Token::End)),
        }
    };
}

fn get_newline_positions_string(s: &str) -> Box<[usize]> {
    let mut res = Vec::new();
    for (ch_i, ch) in s.chars().enumerate() {
        if ch == '\n' {
            res.push(ch_i)
        }
    }
    dbg!(&res);
    res.into_boxed_slice()
}

impl Lexer {
    pub fn new(data: String) -> Self {
        Lexer {
            data: data.chars().collect::<Vec<char>>(),
            data_len: data.len(),
            ptr: 0,
            peek: 0,
            newlines: get_newline_positions_string(&data),
        }
    }

    fn get_err_pos(&self, pos: usize) -> ErrorLocation {
        for (idx, loc) in self.newlines.iter().enumerate() {
            if &pos <= loc {
                let line = idx + 1;
                // Get previous line no. and get the offset
                // from it by subtracting `pos` from it.
                let col = pos - (self.newlines[idx - 1]);

                return err_loc!(line, col);
            }
        }
        // fallback case
        err_loc!(self.newlines[self.newlines.len() - 1], 0)
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
        let mut curr_num_ch = match self.at() {
            Some(ch) => ch,
            None => return Ok(eof!()),
        };

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
                IntErrorKind::PosOverflow => Err(err_with_note!(
                    self.get_err_pos(self.ptr - potential_num.len()),
                    v_deformed_literal!(potential_num),
                    "Number is too big to be a signed 32bit integer!"
                )),
                IntErrorKind::NegOverflow => Err(err_with_note!(
                    self.get_err_pos(self.ptr - potential_num.len()),
                    v_deformed_literal!(potential_num),
                    "Number is too small to be a signed 32bit integer!"
                )),
                _ => Err(err!(
                    self.get_err_pos(self.ptr - potential_num.len()),
                    v_deformed_literal!(potential_num)
                )),
            },
        }
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
                    Some(Ok(t))
                } else {
                    None
                }
            };
        }

        let peek_pos = if self.ptr > 1 {
            self.ptr - 2
        } else {
            return ret!(tok);
        };

        let peek_res = unwrap_to_eof_option!(self.get(peek_pos));

        let is_not_whitespace = !peek_res.is_ascii_whitespace() && peek_pos > 1;
        if is_not_whitespace && !tok.is_none() {
            return Some(Err(err!(
                self.get_err_pos(self.ptr),
                v_unexpected_tok!(peek_res)
            )));
        } else {
            return ret!(tok);
        }
    }

    fn read_word(&mut self) -> String {
        let mut s = String::new();

        self.peek = self.ptr;

        // it is not memoized twice so that
        // the optional property of the memo
        // result can indicate if it is
        // out of bounds or not.
        let mut peek_memo: Option<char>;

        'outerloop: while {
            // memoize the result of the peek.
            // avoids calling peek twice.
            peek_memo = self.peek();

            // manually unwrapped to be able to run
            // code that is placed at the bottom
            // of the function.
            //
            // Very convoluted is_ascii_whitespace call.
            !(match peek_memo {
                Some(pk) => pk,
                None => break 'outerloop,
            })
            .is_ascii_whitespace()
        } {
            let peek = match peek_memo {
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
        s
    }

    fn tokenize_word(&mut self) -> LexerResult<Token> {
        let word_string = self.read_word();
        dbg!(&word_string);
        Ok(Token::Word(word_string))
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

        // Single character symbols
        if let Some(r) = self.tok_single_chr(ch) {
            return Some(r);
        }

        // Everything else is a word
        {
            let word = match self.tokenize_word() {
                Ok(w) => w,
                Err(e) => {
                    return Some(Err(e));
                }
            };

            if let Token::Word(wd_s) = &word {
                if let Some(rw) = ReservedWord::try_to_string(wd_s) {
                    return Some(Ok(Token::ReservedWord(rw)));
                } else {
                    Some(Ok(word))
                }
            } else {
                unreachable!()
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut res: Vec<Token> = Vec::from([Token::Begin]);

        while self.ptr < self.data_len {
            if let Some(r) = self.tokenize_at_ptr() {
                let tok = match r {
                    Ok(t) => t,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                };

                if let Token::End = tok {
                    break;
                }

                res.push(tok);
            }
        }

        res.push(Token::End);
        res
    }
}
