extern crate string_cache;
use super::{
    keywords::Keyword,
    token::{self, Kind, Token, TokenValue},
};
use std::{collections::HashMap, rc::Rc, str::Chars};
use string_cache::DefaultAtom as Atom;

pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
    position: usize,
    curr_char: Option<char>,
    peek_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars();
        let curr_char = chars.next();
        let peek_char = chars.next();

        let lexer = Self {
            source,
            chars,
            position: 0,
            curr_char,
            peek_char,
        };

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.curr_char.is_none() {
            return Token {
                kind: Kind::Eof,
                value: TokenValue::None,
                start: self.position,
                end: self.position,
            };
        }

        let start = self.position;

        let (token_kind, token_value) = match self.curr_char.unwrap() {
            '0'..='9' => (Kind::Number, TokenValue::Number(self.parse_number())),
            'a'..='z' | 'A'..='Z' | '_' | '$' => {
                let word = self.parse_identifier();
                if let Some(keyword) = Keyword::from_str(word) {
                    (Kind::Keyword, TokenValue::Keyword(keyword))
                } else {
                    (Kind::Identifier, TokenValue::String(Atom::from(word)))
                }
            }
            '"' | '\'' | '`' => (
                Kind::String,
                TokenValue::String(Atom::from(self.parse_string_literal())),
            ),
            '+' => {
                self.advance();
                match self.curr_char {
                    Some('+') => {
                        self.advance();
                        (Kind::Increment, TokenValue::None)
                    }
                    Some('=') => {
                        self.advance();
                        (Kind::PlusEquals, TokenValue::None)
                    }
                    _ => (Kind::Plus, TokenValue::None),
                }
            }
            '-' => {
                self.advance();
                match self.curr_char {
                    Some('-') => {
                        self.advance();
                        (Kind::Decrement, TokenValue::None)
                    }
                    Some('=') => {
                        self.advance();
                        (Kind::MinusEquals, TokenValue::None)
                    }
                    _ => (Kind::Minus, TokenValue::None),
                }
            }
            '*' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (Kind::TimesEquals, TokenValue::None)
                    }
                    Some('*') => {
                        self.advance();
                        match self.curr_char {
                            Some('*') => {
                                self.advance();
                                (Kind::PowerEquals, TokenValue::None)
                            }
                            _ => (Kind::Exponentiation, TokenValue::None),
                        }
                    }
                    _ => (Kind::Asterisk, TokenValue::None),
                }
            }
            '/' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (Kind::DivEquals, TokenValue::None)
                    }
                    Some('/') => todo!("Single-line comments"),
                    Some('*') => todo!("Multi-line comments"),
                    _ => (Kind::Slash, TokenValue::None),
                }
            }
            '%' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (Kind::ModEquals, TokenValue::None)
                    }
                    _ => (Kind::Modulus, TokenValue::None),
                }
            }
            '&' => {
                self.advance();
                match self.curr_char {
                    Some('&') => {
                        self.advance();
                        (Kind::LogicalAnd, TokenValue::None)
                    }
                    _ => (Kind::BitwiseAnd, TokenValue::None),
                }
            }
            '|' => {
                self.advance();
                match self.curr_char {
                    Some('|') => {
                        self.advance();
                        (Kind::LogicalOr, TokenValue::None)
                    }
                    _ => (Kind::BitwiseOr, TokenValue::None),
                }
            }
            '?' => {
                self.advance();
                (Kind::Ternary, TokenValue::None)
            }
            '(' => {
                self.advance();
                (Kind::OpenParen, TokenValue::None)
            }
            ')' => {
                self.advance();
                (Kind::CloseParen, TokenValue::None)
            }
            '{' => {
                self.advance();
                (Kind::OpenBrace, TokenValue::None)
            }
            '}' => {
                self.advance();
                (Kind::CloseBrace, TokenValue::None)
            }
            '[' => {
                self.advance();
                (Kind::OpenBracket, TokenValue::None)
            }
            ']' => {
                self.advance();
                (Kind::CloseBracket, TokenValue::None)
            }
            '.' => {
                self.advance();
                (Kind::Dot, TokenValue::None)
            }
            ',' => {
                self.advance();
                (Kind::Comma, TokenValue::None)
            }
            ':' => {
                self.advance();
                (Kind::Colon, TokenValue::None)
            }
            ';' => {
                self.advance();
                (Kind::SemiColon, TokenValue::None)
            }
            '=' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        match self.curr_char {
                            Some('=') => {
                                self.advance();
                                (Kind::TripleEquals, TokenValue::None)
                            }
                            _ => (Kind::DoubleEquals, TokenValue::None),
                        }
                    }
                    Some('>') => {
                        self.advance();
                        (Kind::ArrowFn, TokenValue::None)
                    }
                    _ => (Kind::Equals, TokenValue::None),
                }
            }
            '>' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (Kind::GreaterThanOrEqual, TokenValue::None)
                    }
                    _ => (Kind::GreaterThan, TokenValue::None),
                }
            }
            '<' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (Kind::LessThanOrEqual, TokenValue::None)
                    }
                    _ => (Kind::LessThan, TokenValue::None),
                }
            }
            _ => (Kind::Invalid, TokenValue::None),
        };

        Token {
            kind: token_kind,
            value: token_value,
            start,
            end: self.position,
        }
    }

    fn advance(&mut self) {
        if let Some(ch) = self.curr_char {
            self.position += ch.len_utf8();
        }
        self.curr_char = self.peek_char;
        self.peek_char = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while self.curr_char.is_some_and(|ch| ch.is_whitespace()) {
            self.advance();
        }
    }

    fn parse_identifier(&mut self) -> &str {
        let start_pos = self.position;

        loop {
            match self.curr_char {
                Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$') => self.advance(),
                _ => break,
            }
        }

        &self.source[start_pos..self.position]
    }

    fn parse_number(&mut self) -> f64 {
        let start_pos = self.position;
        let mut has_decimal = false;

        loop {
            match self.curr_char {
                Some('0'..='9') => self.advance(),
                Some('.') => {
                    if has_decimal
                        || self.peek_char.is_none()
                        || self.peek_char.is_some_and(|ch| ch < '0' || ch > '9')
                    {
                        break;
                    }

                    has_decimal = true;
                    self.advance();
                    self.advance();
                }
                Some('n') => {
                    self.advance();
                    break;
                }
                _ => break,
            }
        }

        self.source[start_pos..self.position]
            .parse::<f64>()
            .unwrap()
    }

    fn parse_string_literal(&mut self) -> &str {
        if self.curr_char.unwrap() == '`' {
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

        &self.source[start_pos..self.position]
    }

    fn parse_template_string(&mut self) -> &str {
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

        &self.source[start_pos..self.position]
    }

    fn multi_line_comment(&mut self) -> &str {
        let start_pos = self.position;

        while self.curr_char.is_some_and(|ch| ch != '*')
            && self.peek_char.is_some_and(|ch| ch != '/')
        {
            self.advance();
        }

        // Skip */ characters
        self.advance();
        self.advance();

        &self.source[start_pos..self.position]
    }
}

