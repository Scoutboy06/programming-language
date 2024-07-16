use lazy_static::lazy_static;
use std::{collections::HashMap, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Special tokens
    Invalid(char),
    EndOfFile,

    // Identifiers and literals
    Identifier(String),
    Keyword(Keyword),
    Comment(String),
    StringLiteral(String),
    NumberLiteral(String),
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

    // Type operators,
    Typeof,
    Instanceof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    // Javascript keywords
    Var,
    Let,
    Const,
    Function,
    Async,
    Await,
    Static,
    If,
    Else,
    Try,
    Catch,
    Finally,
    While,
    Do,
    For,
    In,
    Of,
    Break,
    Continue,
    Class,
    Abstract,
    Extends,
    Implements,
    New,
    This,
    Super,
    Private,
    Protected,
    Switch,
    Case,
    Default,
    Type,
    Interface,
    True,
    False,

    // Typescript keywords
    StringType,  // : string
    NumberType,  // : number
    BooleanType, // : boolean
    RecordType,  // : Record<>
    ArrayType,   // : Array<>
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Keyword> = {
        let mut m = HashMap::new();

        // JavaScript keywords
        m.insert("var", Keyword::Var);
        m.insert("let", Keyword::Let);
        m.insert("const", Keyword::Const);
        m.insert("function", Keyword::Function);
        m.insert("async", Keyword::Async);
        m.insert("await", Keyword::Await);
        m.insert("static", Keyword::Static);
        m.insert("if", Keyword::If);
        m.insert("else", Keyword::Else);
        m.insert("try", Keyword::Try);
        m.insert("catch", Keyword::Catch);
        m.insert("finally", Keyword::Finally);
        m.insert("while", Keyword::While);
        m.insert("do", Keyword::Do);
        m.insert("for", Keyword::For);
        m.insert("fn", Keyword::In);
        m.insert("of", Keyword::Of);
        m.insert("break", Keyword::Break);
        m.insert("continue", Keyword::Continue);
        m.insert("class", Keyword::Class);
        m.insert("abstract", Keyword::Abstract);
        m.insert("extends", Keyword::Extends);
        m.insert("implements", Keyword::Implements);
        m.insert("new", Keyword::New);
        m.insert("this", Keyword::This);
        m.insert("super", Keyword::Super);
        m.insert("private", Keyword::Private);
        m.insert("protected", Keyword::Protected);
        m.insert("switch", Keyword::Switch);
        m.insert("case", Keyword::Case);
        m.insert("default", Keyword::Default);
        m.insert("type", Keyword::Type);
        m.insert("interface", Keyword::Interface);
        m.insert("true", Keyword::True);
        m.insert("false", Keyword::False);

        // TypeScript types
        m.insert("string", Keyword::StringType);
        m.insert("number", Keyword::NumberType);
        m.insert("boolean", Keyword::BooleanType);
        m.insert("Record", Keyword::RecordType);
        m.insert("Array", Keyword::ArrayType);

        m
    };
}

impl Keyword {
    fn from_str(s: &str) -> Option<Keyword> {
        KEYWORDS.get(s).cloned()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub value: TokenType,
    pub span: Span,
}

pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
    curr_char: Option<char>,
    peek_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Self {
            source,
            position: 0,
            curr_char: source.chars().nth(0),
            peek_char: source.chars().nth(1),
        };

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.curr_char.is_none() {
            return Token {
                value: TokenType::EndOfFile,
                span: Span {
                    start: self.position,
                    end: self.position,
                },
            };
        }

        let start_pos = self.position;

