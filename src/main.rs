use crate::lexer::Lexer;

mod lexer;
fn main() {
    let input = r#"1 4 \\ arstarstarst\n"#;
    let mut lexer = Lexer::new(input.to_owned());
    println!("{:?}", lexer.lex());
    println!("beans!");
}
