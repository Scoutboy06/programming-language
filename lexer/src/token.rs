use crate::operators::Operator;

use super::keywords::Keyword;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Token {
    pub kind: TokenKind,
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
    Boolean(bool),
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

    pub fn expect_boolean(&self) -> bool {
        match self {
            TokenValue::Boolean(b) => *b,
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
pub enum TokenKind {
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
    Boolean,
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
    Percent,        // %
    Increment,      // ++
    Decrement,      // --
    QuestionMark,   // ?

    // Assignment operators
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

impl TokenKind {
    pub fn is_operator(&self) -> bool {
        match self {
            // Highest precedence
            TokenKind::Exponentiation => true,

            // Multiplicative operations
            TokenKind::Asterisk | TokenKind::Slash | TokenKind::Percent => true,

            // Additive operators
            TokenKind::Plus | TokenKind::Minus => true,

            // Bitwise shift operators
            TokenKind::BitwiseLeftShift
            | TokenKind::BitwiseRightShift
            | TokenKind::ZeroFillRightShift => true,

            // Bitwise operators
            TokenKind::BitwiseAnd => true,
            TokenKind::BitwiseOr | TokenKind::BitwiseXor => true,
            TokenKind::BitwiseNot => true,

            // Logical operators
            TokenKind::LogicalAnd | TokenKind::LogicalOr => true,

            // Comparison operators
            TokenKind::DoubleEquals
            | TokenKind::TripleEquals
            | TokenKind::NotEqual
            | TokenKind::StrictNotEqual
            | TokenKind::LessThan
            | TokenKind::GreaterThan
            | TokenKind::LessThanOrEqual
            | TokenKind::GreaterThanOrEqual => true,
            _ => false,
        }
    }

    pub fn as_operator(&self) -> Option<Operator> {
        match self {
            // Highest precedence
            TokenKind::Exponentiation => Some(Operator::Power),

            // Multiplicative operations
            TokenKind::Asterisk => Some(Operator::Mult),
            TokenKind::Slash => Some(Operator::Div),
            TokenKind::Percent => Some(Operator::Mod),

            // Additive operators
            TokenKind::Plus => Some(Operator::Plus),
            TokenKind::Minus => Some(Operator::Minus),

            // Bitwise shift operators
            TokenKind::BitwiseLeftShift => Some(Operator::BitwiseLeftShift),
            TokenKind::BitwiseRightShift => Some(Operator::BitwiseRightShift),
            TokenKind::ZeroFillRightShift => Some(Operator::ZeroFillRightShift),

            // Bitwise operators
            TokenKind::BitwiseAnd => Some(Operator::BitwiseAnd),
            TokenKind::BitwiseOr => Some(Operator::BitwiseOr),
            TokenKind::BitwiseXor => Some(Operator::BitwiseXor),
            TokenKind::BitwiseNot => Some(Operator::BitwiseNot),

            // Logical operators
            TokenKind::LogicalAnd => Some(Operator::LogicalAnd),
            TokenKind::LogicalOr => Some(Operator::LogicalOr),

            // Comparison operators
            TokenKind::DoubleEquals => Some(Operator::Equals),
            TokenKind::TripleEquals => Some(Operator::StrictEquals),
            TokenKind::NotEqual => Some(Operator::NotEquals),
            TokenKind::StrictNotEqual => Some(Operator::StrictNotEquals),
            TokenKind::LessThan => Some(Operator::LessThan),
            TokenKind::GreaterThan => Some(Operator::GreaterThan),
            TokenKind::LessThanOrEqual => Some(Operator::LessOrEqualsThan),
            TokenKind::GreaterThanOrEqual => Some(Operator::GreaterOrEqualsThan),

            // Arithmetic operators
            TokenKind::PlusEquals => Some(Operator::PlusEquals),
            TokenKind::MinusEquals => Some(Operator::MinusEquals),
            TokenKind::TimesEquals => Some(Operator::TimesEquals),
            TokenKind::DivEquals => Some(Operator::DivEquals),
            TokenKind::PowerEquals => Some(Operator::PowerEquals),
            TokenKind::ModEquals => Some(Operator::ModEquals),

            _ => None,
        }
    }

    pub fn is_arithmetic_operator(&self) -> bool {
        match self {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Asterisk
            | TokenKind::Slash
            | TokenKind::Exponentiation
            | TokenKind::BitwiseAnd
            | TokenKind::BitwiseLeftShift
            | TokenKind::BitwiseNot
            | TokenKind::BitwiseOr
            | TokenKind::BitwiseRightShift
            | TokenKind::ZeroFillRightShift
            | TokenKind::BitwiseXor => true,
            _ => false,
        }
    }

    pub fn get_operator_precedence(&self) -> Option<u8> {
        match self {
            // Highest precedence
            TokenKind::Exponentiation => Some(4),

            // Multiplicative operations
            TokenKind::Asterisk | TokenKind::Slash | TokenKind::Percent => Some(3),

            // Additive operators
            TokenKind::Plus | TokenKind::Minus => Some(2),

            // Bitwise shift operators
            TokenKind::BitwiseLeftShift
            | TokenKind::BitwiseRightShift
            | TokenKind::ZeroFillRightShift => Some(2),

            // Bitwise operators
            TokenKind::BitwiseAnd => Some(1),
            TokenKind::BitwiseOr | TokenKind::BitwiseXor => Some(1),
            TokenKind::BitwiseNot => Some(1), // Typically not used in expressions with precedence

            // Logical operators
            TokenKind::LogicalAnd | TokenKind::LogicalOr => Some(1),

            // Comparison operators
            TokenKind::DoubleEquals
            | TokenKind::TripleEquals
            | TokenKind::NotEqual
            | TokenKind::StrictNotEqual
            | TokenKind::LessThan
            | TokenKind::GreaterThan
            | TokenKind::LessThanOrEqual
            | TokenKind::GreaterThanOrEqual => Some(1),

            _ => None,
        }
    }

    pub fn is_assignment_operator(&self) -> bool {
        match self {
            TokenKind::PlusEquals
            | TokenKind::MinusEquals
            | TokenKind::TimesEquals
            | TokenKind::DivEquals
            | TokenKind::PowerEquals
            | TokenKind::ModEquals => true,
            _ => false,
        }
    }
}
