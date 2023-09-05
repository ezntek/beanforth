use crate::lexer::Lexer;

mod lexer;
fn main() {
    let input = "1 4 + 5 / cr";
    let mut lexer = Lexer::new(input.to_owned());
    println!("{:?}", lexer.tokenize());
    println!("beans!");
}
