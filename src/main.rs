mod lexer;
mod parser;

use crate::{lexer::Lexer, parser::Parser};
fn main() {
    let buf = std::fs::read_to_string("./lexer_input.fth").unwrap();
    dbg!(&buf);

    let mut lexer = Lexer::new(buf);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);

    let ast = Parser::new(tokens).parse();
    println!("{:?}", ast);
    println!("beans!");
}
