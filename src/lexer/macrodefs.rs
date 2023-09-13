#[macro_export]
macro_rules! eof {
    () => {
        Token::End
    };
}

#[macro_export]
macro_rules! nothing {
    () => {
        Token::Nothing
    };
}
