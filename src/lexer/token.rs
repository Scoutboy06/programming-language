use std::rc::Rc;

use string_cache::DefaultAtom as Atom;

use crate::parser::ArithmeticOperator;

use super::keywords::Keyword;

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
    BitwiseAnd,         // &
    BitwiseOr,          // |
    BitwiseNot,         // ~
    BitwiseXor,         // ^
    BitwiseLeftShift,   // >>
    BitwiseRightShift,  // <<
    ZeroFillRightShift, // >>>

    // Other
    ArrowFn,
}

impl Kind {
    pub fn is_operator(&self) -> bool {
        match self {
            Kind::Plus
            | Kind::Minus
            | Kind::Asterisk
            | Kind::Slash
            | Kind::Exponentiation
            | Kind::BitwiseAnd
            | Kind::BitwiseLeftShift
            | Kind::BitwiseNot
            | Kind::BitwiseOr
            | Kind::BitwiseRightShift
            | Kind::ZeroFillRightShift
            | Kind::BitwiseXor => true,
            _ => false,
        }
    }

    pub fn as_operator(&self) -> Option<ArithmeticOperator> {
        match self {
            Kind::Plus => Some(ArithmeticOperator::Plus),
            Kind::Minus => Some(ArithmeticOperator::Minus),
            Kind::Asterisk => Some(ArithmeticOperator::Mult),
            Kind::Slash => Some(ArithmeticOperator::Div),
            Kind::Exponentiation => Some(ArithmeticOperator::Power),
            Kind::BitwiseAnd => Some(ArithmeticOperator::BitwiseAnd),
            Kind::BitwiseLeftShift => Some(ArithmeticOperator::BitwiseLeftShift),
            Kind::BitwiseNot => Some(ArithmeticOperator::BitwiseNot),
            Kind::BitwiseOr => Some(ArithmeticOperator::BitwiseOr),
            Kind::BitwiseRightShift => Some(ArithmeticOperator::BitwiseRightShift),
            Kind::ZeroFillRightShift => Some(ArithmeticOperator::ZeroFillRightShift),
            Kind::BitwiseXor => Some(ArithmeticOperator::BitwiseXor),
            _ => None,
        }
    }

    pub fn as_term_operator(&self) -> Option<ArithmeticOperator> {
        // TODO: Verify order of operations
        match self {
            Kind::Plus => Some(ArithmeticOperator::Plus),
            Kind::Minus => Some(ArithmeticOperator::Minus),
            Kind::BitwiseAnd => Some(ArithmeticOperator::BitwiseAnd),
            Kind::BitwiseLeftShift => Some(ArithmeticOperator::BitwiseLeftShift),
            Kind::BitwiseNot => Some(ArithmeticOperator::BitwiseNot),
            Kind::BitwiseOr => Some(ArithmeticOperator::BitwiseOr),
            Kind::BitwiseRightShift => Some(ArithmeticOperator::BitwiseRightShift),
            Kind::ZeroFillRightShift => Some(ArithmeticOperator::ZeroFillRightShift),
            Kind::BitwiseXor => Some(ArithmeticOperator::BitwiseXor),
            _ => None,
        }
    }

    pub fn as_factor_operator(&self) -> Option<ArithmeticOperator> {
        match self {
            Kind::Asterisk => Some(ArithmeticOperator::Mult),
            Kind::Slash => Some(ArithmeticOperator::Div),
            _ => None,
        }
    }
}
