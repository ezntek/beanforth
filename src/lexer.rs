#[derive(Debug)]
pub enum MathToken {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Token {
    Math(MathToken),
    Literal(i32),
    Invalid,
    Nothing,
    Eof,
}

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
        dbg!(self.ptr);
        self.data[self.ptr]
    }

    fn peek(&self) -> char {
        dbg!(self.peek);
        self.data[self.peek]
    }

    fn analyze(&mut self) -> Token {
        match char::from(self.at()) {
            // Maths Operators
            '+' => {
                self.ptr += 1;
                Token::Math(MathToken::Add)
            }
            '-' => {
                self.ptr += 1;
                Token::Math(MathToken::Sub)
            }
            '*' => {
                self.ptr += 1;
                Token::Math(MathToken::Mul)
            }
            '/' => {
                self.ptr += 1;
                Token::Math(MathToken::Div)
            }

            // Comments
            '\\' => {
                println!("comment");
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

                Token::Nothing
            }

            // Constants
            ch => {
                println!("other");
                let numbers = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

                dbg!(ch);

                if numbers.contains(&ch) {
                    println!("nomber");
                    let mut curr_num_ch: char = self.at();
                    let mut potential_num = String::from(curr_num_ch);

                    self.peek = self.ptr + 1;
                    while numbers.contains(&curr_num_ch) {
                        curr_num_ch = self.peek();
                        dbg!(curr_num_ch);
                        if !numbers.contains(&curr_num_ch) {
                            break;
                        }
                        potential_num.push(curr_num_ch);
                        self.peek += 1;
                    }

                    self.ptr = self.peek + 1;

                    let num = potential_num
                        .parse::<i32>()
                        .expect("falsefully parsed a non-number as a number somewhere!");
                    Token::Literal(num)
                } else {
                    self.ptr += 1;
                    Token::Invalid
                }
            }
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut res: Vec<Token> = Vec::new();

        while self.ptr < self.data.len() {
            res.push(self.analyze());
        }

        res.push(Token::Eof);
        res
    }
}
