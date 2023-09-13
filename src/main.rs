mod error;
mod lexer;
mod parser;

use crate::{lexer::Lexer, parser::Parser};
fn main() {
    let buf = std::fs::read_to_string("./lexer_input.fth").unwrap();
    dbg!(&buf);

    let mut lexer = Lexer::new(buf);
    let tokens = lexer.tokenize();
    let mut s = String::from("[");
    for tok in tokens.iter() {
        s.push_str(format!("{}", tok).as_str());
        s.push_str(", ");
    }
    s.push(']');

    println!("{}", s);

    let ast = Parser::new(tokens).parse();
    println!("{:?}", ast);
    println!("ns!");
}
