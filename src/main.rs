use crate::lexer::Lexer;

mod lexer;
fn main() {
    let input = "1 4 + 5 / \\ arstarstarst";
    let mut lexer = Lexer::new(input.to_owned());
    println!("{:?}", lexer.lex());
    println!("beans!");
}
