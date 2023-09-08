mod lexer;
mod parser;

use crate::lexer::Lexer;
fn main() {
    let input = "
: add 1 2 + 3 - ; (comment)
add
    "; // \\foobar";
    let mut lexer = Lexer::new(input.to_owned());
    println!("{:?}", lexer.tokenize());
    println!("beans!");
}
