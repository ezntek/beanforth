#[macro_export]
macro_rules! eof {
    () => {
        Token::Eof
    };
}

#[macro_export]
macro_rules! nothing {
    () => {
        Token::Nothing
    };
}

#[macro_export]
macro_rules! err_loc {
    ($line:expr, $col:expr) => {
        types::ErrorLocation {
            line: $line,
            column: $col,
        }
    };
}

#[macro_export]
macro_rules! lex_err {
    ($line:expr, $col:expr, $variant:expr) => {
        error::LexerError {
            pos: err_loc!($line, $col),
            note: String::new(),
            variant: $variant,
        }
    };
}

#[macro_export]
macro_rules! note_lex_err {
    ($line:expr, $col:expr, $variant:expr, $note:expr) => {
        error::LexerError {
            pos: err_loc!($line, $col),
            note: $note.to_owned(),
            variant: $variant,
        }
    };
}

// COPIED FROM common_macros
#[macro_export]
macro_rules! const_expr_count {
    () => (0);
    ($e:expr) => (1);
    ($e:expr; $($other_e:expr);*) => ({
        1 $(+ $crate::const_expr_count!($other_e) )*
    });

    ($e:expr; $($other_e:expr);* ; ) => (
        $crate::const_expr_count! { $e; $($other_e);* }
    );
}

#[macro_export]
macro_rules! hash_map {
    (with $map:expr; insert { $($key:expr => $val:expr),* , }) => (
        $crate::hash_map!(with $map; insert { $($key => $val),* })
    );
    (with $map:expr; insert { $($key:expr => $val:expr),* }) => ({
        let count = $crate::const_expr_count!($($key);*);
        #[allow(unused_mut)]
        let mut map = $map;
        map.reserve(count);
        $(
            map.insert($key, $val);
        )*
        map
    });
    ($($key:expr => $val:expr),* ,) => (
        $crate::hash_map!($($key => $val),*)
    );
    ($($key:expr => $val:expr),*) => ({
        let start_capacity = $crate::const_expr_count!($($key);*);
        #[allow(unused_mut)]
        let mut map = ::std::collections::HashMap::with_capacity(start_capacity);
        $( map.insert($key, $val); )*
        map
    });
}
