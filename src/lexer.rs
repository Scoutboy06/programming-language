use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Number,
    Operator,
    Paren,
    EndOfFile,
    Invalid,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
    column: usize,
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
    curr_char: Option<char>,
    peek_char: Option<char>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer {
            chars: source.chars(),
            curr_char: None,
            peek_char: None,
            line: 1,
            column: 0,
        };

        lexer.advance();

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.advance();
        self.skip_whitespace();

        if self.curr_char.is_none() {
            return Token {
                token_type: TokenType::EndOfFile,
                value: String::new(),
                line: self.line,
                column: self.column + 1,
            };
        }

        let curr_char = self.curr_char.unwrap();

        return match curr_char {
            '0'..='9' => self.number(),
            '+' | '-' | '*' | '/' => self.operator(),
            '(' | ')' => Token {
                token_type: TokenType::Paren,
                value: curr_char.to_string(),
                line: self.line,
                column: self.column,
            },
            _ => Token {
                token_type: TokenType::Invalid,
                value: curr_char.to_string(),
                line: self.line,
                column: self.column,
            },
        };
    }

    fn advance(&mut self) {
        self.curr_char = self.peek_char;
        self.peek_char = self.chars.next();

        match self.curr_char {
            Some(ch) => {
                if ch == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
            }
            None => {}
        }
    }

    fn skip_whitespace(&mut self) {
        while self.curr_char.is_some_and(|ch| ch.is_whitespace()) {
            self.advance();
        }
    }

    fn number(&mut self) -> Token {
        let mut num_str = String::from(self.curr_char.unwrap());
        let line = self.line;
        let column = self.column;
        let mut has_decimal = false;

        while self
            .peek_char
            .is_some_and(|ch| (ch >= '0' && ch <= '9') || (ch == '.' && !has_decimal))
        {
            if self.peek_char.unwrap() == '.' {
                has_decimal = true;
            }

            num_str.push(self.peek_char.unwrap());
            self.advance();
        }

        Token {
            token_type: TokenType::Number,
            value: num_str,
            column,
            line,
        }
    }

    fn operator(&mut self) -> Token {
        let mut value = String::from(self.curr_char.unwrap());
        let column = self.column;

        if self.curr_char == Some('*') && self.peek_char == Some('*') {
            value.push('*');
            self.advance();
        }

        Token {
            token_type: TokenType::Operator,
            value,
            line: self.line,
            column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition() {
        let mut lex = Lexer::new("30 + 50");

        assert_eq!(
            lex.next_token(),
            Token {
                token_type: TokenType::Number,
                column: 1,
                line: 1,
                value: "30".to_string()
            }
        );

        assert_eq!(
            lex.next_token(),
            Token {
                token_type: TokenType::Operator,
                value: "+".to_string(),
                line: 1,
                column: 4,
            }
        );

        assert_eq!(
            lex.next_token(),
            Token {
                token_type: TokenType::Number,
                value: "50".to_string(),
                line: 1,
                column: 6,
            }
        );

        assert_eq!(
            lex.next_token(),
            Token {
                token_type: TokenType::EndOfFile,
                value: String::new(),
                line: 1,
                column: 8,
            }
        );
    }

    #[test]
    fn addition_with_paren() {
        let mut lexer = Lexer::new("3 + (5 + 8)");

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                column: 1,
                line: 1,
                value: '3'.to_string()
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator,
                value: '+'.to_string(),
                line: 1,
                column: 3,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren,
                value: '('.to_string(),
                line: 1,
                column: 5,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                value: '5'.to_string(),
                line: 1,
                column: 6,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator,
                value: '+'.to_string(),
                line: 1,
                column: 8,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                value: '8'.to_string(),
                line: 1,
                column: 10,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren,
                value: ')'.to_string(),
                line: 1,
                column: 11,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::EndOfFile,
                value: String::new(),
                line: 1,
                column: 12
            }
        );
    }

    #[test]
    fn subtraction_with_paren() {
        let mut lexer = Lexer::new("3 - (5 - 8)");

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                column: 1,
                line: 1,
                value: '3'.to_string()
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator,
                value: '-'.to_string(),
                line: 1,
                column: 3,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren,
                value: '('.to_string(),
                line: 1,
                column: 5,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                value: '5'.to_string(),
                line: 1,
                column: 6,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator,
                value: '-'.to_string(),
                line: 1,
                column: 8,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                value: '8'.to_string(),
                line: 1,
                column: 10,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren,
                value: ')'.to_string(),
                line: 1,
                column: 11,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::EndOfFile,
                value: String::new(),
                line: 1,
                column: 12
            }
        );
    }

    #[test]
    fn division_with_paren() {
        let mut lexer = Lexer::new("3 / (5 / 8)");

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                column: 1,
                line: 1,
                value: '3'.to_string()
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator,
                value: '/'.to_string(),
                line: 1,
                column: 3,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren,
                value: '('.to_string(),
                line: 1,
                column: 5,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                value: '5'.to_string(),
                line: 1,
                column: 6,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator,
                value: '/'.to_string(),
                line: 1,
                column: 8,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                value: '8'.to_string(),
                line: 1,
                column: 10,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren,
                value: ')'.to_string(),
                line: 1,
                column: 11,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::EndOfFile,
                value: String::new(),
                line: 1,
                column: 12
            }
        );
    }

    #[test]
    fn multiplication_with_paren() {
        let mut lexer = Lexer::new("3 * (5 * 8)");

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                column: 1,
                line: 1,
                value: '3'.to_string()
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator,
                value: '*'.to_string(),
                line: 1,
                column: 3,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren,
                value: '('.to_string(),
                line: 1,
                column: 5,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                value: '5'.to_string(),
                line: 1,
                column: 6,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator,
                value: '*'.to_string(),
                line: 1,
                column: 8,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                value: '8'.to_string(),
                line: 1,
                column: 10,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren,
                value: ')'.to_string(),
                line: 1,
                column: 11,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::EndOfFile,
                value: String::new(),
                line: 1,
                column: 12
            }
        );
    }

    #[test]
    fn exponent_with_paren() {
        let mut lexer = Lexer::new("3 ** (5 ** 8)");

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                column: 1,
                line: 1,
                value: '3'.to_string()
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator,
                value: "**".to_string(),
                line: 1,
                column: 3,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren,
                value: '('.to_string(),
                line: 1,
                column: 6,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                value: '5'.to_string(),
                line: 1,
                column: 7,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Operator,
                value: "**".to_string(),
                line: 1,
                column: 9,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Number,
                value: '8'.to_string(),
                line: 1,
                column: 12,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::Paren,
                value: ')'.to_string(),
                line: 1,
                column: 13,
            }
        );

        assert_eq!(
            lexer.next_token(),
            Token {
                token_type: TokenType::EndOfFile,
                value: String::new(),
                line: 1,
                column: 14
            }
        );
    }
}
