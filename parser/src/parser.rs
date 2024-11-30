// use super::{ArithmeticOperator, NullLiteral, StringLiteral};
use crate::expressions::{
    BinaryExpression, BinaryOperation, BooleanLiteral, Expression, Literal, NullLiteral,
    NumberLiteral, StringLiteral,
};
use crate::nodes::program::Program;
use crate::nodes::Node;
use crate::statements::{
    FunctionDeclaration, Identifier, Shebang, Statement, VariableDeclaration, VariableDeclarator,
    VariableKind,
};
use lexer::{Keyword, Lexer, Token, TokenKind};
use string_cache::DefaultAtom as Atom;

pub struct Parser<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    prev_token_end: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidToken(Token),
    ExpectedClosingParen(Token),
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: Lexer::new(source),
            current_token: Token::default(),
            peek_token: Token::default(),
            prev_token_end: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut body: Vec<Statement> = Vec::new();
        let source_len = self.source.len();

        // Initialize tokens
        self.advance();
        self.advance();

        loop {
            if self.current_token.kind == TokenKind::Eof {
                break;
            }

            let statement: Statement = self.parse_statement()?;
            body.push(statement);
        }

        Ok(Program {
            node: Node::new(0, source_len),
            shebang: None,
            body,
        })
    }

    fn advance(&mut self) {
        self.prev_token_end = self.current_token.end;

        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_token_kind(&self, kind: TokenKind, error: ParserError) -> Result<(), ParserError> {
        if self.current_token.kind != kind {
            Err(error)
        } else {
            Ok(())
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token.kind {
            TokenKind::Keyword => match self.current_token.value.expect_keyword() {
                Keyword::Var | Keyword::Let | Keyword::Const => {
                    let var_dec = self.parse_variable_declaration()?;
                    Ok(Statement::VariableDeclaration(var_dec.into()))
                }
                Keyword::Function => {
                    let fn_dec = self.parse_function_declaration()?;
                    Ok(Statement::FunctionDeclaration(fn_dec.into()))
                }
                _ => todo!(),
            },
            _ => Err(ParserError::InvalidToken(self.current_token.clone())),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        match self.current_token.kind {
            TokenKind::String | TokenKind::Boolean | TokenKind::Number | TokenKind::Null => {
                if self.peek_token.kind.is_operator() {
                    let bin_exp = self.parse_binary_expression()?;
                    return Ok(Expression::BinaryExpression(bin_exp.into()));
                }

                // Literal

                let node = Node::new(self.current_token.start, self.current_token.end);

                let expr: Expression = match self.current_token.kind {
                    TokenKind::String => StringLiteral {
                        node,
                        value: self.current_token.value.expect_string().clone(),
                    }
                    .into(),
                    TokenKind::Boolean => BooleanLiteral {
                        node,
                        value: self.current_token.value.expect_boolean(),
                    }
                    .into(),
                    TokenKind::Number => NumberLiteral {
                        node,
                        value: self.current_token.value.expect_number(),
                    }
                    .into(),
                    _ => return Err(ParserError::InvalidToken(self.current_token.clone())),
                };

                self.advance(); // Consume Literal token

                Ok(expr)
            }
            TokenKind::OpenParen => {
                self.advance(); // Consume OpenParen token
                let expr = self.parse_expression()?;
                match self.current_token.kind {
                    TokenKind::Comma => todo!("Arrow function"),
                    TokenKind::CloseParen => {
                        self.advance();
                        Ok(expr)
                    }
                    _ => Err(ParserError::ExpectedClosingParen(
                        self.current_token.clone(),
                    )),
                }
            }
            TokenKind::OpenBracket => todo!(),
            TokenKind::OpenBrace => todo!(),
            _ => Err(ParserError::InvalidToken(self.current_token.clone())),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration, ParserError> {
        let start_pos = self.current_token.start;

        let kind = match self.current_token.value.expect_keyword() {
            Keyword::Var => VariableKind::Var,
            Keyword::Let => VariableKind::Let,
            Keyword::Const => VariableKind::Const,
            _ => unreachable!(),
        };
        self.advance();

        let mut declarations = Vec::new();

        loop {
            let start = self.current_token.start;

            self.expect_token_kind(
                TokenKind::Identifier,
                ParserError::InvalidToken(self.current_token.clone()),
            )?;
            let identifier = Identifier {
                node: Node::new(start, self.current_token.end),
                name: self.current_token.value.expect_string().clone(),
            };
            self.advance();

            match self.current_token.kind {
                TokenKind::Comma => {
                    declarations.push(VariableDeclarator {
                        node: Node::new(start, self.current_token.end),
                        id: identifier,
                        init: None,
                    });
                    self.advance();
                    continue;
                }
                TokenKind::Colon => todo!(),
                TokenKind::Equals => self.advance(),
                _ => break,
            }

            let expr = self.parse_expression()?;
            let decl = VariableDeclarator {
                node: Node::new(start, self.current_token.end),
                id: identifier,
                init: Some(expr),
            };

            declarations.push(decl);

            if self.current_token.kind != TokenKind::Comma {
                break;
            }
        }

        Ok(VariableDeclaration {
            node: Node::new(start_pos, self.prev_token_end),
            kind,
            declarations,
        })
    }

    fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration, ParserError> {
        todo!()
    }

    fn parse_binary_expression(&mut self) -> Result<BinaryExpression, ParserError> {
        // Handles literals, identifiers, and parenthesized expressions
        fn parse_lit(parser: &mut Parser) -> Result<Option<BinaryExpression>, ParserError> {
            let left = parser.current_token.clone();
            parser.advance();

            let node = Node::new(left.start, left.end);

            match left.kind {
                TokenKind::Number => Ok(Some(
                    NumberLiteral {
                        node,
                        value: left.value.expect_number(),
                    }
                    .into(),
                )),
                TokenKind::String => Ok(Some(
                    StringLiteral {
                        node,
                        value: left.value.expect_string().clone(),
                    }
                    .into(),
                )),
                TokenKind::Null => Ok(Some(NullLiteral { node }.into())),
                TokenKind::Identifier => Ok(Some(BinaryExpression::Identifier(Identifier {
                    node: Node::new(left.start, left.end),
                    name: Atom::from(left.value.expect_string().clone()), // TODO: check if .clone() is cheap
                }))),
                TokenKind::OpenParen => {
                    let factor = parse_factor(parser)?;
                    parser.expect_token_kind(
                        TokenKind::CloseParen,
                        ParserError::InvalidToken(parser.current_token.clone()),
                    )?;
                    parser.advance();
                    Ok(factor)
                }
                TokenKind::OpenBrace => todo!(),
                TokenKind::OpenBracket => todo!(),
                TokenKind::Eof => Ok(None),
                _ => Err(ParserError::InvalidToken(parser.current_token.clone())),
            }
        }

        // Handles factor-level operations (like * and /)
        fn parse_factor(parser: &mut Parser) -> Result<Option<BinaryExpression>, ParserError> {
            let start_pos = parser.current_token.start;
            let mut left = parse_lit(parser)?.unwrap();

            while let Some(operator) = parser.current_token.kind.as_factor_operator() {
                parser.advance();

                let right = parse_factor(parser)?;

                if right.is_none() {
                    break;
                }

                left = BinaryExpression::BinaryOperation(BinaryOperation {
                    node: Node::new(start_pos, parser.prev_token_end),
                    operator,
                    left: Box::new(left),
                    right: Box::new(right.unwrap()),
                });
            }

            Ok(Some(left))
        }

        let start_pos = self.current_token.start;
        let mut left = parse_factor(self)?.unwrap();

        // Handle term-level operators (like + and -)
        while let Some(operator) = self.current_token.kind.as_term_operator() {
            self.advance();

            let right = parse_factor(self)?;

            if right.is_none() {
                break;
            }

            left = BinaryExpression::BinaryOperation(BinaryOperation {
                node: Node::new(start_pos, self.prev_token_end),
                operator,
                left: Box::new(left),
                right: Box::new(right.unwrap()),
            });
        }

        Ok(left)
    }
}