        let value: TokenType = match self.curr_char.unwrap() {
            '0'..='9' => TokenType::NumberLiteral(self.parse_number()),
            '+' => {
                self.advance();
                match self.curr_char {
                    Some('+') => {
                        self.advance();
                        TokenType::Increment
                    }
                    Some('=') => {
                        self.advance();
                        TokenType::PlusEquals
                    }
                    _ => TokenType::Plus,
                }
            }
            '-' => {
                self.advance();
                match self.curr_char {
                    Some('-') => {
                        self.advance();
                        TokenType::Decrement
                    }
                    Some('=') => {
                        self.advance();
                        TokenType::MinusEquals
                    }
                    _ => TokenType::Minus,
                }
            }
            '*' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        TokenType::TimesEquals
                    }
                    Some('*') => {
                        self.advance();
                        match self.curr_char {
                            Some('*') => {
                                self.advance();
                                TokenType::PowerEquals
                            }
                            _ => TokenType::Exponentiation,
                        }
                    }
                    _ => TokenType::Asterisk,
                }
            }
            '/' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        TokenType::DivEquals
                    }
                    Some('/') => todo!("Single-line comments"),
                    Some('*') => todo!("Multi-line comments"),
                    _ => TokenType::Slash,
                }
            }
            '%' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        TokenType::ModEquals
                    }
                    _ => TokenType::Modulus,
                }
            }
            '&' => {
                self.advance();
                match self.curr_char {
                    Some('&') => {
                        self.advance();
                        TokenType::LogicalAnd
                    }
                    _ => TokenType::BitwiseAnd,
                }
            }
            '|' => {
                self.advance();
                match self.curr_char {
                    Some('|') => {
                        self.advance();
                        TokenType::LogicalOr
                    }
                    _ => TokenType::BitwiseOr,
                }
            }
            '?' => {
                self.advance();
                TokenType::Ternary
            }
            '(' => {
                self.advance();
                TokenType::OpenParen
            }
            ')' => {
                self.advance();
                TokenType::CloseParen
            }
            '{' => {
                self.advance();
                TokenType::OpenBrace
            }
            '}' => {
                self.advance();
                TokenType::CloseBrace
            }
            '[' => {
                self.advance();
                TokenType::OpenBracket
            }
            ']' => {
                self.advance();
                TokenType::CloseBracket
            }
            '.' => {
                self.advance();
                TokenType::Dot
            }
            ',' => {
                self.advance();
                TokenType::Comma
            }
            ':' => {
                self.advance();
                TokenType::Colon
            }
            ';' => {
                self.advance();
                TokenType::SemiColon
            }
            '=' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        match self.curr_char {
                            Some('=') => {
                                self.advance();
                                TokenType::TripleEquals
                            }
                            _ => TokenType::DoubleEquals,
                        }
                    }
                    _ => TokenType::Equals,
                }
            }
            '>' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        TokenType::GreaterThanOrEqual
                    }
                    _ => TokenType::GreaterThan,
                }
            }
            '<' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        TokenType::LessThanOrEqual
                    }
                    _ => TokenType::LessThan,
                }
            }
            'a'..='z' | 'A'..='Z' | '_' | '&' => {
                let word = self.parse_identifier();
                if let Some(keyword) = Keyword::from_str(word.as_str()) {
                    TokenType::Keyword(keyword)
                } else {
                    TokenType::Identifier(word.to_owned())
                }
            }
            '"' | '\'' => TokenType::StringLiteral(self.parse_string_literal()),
            '`' => TokenType::StringLiteral(self.parse_template_string()),
            _ => TokenType::Invalid(self.curr_char.unwrap()),
        };

        Token {
            value,
            span: Span {
                start: start_pos,
                end: self.position,
            },
        }
    }

    fn advance(&mut self) {
        if self.position >= self.source.len() - 1 {
            self.curr_char = None;
            self.peek_char = None;
            self.position = self.source.len();
            return;
        }

        self.position += self.curr_char.unwrap().len_utf8();

        self.curr_char = self.peek_char;
        self.peek_char = self.source[self.position + 1..].chars().next();
    }

    fn skip_whitespace(&mut self) {
        while self.curr_char.is_some_and(|ch| ch.is_whitespace()) {
            self.advance();
        }
    }

    fn parse_identifier(&mut self) -> String {
        let start_pos = self.position;

        loop {
            match self.curr_char {
                Some('a'..='z' | 'A'..='Z' | '_' | '$') => self.advance(),
                Some('0'..='9') if self.position > start_pos => self.advance(),
                _ => break,
            }
        }

        self.source[start_pos..self.position].to_owned()
    }

    fn parse_number(&mut self) -> String {
        let start_pos = self.position;
        let mut has_decimal = false;

        loop {
            match self.curr_char {
                Some('0'..='9') => self.advance(),
                Some('.') => {
                    if !has_decimal && self.peek_char.is_some_and(|ch| ch >= '0' && ch <= '9') {
                        has_decimal = true;
                        self.advance();
                        self.advance();
                        continue;
                    } else {
                        break;
                    }
                }
                Some('n') => {
                    self.advance();
                    break;
                }
                _ => break,
            }
        }

        self.source[start_pos..self.position].to_owned()
    }

    fn parse_string_literal(&mut self) -> String {
        if self.curr_char == Some('`') {
            return self.parse_template_string();
        }

        let quote_type = self.curr_char.unwrap();
        let start_pos = self.position;

        while let Some(ch) = self.curr_char {
            if ch == '\\' {
                self.advance(); // Skip \ character
                self.advance(); // Skip escaped character
                continue;
            }

            if ch == quote_type && self.position != start_pos {
                self.advance();
                break;
            }

            self.advance();
        }

        self.source[start_pos..self.position].to_owned()
    }

    fn parse_template_string(&mut self) -> String {
        let start_pos = self.position;
        let mut brace_depth = 0;

        self.advance(); // Skip initial ` character

        while let Some(ch) = self.curr_char {
            if ch == '`' && brace_depth == 0 {
                self.advance(); // Skip the closing ` character
                break;
            } else if ch == '\\' {
                self.advance(); // Skip '\'
                self.advance(); // Skip escaped character
            } else if ch == '$' && self.peek_char == Some('{') {
                self.advance(); // Skip '&'
                self.advance(); // Skip '{'
                brace_depth += 1;
            } else if ch == '}' && brace_depth > 0 {
                self.advance(); // Skip '}'
                brace_depth -= 1;
            } else {
                self.advance();
            }
        }

        self.source[start_pos..self.position].to_owned()
    }

    fn multi_line_comment(&mut self) -> String {
        let mut value = String::new();

        while self.curr_char.is_some_and(|ch| ch != '*')
            && self.peek_char.is_some_and(|ch| ch != '/')
        {
            value.push(self.curr_char.unwrap());
            self.advance();
        }

        // Skip */ characters
        self.advance();
        self.advance();

        value
    }
}