#[cfg(test)]
mod tests {
    use super::{Keyword, Kind, Lexer};
    use crate::lexer::token::TokenValue;
    use std::rc::Rc;
    use string_cache::DefaultAtom as Atom;

    fn expect_tokens(source_code: &str, expected_tokens: &[(Kind, TokenValue)]) {
        let mut lexer = Lexer::new(source_code);

        for i in 0.. {
            let token = lexer.next_token();
            // dbg!(&token);

            if token.kind == Kind::Eof {
                assert_eq!(token.value, TokenValue::None);
                assert_eq!(i, expected_tokens.len());
                break;
            }

            assert_eq!(token.kind, expected_tokens[i].0);
            assert_eq!(token.value, expected_tokens[i].1);
        }
    }

    #[test]
    fn empty() {
        let source_code = " ";
        expect_tokens(&source_code, &vec![]);
    }

    #[test]
    fn numbers() {
        let source_code = "8 + 5 - 2 / 2";
        expect_tokens(
            &source_code,
            &vec![
                (Kind::Number, TokenValue::Number(8.0)),
                (Kind::Plus, TokenValue::None),
                (Kind::Number, TokenValue::Number(5.0)),
                (Kind::Minus, TokenValue::None),
                (Kind::Number, TokenValue::Number(2.0)),
                (Kind::Slash, TokenValue::None),
                (Kind::Number, TokenValue::Number(2.0)),
            ],
        );
    }

    #[test]
    fn let_statement() {
        let source_code = "let x = 123.0 + 456.0;";
        expect_tokens(
            &source_code,
            &vec![
                (Kind::Keyword, TokenValue::Keyword(Keyword::Let)),
                (Kind::Identifier, TokenValue::String(Atom::from("x"))),
                (Kind::Equals, TokenValue::None),
                (Kind::Number, TokenValue::Number(123.0)),
                (Kind::Plus, TokenValue::None),
                (Kind::Number, TokenValue::Number(456.0)),
                (Kind::SemiColon, TokenValue::None),
            ],
        );
    }

