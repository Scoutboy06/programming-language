use super::keywords::Keyword;
use super::operators::ArithmeticOperator;
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

impl TokenKind {
    pub fn is_operator(&self) -> bool {
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

    pub fn as_operator(&self) -> Option<ArithmeticOperator> {
        match self {
            TokenKind::Plus => Some(ArithmeticOperator::Plus),
            TokenKind::Minus => Some(ArithmeticOperator::Minus),
            TokenKind::Asterisk => Some(ArithmeticOperator::Mult),
            TokenKind::Slash => Some(ArithmeticOperator::Div),
            TokenKind::Exponentiation => Some(ArithmeticOperator::Power),
            TokenKind::BitwiseAnd => Some(ArithmeticOperator::BitwiseAnd),
            TokenKind::BitwiseLeftShift => Some(ArithmeticOperator::BitwiseLeftShift),
            TokenKind::BitwiseNot => Some(ArithmeticOperator::BitwiseNot),
            TokenKind::BitwiseOr => Some(ArithmeticOperator::BitwiseOr),
            TokenKind::BitwiseRightShift => Some(ArithmeticOperator::BitwiseRightShift),
            TokenKind::ZeroFillRightShift => Some(ArithmeticOperator::ZeroFillRightShift),
            TokenKind::BitwiseXor => Some(ArithmeticOperator::BitwiseXor),
            _ => None,
        }
    }

    pub fn as_term_operator(&self) -> Option<ArithmeticOperator> {
        // TODO: Verify order of operations
        match self {
            TokenKind::Plus => Some(ArithmeticOperator::Plus),
            TokenKind::Minus => Some(ArithmeticOperator::Minus),
            TokenKind::BitwiseAnd => Some(ArithmeticOperator::BitwiseAnd),
            TokenKind::BitwiseLeftShift => Some(ArithmeticOperator::BitwiseLeftShift),
            TokenKind::BitwiseNot => Some(ArithmeticOperator::BitwiseNot),
            TokenKind::BitwiseOr => Some(ArithmeticOperator::BitwiseOr),
            TokenKind::BitwiseRightShift => Some(ArithmeticOperator::BitwiseRightShift),
            TokenKind::ZeroFillRightShift => Some(ArithmeticOperator::ZeroFillRightShift),
            TokenKind::BitwiseXor => Some(ArithmeticOperator::BitwiseXor),
            _ => None,
        }
    }

    pub fn as_factor_operator(&self) -> Option<ArithmeticOperator> {
        match self {
            TokenKind::Asterisk => Some(ArithmeticOperator::Mult),
            TokenKind::Slash => Some(ArithmeticOperator::Div),
            _ => None,
        }
    }
}
