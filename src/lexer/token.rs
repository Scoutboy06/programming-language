use std::rc::Rc;

use string_cache::DefaultAtom as Atom;

use super::{keywords::Keyword, Span};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Token {
    pub kind: Kind,
    pub value: TokenValue,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TokenValue {
    #[default]
    None,
    Number(f64),
    String(Atom),
    Keyword(Keyword),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Kind {
    // Special tokens
    #[default]
    Invalid,
    Eof,
    Shebang,

    // Identifiers and literals
    Keyword,
    Identifier,
    String,
    Number,
    Null,

    // Punctuation
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    Dot,          // .
    Comma,        // ,
    Colon,        // :
    SemiColon,    // ;

    // Operators
    Equals,         // =
    Plus,           // +
    Minus,          // -
    Slash,          // /
    Asterisk,       // *
    Exponentiation, // **
    Modulus,        // %
    Increment,      // ++
    Decrement,      // --
    Ternary,        // ?

    // Compound operators
    PlusEquals,  // +=
    MinusEquals, // -=
    TimesEquals, // *=
    DivEquals,   // /=
    PowerEquals, // **=
    ModEquals,   // %=

    // Logical operators
    LogicalOr,  // ||
    LogicalAnd, // &&

    // Comparison operators
    DoubleEquals,       // ==
    TripleEquals,       // ===
    NotEqual,           // !=
    StrictNotEqual,     // !==
    LessThan,           // <
    GreaterThan,        // >
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=

    // Bitwise operators
    BitwiseAnd,                // &
    BitwiseOr,                 // |
    BitwiseNot,                // ~
    BitwiseXor,                // ^
    BitwiseLeftShift,          // >>
    BitwiseRightShift,         // <<
    BitwiseUnsignedRightShift, // >>>
}

impl TokenValue {
    pub fn expect_none(&self) {
        match self {
            TokenValue::None => {}
            _ => unreachable!(),
        }
    }

    pub fn expect_number(&self) -> f64 {
        match self {
            TokenValue::Number(num) => num.clone(),
            _ => unreachable!(),
        }
    }

    pub fn expect_string(&self) -> &Atom {
        match self {
            TokenValue::String(atom) => atom,
            _ => unreachable!(),
        }
    }

    pub fn expect_keyword(&self) -> Keyword {
        match self {
            TokenValue::Keyword(kw) => kw.clone(),
            _ => unreachable!(),
        }
    }
}