    #[test]
    fn function() {
        let source_code = "function sum(n1: number, n2: number): number {}";
        expect_tokens(
            &source_code,
            &vec![
                (Kind::Keyword, TokenValue::Keyword(Keyword::Function)),
                (Kind::Identifier, TokenValue::String(Atom::from("sum"))),
                (Kind::OpenParen, TokenValue::None),
                (Kind::Identifier, TokenValue::String(Atom::from("n1"))),
                (Kind::Colon, TokenValue::None),
                (Kind::Keyword, TokenValue::Keyword(Keyword::NumberType)),
                (Kind::Comma, TokenValue::None),
                (Kind::Identifier, TokenValue::String(Atom::from("n2"))),
                (Kind::Colon, TokenValue::None),
                (Kind::Keyword, TokenValue::Keyword(Keyword::NumberType)),
                (Kind::CloseParen, TokenValue::None),
                (Kind::Colon, TokenValue::None),
                (Kind::Keyword, TokenValue::Keyword(Keyword::NumberType)),
                (Kind::OpenBrace, TokenValue::None),
                (Kind::CloseBrace, TokenValue::None),
            ],
        );
    }

    #[test]
    fn string_literal() {
        let source_code = "let x = 'This is a string literal';";
        expect_tokens(
            &source_code,
            &vec![
                (Kind::Keyword, TokenValue::Keyword(Keyword::Let)),
                (Kind::Identifier, TokenValue::String(Atom::from("x"))),
                (Kind::Equals, TokenValue::None),
                (
                    Kind::String,
                    TokenValue::String(Atom::from("'This is a string literal'")),
                ),
                (Kind::SemiColon, TokenValue::None),
            ],
        );
    }

    #[test]
    fn template_string_literal() {
        let source_code = "let x = `A ${string_type} string with ${is_nested ? `${nested_level} nestings` : ''}`;";
        expect_tokens(
            &source_code,
            &vec![
                (Kind::Keyword, TokenValue::Keyword(Keyword::Let)),
                (Kind::Identifier, TokenValue::String(Atom::from("x"))),
                (Kind::Equals, TokenValue::None),
                (Kind::String, TokenValue::String(Atom::from(
                    "`A ${string_type} string with ${is_nested ? `${nested_level} nestings` : ''}`"
                ))),
                (Kind::SemiColon, TokenValue::None),
            ],
        );
    }

    #[test]
    fn if_statement_with_boolean() {
        let source_code = "if(false) {} else if(true) {} else {}";
        expect_tokens(
            &source_code,
            &vec![
                (Kind::Keyword, TokenValue::Keyword(Keyword::If)),
                (Kind::OpenParen, TokenValue::None),
                (Kind::Keyword, TokenValue::Keyword(Keyword::False)),
                (Kind::CloseParen, TokenValue::None),
                (Kind::OpenBrace, TokenValue::None),
                (Kind::CloseBrace, TokenValue::None),
                (Kind::Keyword, TokenValue::Keyword(Keyword::Else)),
                (Kind::Keyword, TokenValue::Keyword(Keyword::If)),
                (Kind::OpenParen, TokenValue::None),
                (Kind::Keyword, TokenValue::Keyword(Keyword::True)),
                (Kind::CloseParen, TokenValue::None),
                (Kind::OpenBrace, TokenValue::None),
                (Kind::CloseBrace, TokenValue::None),
                (Kind::Keyword, TokenValue::Keyword(Keyword::Else)),
                (Kind::OpenBrace, TokenValue::None),
                (Kind::CloseBrace, TokenValue::None),
            ],
        );
    }

    #[test]
    fn if_statement_with_variables() {
        let source_code = "if (this.pos.x > window.innerWidth) {}";
        expect_tokens(
            &source_code,
            &vec![
                (Kind::Keyword, TokenValue::Keyword(Keyword::If)),
                (Kind::OpenParen, TokenValue::None),
                (Kind::Keyword, TokenValue::Keyword(Keyword::This)),
                (Kind::Dot, TokenValue::None),
                (Kind::Identifier, TokenValue::String(Atom::from("pos"))),
                (Kind::Dot, TokenValue::None),
                (Kind::Identifier, TokenValue::String(Atom::from("x"))),
                (Kind::GreaterThan, TokenValue::None),
                (Kind::Identifier, TokenValue::String(Atom::from("window"))),
                (Kind::Dot, TokenValue::None),
                (
                    Kind::Identifier,
                    TokenValue::String(Atom::from("innerWidth")),
                ),
                (Kind::CloseParen, TokenValue::None),
                (Kind::OpenBrace, TokenValue::None),
                (Kind::CloseBrace, TokenValue::None),
            ],
        );
    }
}
