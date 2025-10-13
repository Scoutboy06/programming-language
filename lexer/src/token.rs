use super::keywords::Keyword;
use crate::operators::Operator;

use string_cache::DefaultAtom as Atom;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn is(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }

    pub fn eof() -> Self {
        Self {
            kind: TokenKind::Eof,
            value: TokenValue::None,
            start: 0,
            end: 0,
        }
    }

    pub fn is_operator(&self) -> bool {
        matches!(self.kind, TokenKind::Operator)
    }

    pub fn as_operator(&self) -> Option<Operator> {
        use TokenValue as TV;
        match self.value {
            TV::Operator(op) => Some(op),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TokenValue {
    #[default]
    None,
    Consumed,
    Number(f64),
    String(String),
    Boolean(bool),
    Keyword(Keyword),
    Identifier(Atom),
    Regex(RegexValue),
    Operator(Operator),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegexValue {
    pub pattern: String,
    pub flags: String,
}

impl TokenValue {
    pub fn expect_none(&self) {
        match self {
            TokenValue::None => {}
            _ => unreachable!("Expected a None value"),
        }
    }

    pub fn expect_number(&self) -> f64 {
        match self {
            TokenValue::Number(num) => *num,
            _ => unreachable!("Expected a Number token"),
        }
    }

    pub fn expect_boolean(&self) -> bool {
        match self {
            TokenValue::Boolean(b) => *b,
            _ => unreachable!("Expected a Boolean token"),
        }
    }

    pub fn expect_string(&self) -> &str {
        match self {
            TokenValue::String(s) => &s,
            _ => unreachable!("Expected a String token"),
        }
    }

    /// Consumes the token's string value
    pub fn consume_string(&mut self) -> String {
        match std::mem::replace(self, Self::Consumed) {
            Self::String(s) => s,
            _ => unreachable!("Expected a String token"),
        }
    }

    pub fn expect_keyword(&self) -> Keyword {
        match self {
            TokenValue::Keyword(kw) => *kw,
            _ => unreachable!("Expected a Keyword token"),
        }
    }

    pub fn expect_identifier(&self) -> &Atom {
        match self {
            TokenValue::Identifier(atom) => atom,
            _ => unreachable!("Expected an Identifier token"),
        }
    }

    pub fn expect_regex(&self) -> &RegexValue {
        match self {
            TokenValue::Regex(r) => &r,
            _ => unreachable!("Expected a Regex token"),
        }
    }

    pub fn consume_regex(&mut self) -> RegexValue {
        match std::mem::replace(self, Self::Consumed) {
            TokenValue::Regex(r) => r,
            _ => unreachable!("Expected a Regex token"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum TokenKind {
    // Special tokens
    #[default]
    Invalid,
    Consumed,
    Eof,
    MultiLineComment,
    SingleLineComment,

    // Identifiers and literals
    Keyword,
    Identifier,
    String,
    Number,
    Boolean,
    Null,
    RegexLiteral,
    Operator,

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
    QuestionMark, // ?

    // Other
    ArrowFn,
}
