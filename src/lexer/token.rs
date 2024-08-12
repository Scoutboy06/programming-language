use std::rc::Rc;

use string_cache::DefaultAtom as Atom;

use super::{keywords::Keyword, Span};

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: Kind,
    pub value: TokenValue,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    // Special tokens
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
    Equals,     // =
    Plus,       // +
    Minus,      // -
    Slash,      // /
    Asterisk,   // *
    LogicalOr,  // ||
    LogicalAnd, // &&

    // Compound operators
    PlusEquals,  // +=
    MinusEquals, // -=
    TimesEquals, // *=
    DivEquals,   // /=
    PowerEquals, // **=
    ModEquals,   // %=

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

    // Other operators
    Exponentiation, // **
    Modulus,        // %
    Increment,      // ++
    Decrement,      // --
    Ternary,        // ?
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    None,
    Number(f64),
    String(Atom),
    Keyword(Keyword),
}