#[cfg(test)]
mod tests {
    use super::{Keyword, Lexer, TokenType};

    #[test]
    fn numbers() {
        let source_code = "8 + 5 - 2 / 2";
        let mut lexer = Lexer::new(&source_code);

        let expected_tokens = vec![
            TokenType::NumberLiteral("8".to_string()),
            TokenType::Plus,
            TokenType::NumberLiteral("5".to_string()),
            TokenType::Minus,
            TokenType::NumberLiteral("2".to_string()),
            TokenType::Slash,
            TokenType::NumberLiteral("2".to_string()),
        ];

        for i in 0.. {
            let token = lexer.next_token();

            if token.value == TokenType::EndOfFile {
                assert_eq!(i, expected_tokens.len());
                break;
            }

            assert_eq!(token.value, expected_tokens[i]);
        }
    }

    #[test]
    fn let_statement() {
        let source_code = "let x = 123 + 456;";
        let mut lexer = Lexer::new(&source_code);

        let expected_tokens = vec![
            TokenType::Keyword(Keyword::Let),
            TokenType::Identifier("x".to_string()),
            TokenType::Equals,
            TokenType::NumberLiteral("123".to_string()),
            TokenType::Plus,
            TokenType::NumberLiteral("456".to_string()),
            TokenType::SemiColon,
        ];

        for i in 0.. {
            let token = lexer.next_token();

            if token.value == TokenType::EndOfFile {
                assert_eq!(i, expected_tokens.len());
                break;
            }

            assert_eq!(token.value, expected_tokens[i]);
        }
    }

