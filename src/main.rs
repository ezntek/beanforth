mod lexer;
mod parser;

use crate::lexer::Lexer;
fn main() {
    let buf = std::fs::read_to_string("./lexer_input.fth").unwrap();
    dbg!(&buf);

    let mut lexer = Lexer::new(buf);
    println!("{:?}", lexer.tokenize());
    println!("beans!");
}
