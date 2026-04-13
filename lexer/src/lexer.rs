use crate::{token::RegexValue, Keyword, Token, TokenKind, TokenValue};
use std::{collections::VecDeque, str::Chars};

pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
    position: usize,
    curr_char: Option<char>,
    char_queue: VecDeque<char>,
    token_queue: VecDeque<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut chars = source.chars();
        let curr_char = chars.next();

        Self {
            source,
            chars,
            position: 0,
            curr_char,
            char_queue: Default::default(),
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

    fn peek_char(&mut self, offset: usize) -> Option<char> {
        while self.char_queue.len() <= offset {
            if let Some(ch) = self.chars.next() {
                self.char_queue.push_back(ch);
            } else {
                return None;
            }
        }
        self.char_queue.get(offset).cloned()
    }

    fn advance(&mut self) {
        if let Some(ch) = self.curr_char {
            self.position += ch.len_utf8();
        }
        if self.char_queue.len() > 0 {
            self.curr_char = self.char_queue.pop_front();
        } else {
            self.curr_char = self.chars.next();
        }
    }

    fn skip_whitespace(&mut self) {
        while self.curr_char.is_some_and(|ch| ch.is_whitespace()) {
            self.advance();
        }
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
                TV::String(self.parse_string_literal(true).into()),
            ),
            '!' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        match self.curr_char {
                            Some('=') => {
                                self.advance();
                                (TK::BangEqEq, TV::None)
                            }
                            _ => (TK::BangEq, TV::None),
                        }
                    }
                    _ => (TK::Bang, TV::None),
                }
            }
            '+' => {
                self.advance();
                match self.curr_char {
                    Some('+') => {
                        self.advance();
                        (TK::PlusPlus, TV::None)
                    }
                    Some('=') => {
                        self.advance();
                        (TK::PlusEq, TV::None)
                    }
                    _ => (TK::Plus, TV::None),
                }
            }
            '-' => {
                self.advance();
                match self.curr_char {
                    Some('-') => {
                        self.advance();
                        (TK::MinusMinus, TV::None)
                    }
                    Some('=') => {
                        self.advance();
                        (TK::MinusEq, TV::None)
                    }
                    _ => (TK::Minus, TV::None),
                }
            }
            '*' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TK::StarEq, TV::None)
                    }
                    Some('*') => {
                        self.advance();
                        match self.curr_char {
                            Some('=') => {
                                self.advance();
                                (TK::StarStarEq, TV::None)
                            }
                            _ => (TK::StarStar, TV::None),
                        }
                    }
                    _ => (TK::Star, TV::None),
                }
            }
            '/' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TK::SlashEq, TV::None)
                    }
                    Some('/') => {
                        // Single-line comment
                        self.advance();
                        while self.curr_char.is_some_and(|ch| ch != '\n') {
                            self.advance();
                        }
                        (TK::SingleLineComment, TokenValue::None)
                    }
                    Some('*') => {
                        // Multi-line comments
                        let mut is_escaped = false;
                        loop {
                            match self.curr_char {
                                Some('\\') => {
                                    is_escaped = !is_escaped;
                                }
                                Some('*') => {
                                    self.advance();
                                    if self.curr_char == Some('/') {
                                        self.advance();
                                        break;
                                    }
                                }
                                Some(_) => self.advance(),
                                None => break,
                            }
                        }
                        (TokenKind::MultiLineComment, TokenValue::None)
                    }
                    None => (TK::Slash, TV::None),
                    _ => {
                        if let Some(regex_val) = self.maybe_consume_regex(start) {
                            (TK::RegexLiteral, regex_val)
                        } else {
                            self.advance();
                            (TK::Slash, TV::None)
                        }
                    }
                }
            }
            '%' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TK::PercentEq, TV::None)
                    }
                    _ => (TK::Percent, TV::None),
                }
            }
            '&' => {
                self.advance();
                match self.curr_char {
                    Some('&') => {
                        self.advance();
                        (TK::AmpAmp, TV::None)
                    }
                    _ => (TK::Amp, TV::None),
                }
            }
            '|' => {
                self.advance();
                match self.curr_char {
                    Some('|') => {
                        self.advance();
                        (TK::PipePipe, TV::None)
                    }
                    _ => (TK::Pipe, TV::None),
                }
            }
            '?' => {
                self.advance();
                match self.curr_char {
                    Some('?') => {
                        self.advance();
                        (TK::QuestionQuestion, TV::None)
                    }
                    _ => (TK::Question, TV::None),
                }
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
                (TK::Semi, TV::None)
            }
            '=' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        match self.curr_char {
                            Some('=') => {
                                self.advance();
                                (TK::EqEqEq, TV::None)
                            }
                            _ => (TK::EqEq, TV::None),
                        }
                    }
                    Some('>') => {
                        self.advance();
                        (TK::EqGt, TV::None)
                    }
                    _ => (TK::Eq, TV::None),
                }
            }
            '>' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TK::GtEq, TV::None)
                    }
                    _ => (TK::Gt, TV::None),
                }
            }
            '<' => {
                self.advance();
                match self.curr_char {
                    Some('=') => {
                        self.advance();
                        (TK::LtEq, TV::None)
                    }
                    _ => (TK::Lt, TV::None),
                }
            }
            _ => (TK::Invalid, TV::None),
        };

        if matches!(token_kind, TK::SingleLineComment | TK::MultiLineComment) {
            return self.lex_next_token();
        }

        Token {
            kind: token_kind,
            value: token_value,
            start,
            end: self.position,
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
                        || self.peek_char(0).is_none()
                        || self.peek_char(0).is_some_and(|ch| ch < '0' || ch > '9')
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
            } else if ch == '$' && self.peek_char(0) == Some('{') {
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

    fn maybe_consume_regex(&mut self, start_pos: usize) -> Option<TokenValue> {
        let mut i = 0;
        let mut is_escaped = false;
        let pattern_end: usize;

        loop {
            let Some(ch) = self.peek_char(i) else {
                return None;
            };

            match ch {
                '\n' => return None,
                '\\' => {
                    is_escaped = !is_escaped;
                }
                '/' if !is_escaped => {
                    // Enf of regex pattern
                    i += 1;
                    pattern_end = start_pos + i + 1;
                    break;
                }
                _ => {
                    is_escaped = false;
                }
            }

            i += 1;
        }

        // We found a regex pattern
        // Now we find the flags
        let flags_start = start_pos + i + 2;
        let mut flags_end: usize = flags_start;
        while let Some(ch) = self.peek_char(i) {
            match ch {
                'd' | 'g' | 'i' | 'm' | 's' | 'u' | 'v' | 'y' => {
                    i += 1;
                    flags_end += 1;
                }
                _ => break,
            }
        }

        // Consume the whole char queue
        self.position += i + 1;
        self.curr_char = self.char_queue.pop_back();
        self.char_queue.clear();

        Some(TokenValue::Regex(RegexValue {
            pattern: self.source[start_pos + 1..pattern_end].to_owned(),
            flags: self.source[flags_start..flags_end].to_owned(),
        }))
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