    #[test]
    fn function() {
        let source_code = "function sum(n1: number, n2: number): number {}";
        let mut lexer = Lexer::new(&source_code);

        let expected_tokens = vec![
            TokenType::Keyword(Keyword::Function),
            TokenType::Identifier("sum".to_string()),
            TokenType::OpenParen,
            TokenType::Identifier("n1".to_string()),
            TokenType::Colon,
            TokenType::Keyword(Keyword::NumberType),
            TokenType::Comma,
            TokenType::Identifier("n2".to_string()),
            TokenType::Colon,
            TokenType::Keyword(Keyword::NumberType),
            TokenType::CloseParen,
            TokenType::Colon,
            TokenType::Keyword(Keyword::NumberType),
            TokenType::OpenBrace,
            TokenType::CloseBrace,
        ];

        for i in 0.. {
            let token = lexer.next_token();

            if token.value == TokenType::EndOfFile {
                assert_eq!(i, expected_tokens.len());
                break;
            }

            assert_eq!(token.value, expected_tokens[i]);
        }
    }

    #[test]
    fn string_literal() {
        let source_code = "let x = 'This is a string literal';";
        let mut lexer = Lexer::new(&source_code);

        let expected_tokens = vec![
            TokenType::Keyword(Keyword::Let),
            TokenType::Identifier("x".to_string()),
            TokenType::Equals,
            TokenType::StringLiteral("'This is a string literal'".to_string()),
            TokenType::SemiColon,
        ];

        for i in 0.. {
            let token = lexer.next_token();

            if token.value == TokenType::EndOfFile {
                assert_eq!(i, expected_tokens.len());
                break;
            }

            assert_eq!(token.value, expected_tokens[i]);
        }
    }

    #[test]
    fn template_string_literal() {
        let source_code = "let x = `A ${string_type} string with ${is_nested ? `${nested_level} nestings` : ''}`;";
        let mut lexer = Lexer::new(&source_code);

        let expected_tokens = vec![
            TokenType::Keyword(Keyword::Let),
            TokenType::Identifier("x".to_string()),
            TokenType::Equals,
            TokenType::StringLiteral(
                "`A ${string_type} string with ${is_nested ? `${nested_level} nestings` : ''}`"
                    .to_string(),
            ),
            TokenType::SemiColon,
        ];

        for i in 0.. {
            let token = lexer.next_token();

            if token.value == TokenType::EndOfFile {
                assert_eq!(i, expected_tokens.len());
                break;
            }

            assert_eq!(token.value, expected_tokens[i]);
        }
    }

    #[test]
    fn if_statement_with_boolean() {
        let source_code = "if(false) {} else if(true) {} else {}";
        let mut lexer = Lexer::new(&source_code);

        let expected_tokens = vec![
            TokenType::Keyword(Keyword::If),
            TokenType::OpenParen,
            TokenType::Keyword(Keyword::False),
            TokenType::CloseParen,
            TokenType::OpenBrace,
            TokenType::CloseBrace,
            TokenType::Keyword(Keyword::Else),
            TokenType::Keyword(Keyword::If),
            TokenType::OpenParen,
            TokenType::Keyword(Keyword::True),
            TokenType::CloseParen,
            TokenType::OpenBrace,
            TokenType::CloseBrace,
            TokenType::Keyword(Keyword::Else),
            TokenType::OpenBrace,
            TokenType::CloseBrace,
        ];

        for i in 0.. {
            let token = lexer.next_token();

            if token.value == TokenType::EndOfFile {
                assert_eq!(i, expected_tokens.len());
                break;
            }

            assert_eq!(token.value, expected_tokens[i]);
        }
    }

    #[test]
    fn if_statement_with_variables() {
        let source_code = "if (this.pos.x > window.innerWidth) {}";
        let mut lexer = Lexer::new(&source_code);

        let expected_tokens = vec![
            TokenType::Keyword(Keyword::If),
            TokenType::OpenParen,
            TokenType::Keyword(Keyword::This),
            TokenType::Dot,
            TokenType::Identifier("pos".to_string()),
            TokenType::Dot,
            TokenType::Identifier("x".to_string()),
            TokenType::GreaterThan,
            TokenType::Identifier("window".to_string()),
            TokenType::Dot,
            TokenType::Identifier("innerWidth".to_string()),
            TokenType::CloseParen,
            TokenType::OpenBrace,
            TokenType::CloseBrace,
        ];

        for i in 0.. {
            let token = lexer.next_token();

            if token.value == TokenType::EndOfFile {
                assert_eq!(i, expected_tokens.len());
                break;
            }

            assert_eq!(token.value, expected_tokens[i]);
        }
    }
}
