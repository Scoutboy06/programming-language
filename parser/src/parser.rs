// use super::{ArithmeticOperator, NullLiteral, StringLiteral};
use crate::expressions::{
    BinaryExpression, BinaryOperation, Expression, NullLiteral, NumberLiteral, StringLiteral,
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
    prev_token_end: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidToken(Token),
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: Lexer::new(source),
            current_token: Token::default(),
            prev_token_end: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut body: Vec<Statement> = Vec::new();
        let source_len = self.source.len();

        self.advance(); // Initialize first token

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
        self.current_token = self.lexer.next_token();
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
                _ => unreachable!(),
            },
            _ => Err(ParserError::InvalidToken(self.current_token.clone())),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        match self.current_token.kind {
            TokenKind::Number | TokenKind::String | TokenKind::Null => {
                let bin_exp = self.parse_binary_expression()?;
                Ok(Expression::BinaryExpression(Box::new(bin_exp)))
            }
            TokenKind::OpenBrace => todo!(),
            TokenKind::OpenBracket => todo!(),
            TokenKind::OpenParen => todo!(),
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

        let mut declarations = Vec::new();

        self.advance();

        loop {
            let start = self.current_token.start;

            self.expect_token_kind(
                TokenKind::Identifier,
                ParserError::InvalidToken(self.current_token.clone()),
            )?;
            let identifier = Identifier {
                node: Node::new(start, self.current_token.end),
                name: self.current_token.value.expect_string().clone(), // TODO: check if .clone() is cheap?
            };

            self.advance();

            if self.current_token.kind == TokenKind::Comma {
                declarations.push(VariableDeclarator {
                    node: Node::new(start, self.current_token.end),
                    id: identifier,
                    init: None,
                });

                continue;
            }

            if self.current_token.kind != TokenKind::Equals {
                break;
            }

            // TODO: support explicit types ("let a: number")

            self.advance();

            let decl = match self.current_token.kind {
                TokenKind::Number => {
                    let bin_exp = self.parse_binary_expression()?;

                    VariableDeclarator {
                        node: Node::new(start, self.current_token.end),
                        id: identifier,
                        init: Some(Expression::BinaryExpression(Box::new(bin_exp))),
                    }
                }
                TokenKind::String => todo!(),
                TokenKind::Null => todo!(),
                TokenKind::OpenBracket => todo!(),
                TokenKind::OpenBrace => todo!(),
                TokenKind::OpenParen => todo!(),
                _ => return Err(ParserError::InvalidToken(self.current_token.clone())),
            };

            declarations.push(decl);

            self.advance();

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
                TokenKind::Number => Ok(Some(NumberLiteral::as_bin_expression(
                    node,
                    left.value.expect_number(),
                ))),
                TokenKind::String => Ok(Some(StringLiteral::as_bin_expression(
                    node,
                    left.value.expect_string().clone(), // TODO: check if .clone() is cheap
                ))),
                TokenKind::Null => Ok(Some(NullLiteral::as_bin_expression(node))),
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
