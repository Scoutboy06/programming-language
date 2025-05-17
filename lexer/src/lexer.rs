use crate::{Keyword, Token, TokenKind, TokenValue};
use std::{collections::VecDeque, str::Chars};

pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
    position: usize,
    curr_char: Option<char>,
    peek_char: Option<char>,
    token_queue: VecDeque<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars();
        let curr_char = chars.next();
        let peek_char = chars.next();

        Self {
            source,
            chars,
            position: 0,
            curr_char,
            peek_char,
            token_queue: Default::default(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(token) = self.token_queue.pop_front() {
            token
        } else {
            self.lex_next_token()
        }
    }

    pub fn peek_token(&mut self) -> &Token {
        if self.token_queue.is_empty() {
            let token = self.lex_next_token();
            self.token_queue.push_back(token);
        }
        self.token_queue.front().unwrap()
    }

    pub fn peek_token_at(&mut self, offset: usize) -> &Token {
        while self.token_queue.len() <= offset {
            let token = self.lex_next_token();
            self.token_queue.push_back(token);
        }
        self.token_queue.get(offset).unwrap()
    }

    fn lex_next_token(&mut self) -> Token {
        use TokenKind as TK;
        use TokenValue as TV;
        self.skip_whitespace();

        if self.curr_char.is_none() {
            return Token::eof();
        }

        let start = self.position;

        let (token_kind, token_value) = match self.curr_char.unwrap() {
            '0'..='9' => (TK::Number, TV::Number(self.parse_number())),
            'a'..='z' | 'A'..='Z' | '_' | '$' => {
                let word = self.parse_identifier();
                if let Some(keyword) = Keyword::from_str(word) {
                    match keyword {
                        Keyword::True => (TK::Boolean, TV::Boolean(true)),
                        Keyword::False => (TK::Boolean, TV::Boolean(false)),
                        _ => (TK::Keyword, TV::Keyword(keyword)),
                    }
                } else {
                    (TK::Identifier, TV::Identifier(word.into()))
                }
            }
            '"' | '\'' | '`' => (
                TK::String,
                TV::String(self.parse_string_literal(false).into()),
            ),
            '+' => {
                self.advance();
                match self.curr_char {
                    Some('+') => {
                        self.advance();
                        (TK::Increment, TV::None)
                    }
                    Some('=') => {
                        self.advance();
                        (TK::PlusEquals, TV::None)
                    }
                    _ => (TK::Plus, TV::None),
                }
            }
            '-' => {
                self.advance();
                match self.curr_char {
                    Some('-') => {
                        self.advance();
                        (TK::Decrement, TV::None)
                    }
                    Some('=') => {
                        self.advance();
                        (TK::MinusEquals, TV::None)
                    }
                    _ => (TK::Minus, TV::None),
                }
            }
            '*' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TK::TimesEquals, TV::None)
                    }
                    Some('*') => {
                        self.advance();
                        match self.curr_char {
                            Some('*') => {
                                self.advance();
                                (TK::PowerEquals, TV::None)
                            }
                            _ => (TK::Exponentiation, TV::None),
                        }
                    }
                    _ => (TK::Asterisk, TV::None),
                }
            }
            '/' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TK::DivEquals, TV::None)
                    }
                    Some('/') => todo!("Single-line comments"),
                    Some('*') => todo!("Multi-line comments"),
                    _ => (TK::Slash, TV::None),
                }
            }
            '%' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TK::ModEquals, TV::None)
                    }
                    _ => (TK::Percent, TV::None),
                }
            }
            '&' => {
                self.advance();
                match self.curr_char {
                    Some('&') => {
                        self.advance();
                        (TK::LogicalAnd, TV::None)
                    }
                    _ => (TK::BitwiseAnd, TV::None),
                }
            }
            '|' => {
                self.advance();
                match self.curr_char {
                    Some('|') => {
                        self.advance();
                        (TK::LogicalOr, TV::None)
                    }
                    _ => (TK::BitwiseOr, TV::None),
                }
            }
            '?' => {
                self.advance();
                (TK::QuestionMark, TV::None)
            }
            '(' => {
                self.advance();
                (TK::OpenParen, TV::None)
            }
            ')' => {
                self.advance();
                (TK::CloseParen, TV::None)
            }
            '{' => {
                self.advance();
                (TK::OpenBrace, TV::None)
            }
            '}' => {
                self.advance();
                (TK::CloseBrace, TV::None)
            }
            '[' => {
                self.advance();
                (TK::OpenBracket, TV::None)
            }
            ']' => {
                self.advance();
                (TK::CloseBracket, TV::None)
            }
            '.' => {
                self.advance();
                (TK::Dot, TV::None)
            }
            ',' => {
                self.advance();
                (TK::Comma, TV::None)
            }
            ':' => {
                self.advance();
                (TK::Colon, TV::None)
            }
            ';' => {
                self.advance();
                (TK::SemiColon, TV::None)
            }
            '=' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        match self.curr_char {
                            Some('=') => {
                                self.advance();
                                (TK::TripleEquals, TV::None)
                            }
                            _ => (TK::DoubleEquals, TV::None),
                        }
                    }
                    Some('>') => {
                        self.advance();
                        (TK::ArrowFn, TV::None)
                    }
                    _ => (TK::Equals, TV::None),
                }
            }
            '>' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TK::GreaterThanOrEqual, TV::None)
                    }
                    _ => (TK::GreaterThan, TV::None),
                }
            }
            '<' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TK::LessThanOrEqual, TV::None)
                    }
                    _ => (TK::LessThan, TV::None),
                }
            }
            _ => (TK::Invalid, TV::None),
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

    fn _skip_multi_line_comment(&mut self) {
        self.advance();
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();
        if tok.is(TokenKind::Eof) {
            None
        } else {
            Some(tok)
        }
    }
}
