use crate::{Keyword, Token, TokenKind, TokenValue};
use std::str::Chars;
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
                kind: TokenKind::Eof,
                value: TokenValue::None,
                start: self.position,
                end: self.position,
            };
        }

        let start = self.position;

        let (token_kind, token_value) = match self.curr_char.unwrap() {
            '0'..='9' => (TokenKind::Number, TokenValue::Number(self.parse_number())),
            'a'..='z' | 'A'..='Z' | '_' | '$' => {
                let word = self.parse_identifier();
                if let Some(keyword) = Keyword::from_str(word) {
                    match keyword {
                        Keyword::True => (TokenKind::Boolean, TokenValue::Boolean(true)),
                        Keyword::False => (TokenKind::Boolean, TokenValue::Boolean(false)),
                        _ => (TokenKind::Keyword, TokenValue::Keyword(keyword)),
                    }
                } else {
                    (TokenKind::Identifier, TokenValue::String(Atom::from(word)))
                }
            }
            '"' | '\'' | '`' => (
                TokenKind::String,
                TokenValue::String(Atom::from(self.parse_string_literal(true))), // We strip out quotes
            ),
            '+' => {
                self.advance();
                match self.curr_char {
                    Some('+') => {
                        self.advance();
                        (TokenKind::Increment, TokenValue::None)
                    }
                    Some('=') => {
                        self.advance();
                        (TokenKind::PlusEquals, TokenValue::None)
                    }
                    _ => (TokenKind::Plus, TokenValue::None),
                }
            }
            '-' => {
                self.advance();
                match self.curr_char {
                    Some('-') => {
                        self.advance();
                        (TokenKind::Decrement, TokenValue::None)
                    }
                    Some('=') => {
                        self.advance();
                        (TokenKind::MinusEquals, TokenValue::None)
                    }
                    _ => (TokenKind::Minus, TokenValue::None),
                }
            }
            '*' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TokenKind::TimesEquals, TokenValue::None)
                    }
                    Some('*') => {
                        self.advance();
                        match self.curr_char {
                            Some('*') => {
                                self.advance();
                                (TokenKind::PowerEquals, TokenValue::None)
                            }
                            _ => (TokenKind::Exponentiation, TokenValue::None),
                        }
                    }
                    _ => (TokenKind::Asterisk, TokenValue::None),
                }
            }
            '/' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TokenKind::DivEquals, TokenValue::None)
                    }
                    Some('/') => todo!("Single-line comments"),
                    Some('*') => todo!("Multi-line comments"),
                    _ => (TokenKind::Slash, TokenValue::None),
                }
            }
            '%' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TokenKind::ModEquals, TokenValue::None)
                    }
                    _ => (TokenKind::Modulus, TokenValue::None),
                }
            }
            '&' => {
                self.advance();
                match self.curr_char {
                    Some('&') => {
                        self.advance();
                        (TokenKind::LogicalAnd, TokenValue::None)
                    }
                    _ => (TokenKind::BitwiseAnd, TokenValue::None),
                }
            }
            '|' => {
                self.advance();
                match self.curr_char {
                    Some('|') => {
                        self.advance();
                        (TokenKind::LogicalOr, TokenValue::None)
                    }
                    _ => (TokenKind::BitwiseOr, TokenValue::None),
                }
            }
            '?' => {
                self.advance();
                (TokenKind::Ternary, TokenValue::None)
            }
            '(' => {
                self.advance();
                (TokenKind::OpenParen, TokenValue::None)
            }
            ')' => {
                self.advance();
                (TokenKind::CloseParen, TokenValue::None)
            }
            '{' => {
                self.advance();
                (TokenKind::OpenBrace, TokenValue::None)
            }
            '}' => {
                self.advance();
                (TokenKind::CloseBrace, TokenValue::None)
            }
            '[' => {
                self.advance();
                (TokenKind::OpenBracket, TokenValue::None)
            }
            ']' => {
                self.advance();
                (TokenKind::CloseBracket, TokenValue::None)
            }
            '.' => {
                self.advance();
                (TokenKind::Dot, TokenValue::None)
            }
            ',' => {
                self.advance();
                (TokenKind::Comma, TokenValue::None)
            }
            ':' => {
                self.advance();
                (TokenKind::Colon, TokenValue::None)
            }
            ';' => {
                self.advance();
                (TokenKind::SemiColon, TokenValue::None)
            }
            '=' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        match self.curr_char {
                            Some('=') => {
                                self.advance();
                                (TokenKind::TripleEquals, TokenValue::None)
                            }
                            _ => (TokenKind::DoubleEquals, TokenValue::None),
                        }
                    }
                    Some('>') => {
                        self.advance();
                        (TokenKind::ArrowFn, TokenValue::None)
                    }
                    _ => (TokenKind::Equals, TokenValue::None),
                }
            }
            '>' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TokenKind::GreaterThanOrEqual, TokenValue::None)
                    }
                    _ => (TokenKind::GreaterThan, TokenValue::None),
                }
            }
            '<' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TokenKind::LessThanOrEqual, TokenValue::None)
                    }
                    _ => (TokenKind::LessThan, TokenValue::None),
                }
            }
            _ => (TokenKind::Invalid, TokenValue::None),
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

    fn parse_string_literal(&mut self, strip_quotes: bool) -> &str {
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

        if strip_quotes {
            &self.source[start_pos + 1..self.position - 1]
        } else {
            &self.source[start_pos..self.position]
        }
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

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();
        if tok.kind == TokenKind::Eof {
            None
        } else {
            Some(tok)
        }
    }
}
