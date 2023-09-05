use crate::lexer::Lexer;

mod lexer;
fn main() {
    let input = "  1      3 4  5 / + variable \\aarst "; // \\foobar";
    let mut lexer = Lexer::new(input.to_owned());
    println!("{:?}", lexer.tokenize());
    println!("beans!");
}
