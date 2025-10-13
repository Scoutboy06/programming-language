use crate::throw_error;

use crate::expressions::types::{
    ArrayType, AstType, KeywordType, TypeAnnotation, TypeParameter, TypeParameterDeclaration,
    TypeReference,
};
use crate::expressions::{
    ArrayExpression, ArrowFunctionExpression, AssignmentExpression, BinaryExpression,
    BooleanLiteral, CallExpression, Expression, FunctionExpression, Identifier, Literal,
    MemberExpression, NewExpression, NullLiteral, NumberLiteral, ObjectExpression,
    ParenthesisExpression, RegexLiteral, StringLiteral, TernaryExpression, ThisExpression,
    TypeofExpression, UnaryExpression, UpdateExpression, VariableKind,
};
use crate::nodes::{program::Program, Node};
use crate::statements::{
    BlockStatement, BreakStatement, ContinueStatement, EnumMember, EnumStatement,
    ExpressionStatement, ForStatement, FunctionDeclaration, IfStatement, Parameter,
    ReturnStatement, Statement, ThrowStatement, VariableDeclaration, VariableDeclarator,
    WhileStatement,
};
use crate::utils::parser_error::{ParserError, ParserErrorInfo};
use lexer::{AssignmentOperator, Keyword, Lexer, Operator, Token, TokenKind};

pub struct Parser<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: Lexer::new(source),
            current_token: Token::default(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut body: Vec<Statement> = Vec::new();
        let source_len = self.source.len();

        // Initialize tokens
        self.advance();

        loop {
            if self.current_token.is(TokenKind::Eof) {
                break;
            }

            let statement = self.parse_statement(true);

            match statement {
                Ok(s) => body.push(s),
                Err(err) => {
                    return Err(ParserError {
                        id: err.id,
                        kind: err.kind,
                        token: self.current_token.clone(),
                    })
                }
            }
        }

        Ok(Program {
            node: Node::new(0, source_len),
            body,
        })
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect_token_kind(&self, kind: TokenKind) -> Result<(), ParserErrorInfo> {
        if self.current_token.kind == kind {
            Ok(())
        } else {
            throw_error!(InvalidToken);
        }
    }

    fn expect_and_consume_token(&mut self, kind: TokenKind) -> Result<(), ParserErrorInfo> {
        if self.current_token.kind == kind {
            self.advance();
            Ok(())
        } else {
            throw_error!(InvalidToken);
        }
    }

    /// Parses a single statement (e.g., variable declarations, control flow statements, function definitions).
    fn parse_statement(&mut self, include_basic_semi: bool) -> Result<Statement, ParserErrorInfo> {
        match self.current_token.kind {
            TokenKind::Keyword => match self.current_token.value.expect_keyword() {
                Keyword::Var | Keyword::Let | Keyword::Const => {
                    if self.current_token.value.expect_keyword() == Keyword::Const {
                        let peek = self.lexer.peek_token();
                        if peek.is(TokenKind::Keyword)
                            && peek.value.expect_keyword() == Keyword::Enum
                        {
                            return Ok(self.parse_enum_declaration(true, false)?.into());
                        }
                    }
                    Ok(self.parse_variable_declaration(include_basic_semi)?.into())
                }
                Keyword::Function => Ok(self.parse_function_declaration(false)?.into()),
                Keyword::Return => Ok(self.parse_return_statement()?.into()),
                Keyword::If => Ok(self.parse_if_statement()?.into()),
                Keyword::While => Ok(self.parse_while_statement()?.into()),
                Keyword::For => Ok(self.parse_for_statement()?.into()),
                Keyword::Enum => Ok(self.parse_enum_declaration(false, false)?.into()),
                Keyword::Declare => Ok(self.parse_enum_declaration(false, true)?.into()),
                Keyword::Typeof => {
                    let expr = self.parse_expression()?;
                    Ok(ExpressionStatement {
                        node: *expr.node(),
                        expression: expr,
                    }
                    .into())
                }
                Keyword::Throw => {
                    let start_pos = self.current_token.start;
                    self.advance(); // Consume "throw" token
                    let expr = self.parse_expression()?;
                    if self.current_token.is(TokenKind::SemiColon) {
                        self.advance(); // Consume ";" token
                    }
                    Ok(ThrowStatement {
                        node: Node::new(start_pos, expr.node().end),
                        expr,
                    }
                    .into())
                }
                Keyword::New | Keyword::This => {
                    let expr = self.parse_expression()?;
                    Ok(ExpressionStatement {
                        node: *expr.node(),
                        expression: expr,
                    }
                    .into())
                }
                Keyword::Continue => {
                    let start_pos = self.current_token.start;
                    let mut end_pos = self.current_token.end;
                    self.advance(); // Consume "continue" token;

                    let id = if self.current_token.is(TokenKind::Identifier) {
                        end_pos = self.current_token.end;
                        Some(Identifier {
                            node: Node::new(self.current_token.start, self.current_token.end),
                            name: self.current_token.value.expect_identifier().to_owned(),
                        })
                    } else {
                        None
                    };

                    if include_basic_semi && self.current_token.is(TokenKind::SemiColon) {
                        end_pos = self.current_token.end;
                        self.advance(); // Consume ';' token
                    }

                    Ok(ContinueStatement {
                        node: Node::new(start_pos, end_pos),
                        id,
                    }
                    .into())
                }
                Keyword::Break => {
                    let start_pos = self.current_token.start;
                    let mut end_pos = self.current_token.end;
                    self.advance(); // Consume "break" token

                    let id = if self.current_token.is(TokenKind::Identifier) {
                        end_pos = self.current_token.end;
                        Some(Identifier {
                            node: Node::new(self.current_token.start, self.current_token.end),
                            name: self.current_token.value.expect_identifier().to_owned(),
                        })
                    } else {
                        None
                    };

                    if include_basic_semi && self.current_token.is(TokenKind::SemiColon) {
                        end_pos = self.current_token.end;
                        self.advance(); // Consume ';' token
                    }

                    Ok(BreakStatement {
                        node: Node::new(start_pos, end_pos),
                        id,
                    }
                    .into())
                }
                _ => {
                    // throw_error!(InvalidToken);
                    use crate::utils::parser_error::{ErrorKind, ParserErrorInfo};
                    let err = ParserErrorInfo {
                        kind: ErrorKind::InvalidToken,
                        #[cfg(debug_assertions)]
                        id: concat!(file!(), ":", line!()).to_owned(),
                    };
                    return Err(err);
                }
            },
            TokenKind::OpenBrace => Ok(self.parse_block_statement()?.into()),
            _ => {
                let expr = self.parse_expression()?;
                let end_pos = if self.current_token.is(TokenKind::SemiColon) && include_basic_semi {
                    let pos = self.current_token.end;
                    self.advance();
                    pos
                } else {
                    expr.node().end
                };

                Ok(ExpressionStatement {
                    node: Node::new(expr.node().start, end_pos),
                    expression: expr,
                }
                .into())
            }
        }
    }

    /// Parses an expression (e.g., arithmetic operations, logical operations, or function calls).
    fn parse_expression(&mut self) -> Result<Expression, ParserErrorInfo> {
        let mut lhs = self.parse_primary_expression()?;

        loop {
            match self.current_token.kind {
                TokenKind::OpenParen => {
                    let call_exp = self.parse_call_expression(lhs)?;
                    lhs = Expression::CallExpression(Box::new(call_exp));
                }
                TokenKind::Dot | TokenKind::OpenBracket => {
                    let mem_exp = self.parse_member_expression(lhs)?;
                    lhs = Expression::MemberExpression(Box::new(mem_exp));
                }
                TokenKind::Increment | TokenKind::Decrement => {
                    let op = self
                        .current_token
                        .kind
                        .as_operator()
                        .unwrap() // Safe unwrap because of match statement
                        .as_update_operator()
                        .unwrap(); // Safe unwrap because of match statement
                    let expr = UpdateExpression {
                        node: Node::new(lhs.node().start, self.current_token.end),
                        argument: lhs,
                        operator: op,
                        prefix: false,
                    };
                    self.advance(); // Consume Update operator token
                    lhs = expr.into();
                }
                TokenKind::QuestionMark => {
                    self.advance();
                    let truthy_expr = self.parse_expression()?;
                    self.expect_and_consume_token(TokenKind::Colon)?;
                    let falsy_expr = self.parse_expression()?;
                    lhs = TernaryExpression {
                        node: Node::new(lhs.node().start, falsy_expr.node().end),
                        truthy_expr: Box::new(truthy_expr),
                        falsy_expr: Box::new(falsy_expr),
                    }
                    .into();
                    break;
                }
                // TokenKind::SemiColon => {
                //     self.advance(); // Comsume ';' token
                //     break;
                // }
                _ if self.current_token.kind.is_assignment_operator() => {
                    let expr = self.parse_assignment_expression(lhs)?;
                    lhs = Expression::AssignmentExpression(Box::new(expr));
                }
                _ if self.current_token.kind.is_operator() => {
                    let bin_exp = self.parse_binary_expression(Some(lhs), 0)?;
                    lhs = Expression::BinaryExpression(Box::new(bin_exp));
                }
                _ => break,
            }
        }

        Ok(lhs)
    }

    /// Parses literal values, such as numbers, strings, booleans, null, arrays, objects, member expressions, and parenthesised expressions
    fn parse_primary_expression(&mut self) -> Result<Expression, ParserErrorInfo> {
        match self.current_token.kind {
            TokenKind::String | TokenKind::Boolean | TokenKind::Number | TokenKind::Null => {
                Ok(self.parse_literal()?.into())
            }
            TokenKind::Identifier => {
                let identifier = Identifier {
                    node: Node::new(self.current_token.start, self.current_token.end),
                    name: self.current_token.value.expect_identifier().clone(),
                };
                self.advance(); // Consume Identifier token

                if self.current_token.is(TokenKind::Dot) {
                    Ok(self.parse_member_expression(identifier.into())?.into())
                } else {
                    Ok(identifier.into())
                }
            }
            TokenKind::OpenParen => {
                // Peek forward to determine if is an arrow function
                // Only consumes tokens if it is an arrow function
                if self.peek_is_arrow_function() {
                    let arr_fn_expr = self.parse_arrow_function()?;
                    return Ok(Expression::ArrowFunctionExpression(Box::new(arr_fn_expr)));
                }

                let start_pos = self.current_token.start;
                self.advance(); // Consume "(" token

                let expression = self.parse_expression()?;

                let paren_expr = ParenthesisExpression {
                    node: Node::new(start_pos, self.current_token.end),
                    expression,
                };

                self.expect_and_consume_token(TokenKind::CloseParen)?;

                Ok(paren_expr.into())
            }
            TokenKind::OpenBracket => Ok(self.parse_array_literal()?.into()),
            TokenKind::OpenBrace => Ok(self.parse_object_literal()?.into()),
            TokenKind::Exclamation | TokenKind::Plus | TokenKind::Minus => {
                let unary_kind = match self.current_token.kind {
                    TokenKind::Exclamation => UnaryKind::Not,
                    TokenKind::Plus => UnaryKind::Plus,
                    TokenKind::Minus => UnaryKind::Minus,
                    _ => unreachable!(),
                };
                let start_pos = self.current_token.start;
                self.advance(); // Consume unary token
                let expr = self.parse_expression()?;
                Ok(UnaryExpression {
                    node: Node::new(start_pos, expr.node().end),
                    kind: unary_kind,
                    expression: Box::new(expr),
                }
                .into())
            }
            TokenKind::Keyword => match self.current_token.value.expect_keyword() {
                Keyword::Function => Ok(self.parse_function_expression(false)?.into()),
                Keyword::Typeof => Ok(self.parse_typeof_expression()?.into()),
                Keyword::New => Ok(self.parse_new_expression()?.into()),
                Keyword::This => {
                    let expr = ThisExpression {
                        node: Node::new(self.current_token.start, self.current_token.end),
                    };
                    self.advance(); // Consume "this" token
                    Ok(expr.into())
                }
                _ => throw_error!(InvalidToken),
            },
            TokenKind::RegexLiteral => {
                let node = Node::new(self.current_token.start, self.current_token.end);
                let value = self.current_token.value.consume_regex();
                self.advance(); // Consume Regex token
                Ok(Literal::RegexLiteral(RegexLiteral { node, value }).into())
            }
            _ => throw_error!(InvalidToken),
        }
    }

    /// Peeks forward in the token stream
    fn peek_is_arrow_function(&mut self) -> bool {
        match self.lexer.peek_token_at(0).kind {
            TokenKind::Identifier => {}
            TokenKind::CloseParen => return true,
            _ => return false,
        }
        match self.lexer.peek_token_at(1).kind {
            TokenKind::Colon | TokenKind::Comma => true,
            TokenKind::CloseParen => self.lexer.peek_token_at(2).is(TokenKind::ArrowFn),
            _ => false,
        }
    }

    /// Parses a block of code, usually enclosed by `{}`.
    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.expect_and_consume_token(TokenKind::OpenBrace)?;

        let mut statements: Vec<Statement> = Vec::new();

        while self.current_token.kind != TokenKind::CloseBrace {
            let stmt = self.parse_statement(true)?;
            statements.push(stmt);
        }

        let block = BlockStatement {
            node: Node::new(start_pos, self.current_token.end),
            statements,
        };

        self.advance(); // Consume "}" token

        Ok(block)
    }

    /// Parses a variable declaration, including `let`, `const`, or `var` keywords.
    fn parse_variable_declaration(
        &mut self,
        include_semi: bool,
    ) -> Result<VariableDeclaration, ParserErrorInfo> {
        let start_pos = self.current_token.start;

        let kind = match self.current_token.value.expect_keyword() {
            Keyword::Var => VariableKind::Var,
            Keyword::Let => VariableKind::Let,
            Keyword::Const => VariableKind::Const,
            _ => unreachable!(),
        };
        self.advance();

        let mut declarations = Vec::new();
        let mut end_pos;

        loop {
            let start = self.current_token.start;
            end_pos = self.current_token.end;

            self.expect_token_kind(TokenKind::Identifier)?;
            let identifier = Identifier {
                node: Node::new(start, self.current_token.end),
                name: self.current_token.value.expect_identifier().clone(),
            };
            self.advance(); // Consume Identifier token

            let type_annotation = if self.current_token.is(TokenKind::Colon) {
                let ann = self.parse_type_annotation()?;
                end_pos = ann.node.end;
                Some(ann)
            } else {
                None
            };

            let init = if self.current_token.is(TokenKind::Equals) {
                self.advance(); // Consume "=" token
                let expr = self.parse_expression()?;
                end_pos = expr.node().end;
                Some(expr)
            } else {
                None
            };

            let decl = VariableDeclarator {
                node: Node::new(start, end_pos),
                id: identifier,
                type_annotation,
                init,
            };

            declarations.push(decl);

            if self.current_token.kind != TokenKind::Comma {
                break;
            }
            self.advance(); // Consume "," token
        }

        if include_semi && self.current_token.is(TokenKind::SemiColon) {
            end_pos = self.current_token.end;
            self.advance() // Consume ";" token
        }

        Ok(VariableDeclaration {
            node: Node::new(start_pos, end_pos),
            kind,
            declarations,
        })
    }

    /// Parses a function declaration, including its name, parameters, and body.
    fn parse_function_declaration(
        &mut self,
        is_async: bool,
    ) -> Result<FunctionDeclaration, ParserErrorInfo> {
        let start_pos = self.current_token.start;

        self.advance(); // Consume "function" token

        self.expect_token_kind(TokenKind::Identifier)?;
        let id = Identifier {
            node: Node::new(self.current_token.start, self.current_token.end),
            name: self.current_token.value.expect_identifier().clone(),
        };
        self.advance(); // Consume Identifier token

        let type_parameters = if self.current_token.is(TokenKind::LessThan) {
            Some(self.parse_type_parameter_declaration()?)
        } else {
            None
        };

        let is_generator = self.current_token.is(TokenKind::Asterisk);
        if is_generator {
            self.advance(); // Consume "*" token
        }

        self.expect_token_kind(TokenKind::OpenParen)?;

        let params = self.parse_parameter_list()?;

        let return_type = if self.current_token.is(TokenKind::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        let body = self.parse_block_statement()?;

        Ok(FunctionDeclaration {
            node: Node::new(start_pos, body.node.end),
            id,
            type_parameters,
            params,
            return_type,
            is_generator,
            is_async,
            body,
        })
    }

    /// Parses a function declaration, including its name, parameters, and body.
    fn parse_function_expression(
        &mut self,
        is_async: bool,
    ) -> Result<FunctionExpression, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        let mut is_generator = false;
        let mut return_type: Option<TypeAnnotation> = None;
        let mut id: Option<Identifier> = None;

        self.advance(); // Consume "function" keyword token

        if self.current_token.is(TokenKind::Asterisk) {
            is_generator = true;
            self.advance(); // Consume "*" token
        }

        if self.current_token.is(TokenKind::Identifier) {
            id = Some(Identifier {
                node: Node::new(self.current_token.start, self.current_token.end),
                name: self.current_token.value.expect_identifier().clone(),
            });
            self.advance(); // Consume Identifier token
        }

        self.expect_token_kind(TokenKind::OpenParen)?;

        let params = self.parse_parameter_list()?;

        // Explicit return type, like "function a(): number {}"
        if self.current_token.is(TokenKind::Colon) {
            return_type = Some(self.parse_type_annotation()?);
        }

        let body = self.parse_block_statement()?;

        Ok(FunctionExpression {
            node: Node::new(start_pos, body.node.end),
            id,
            params,
            return_type,
            body,
            is_generator,
            is_async,
        })
    }

    fn parse_method(&mut self) -> Result<Method, ParserErrorInfo> {
        let start_pos = self.current_token.start;

        let is_async = if self.current_token.is(TokenKind::Keyword)
            && self.current_token.value.expect_keyword() == Keyword::Async
        {
            self.advance(); // Consume "async" keyword
            true
        } else {
            false
        };

        let is_generator = if self.current_token.is(TokenKind::Asterisk) {
            self.advance(); // Consume "*" token
            true
        } else {
            false
        };

        self.expect_token_kind(TokenKind::Identifier)?;
        let id = Identifier {
            node: Node::new(self.current_token.start, self.current_token.end),
            name: self.current_token.value.expect_identifier().clone(),
        };
        self.advance(); // Consume Identifier token

        let type_parameters = if self.current_token.is(TokenKind::LessThan) {
            Some(self.parse_type_parameter_declaration()?)
        } else {
            None
        };

        let parameters = self.parse_parameter_list()?;

        let return_type = if self.current_token.is(TokenKind::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        let body = self.parse_block_statement()?;

        Ok(Method {
            node: Node::new(start_pos, body.node.end),
            is_async,
            is_generator,
            id,
            type_parameters,
            parameters,
            return_type,
            body,
        })
    }

    fn parse_arrow_function(&mut self) -> Result<ArrowFunctionExpression, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        let parameters = self.parse_parameter_list()?;

        let return_type = if self.current_token.is(TokenKind::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        self.expect_and_consume_token(TokenKind::ArrowFn)?;
        let body = self.parse_statement(false)?;

        Ok(ArrowFunctionExpression {
            node: Node::new(start_pos, body.node().end),
            parameters,
            return_type,
            body,
        })
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>, ParserErrorInfo> {
        self.expect_and_consume_token(TokenKind::OpenParen)?;

        let mut params: Vec<Parameter> = Vec::new();

        while self.current_token.kind != TokenKind::CloseParen {
            let start_pos = self.current_token.start;
            self.expect_token_kind(TokenKind::Identifier)?;
            let id = self.current_token.value.expect_identifier().clone();
            let identifier = Identifier {
                node: Node::new(self.current_token.start, self.current_token.end),
                name: id,
            };
            let mut end_pos = self.current_token.end;
            self.advance(); // Consume Identifier token

            let optional = if self.current_token.is(TokenKind::QuestionMark) {
                end_pos = self.current_token.end;
                self.advance(); // Consume "?" token
                true
            } else {
                false
            };

            let type_annotation = if self.current_token.is(TokenKind::Colon) {
                let ann = self.parse_type_annotation()?;
                end_pos = ann.node.end;
                Some(ann)
            } else {
                None
            };

            let param = Parameter {
                node: Node::new(start_pos, end_pos),
                identifier,
                optional,
                type_annotation,
            };

            params.push(param);

            match self.current_token.kind {
                TokenKind::Comma => {
                    self.advance(); // Consume "," token
                }
                TokenKind::CloseParen => break,
                _ => throw_error!(InvalidToken),
            }
        }

        self.advance(); // Consume ")" token

        Ok(params)
    }

    /// Parses an `if` statement, including `else if` and `else` clauses.
    fn parse_if_statement(&mut self) -> Result<IfStatement, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "if" keyword token
        self.expect_and_consume_token(TokenKind::OpenParen)?;

        let test = self.parse_expression()?;

        self.expect_and_consume_token(TokenKind::CloseParen)?;

        let body = self.parse_statement(true)?;

        let consequent: Option<Statement> = match self.current_token.kind {
            TokenKind::Keyword => match self.current_token.value.expect_keyword() {
                Keyword::Else => {
                    self.advance(); // Consume "else" keyword token
                    Some(self.parse_statement(true)?)
                }
                _ => None,
            },
            _ => None,
        };

        let end_pos = match &consequent {
            Some(stmt) => stmt.node().end,
            None => body.node().end,
        };

        Ok(IfStatement {
            node: Node::new(start_pos, end_pos),
            test,
            alternate: body,
            consequent,
        })
    }

    /// Parses a `for` loop, including `for-in` and `for-of` loops.
    fn parse_for_statement(&mut self) -> Result<ForStatement, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "for" keyword token
        self.expect_and_consume_token(TokenKind::OpenParen)?;

        let head = self.parse_for_head()?;

        match self.current_token.kind {
            TokenKind::SemiColon => Ok(self.parse_for_classic(start_pos, head)?.into()),
            TokenKind::Keyword => match self.current_token.value.expect_keyword() {
                Keyword::In => {
                    self.advance(); // Consume "in" token
                    Ok(self.parse_for_in_or_of(start_pos, head, false)?.into())
                }
                Keyword::Of => {
                    self.advance(); // Consume "of" token
                    Ok(self.parse_for_in_or_of(start_pos, head, true)?.into())
                }
                _ => throw_error!(InvalidToken),
            },
            _ => throw_error!(InvalidToken),
        }
    }

    fn parse_for_head(&mut self) -> Result<ForHead, ParserErrorInfo> {
        if self.current_token.is(TokenKind::SemiColon) {
            return Ok(ForHead::Empty);
        }

        if self.current_token.is(TokenKind::Keyword) {
            let kw = self.current_token.value.expect_keyword();
            if matches!(kw, Keyword::Var | Keyword::Let | Keyword::Const) {
                let decl = self.parse_variable_declaration(false)?;
                return Ok(ForHead::VarDecl(decl));
            }
        }

        let expr = self.parse_expression()?;
        Ok(ForHead::Expr(expr))
    }

    fn parse_for_classic(
        &mut self,
        start_pos: usize,
        head: ForHead,
    ) -> Result<ForClassic, ParserErrorInfo> {
        let init: Option<Statement> = match head {
            ForHead::Empty => None,
            ForHead::VarDecl(decl) => Some(decl.into()),
            ForHead::Expr(expr) => Some(Statement::ExpressionStatement(Box::new(
                ExpressionStatement {
                    node: Node::new(start_pos, expr.node().end),
                    expression: expr,
                },
            ))),
        };

        self.expect_and_consume_token(TokenKind::SemiColon)?;

        let test: Option<Expression> = if !self.current_token.is(TokenKind::SemiColon) {
            Some(self.parse_expression()?.into())
        } else {
            None
        };

        self.expect_and_consume_token(TokenKind::SemiColon)?;

        let update: Option<Expression> = if !self.current_token.is(TokenKind::CloseParen) {
            Some(self.parse_expression()?.into())
        } else {
            None
        };

        self.expect_and_consume_token(TokenKind::CloseParen)?;
        let body = self.parse_statement(true)?;

        Ok(ForClassic {
            node: Node::new(start_pos, body.node().end),
            init,
            test,
            update,
            body,
        }
        .into())
    }

    fn parse_for_in_or_of(
        &mut self,
        start: usize,
        head: ForHead,
        is_of: bool,
    ) -> Result<ForStatement, ParserErrorInfo> {
        let right = self.parse_expression()?;
        self.expect_and_consume_token(TokenKind::CloseParen)?;
        let body = self.parse_statement(true)?;

        let node = Node::new(start, body.node().end);
        let left = match head {
            ForHead::VarDecl(decl) => ForLeft::VariableDeclaration(decl),
            ForHead::Expr(expr) => ForLeft::Expression(expr),
            ForHead::Empty => throw_error!(InternalError),
        };

        if is_of {
            Ok(ForOf {
                node,
                left,
                right,
                body,
            }
            .into())
        } else {
            Ok(ForIn {
                node,
                left,
                right,
                body,
            }
            .into())
        }
    }

    /// Parses `while` loop
    fn parse_while_statement(&mut self) -> Result<WhileStatement, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "while" keyword token

        self.expect_and_consume_token(TokenKind::OpenParen)?;

        let condition = self.parse_expression()?;

        self.expect_and_consume_token(TokenKind::CloseParen)?;

        let body = self.parse_statement(true)?;

        Ok(WhileStatement {
            node: Node::new(start_pos, body.node().end),
            condition,
            body,
        })
    }

    /// Parses a `return` statement.
    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        let mut end_pos = self.current_token.end;
        self.advance(); // Consume "return" token

        // TODO: Fix this case
        let argument = if !self.current_token.is(TokenKind::SemiColon) {
            let expr = self.parse_expression()?;
            end_pos = expr.node().end;
            Some(expr)
        } else {
            None
        };

        if self.current_token.is(TokenKind::SemiColon) {
            end_pos = self.current_token.end;
            self.advance();
        }

        Ok(ReturnStatement {
            node: Node::new(start_pos, end_pos),
            argument,
        })
    }

    /// Parses an assignment operation, such as `=` or compound assignments (e.g., `+=`, `-=`).
    fn parse_assignment_expression(
        &mut self,
        lhs: Expression,
    ) -> Result<AssignmentExpression, ParserErrorInfo> {
        let operator: AssignmentOperator = if self.current_token.is_operator() {
            Ok(self.current_token.kind.as_operator().unwrap())
        } else {
            throw_error!(InternalError)
        }?;

        self.advance(); // Consume operator token

        let expr = self.parse_expression()?;

        Ok(AssignmentExpression {
            node: Node::new(lhs.node().start, expr.node().end),
            left: lhs,
            right: expr,
            operator,
        })
    }

    /// Parses an array literal, such as [42]
    fn parse_array_literal(&mut self) -> Result<ArrayExpression, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "[" token

        let mut items: Vec<Expression> = Vec::new();

        loop {
            if self.current_token.is(TokenKind::CloseBracket) {
                break;
            }

            let expr = self.parse_expression()?;
            items.push(expr);

            match self.current_token.kind {
                TokenKind::Comma => self.advance(),
                TokenKind::CloseBracket => break,
                _ => throw_error!(InvalidToken),
            }
        }

        let arr = ArrayExpression {
            node: Node::new(start_pos, self.current_token.end), // Includes "[" and "]" tokens
            items,
        };

        self.advance(); // Consume "]" token

        Ok(arr)
    }

    /// Parses an object literal, such as { a: 4 }
    fn parse_object_literal(&mut self) -> Result<ObjectExpression, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "{" token

        let mut items = Vec::new();

        loop {
            if self.current_token.is(TokenKind::CloseBrace) {
                break;
            }

            let item: ObjectItem = match self.current_token.kind {
                TokenKind::String => {
                    let key = StringLiteral {
                        node: Node::new(self.current_token.start, self.current_token.end),
                        value: self.current_token.value.consume_string(),
                    }
                    .into();
                    self.advance(); // Consume String token
                    self.expect_and_consume_token(TokenKind::Colon)?;
                    let value = self.parse_expression()?;
                    ObjectItem::KV(KV { key, value })
                }
                TokenKind::Identifier => {
                    match self.lexer.peek_token().kind {
                        TokenKind::OpenParen => ObjectItem::Method(self.parse_method()?),
                        TokenKind::Colon => {
                            let key = Identifier {
                                node: Node::new(self.current_token.start, self.current_token.end),
                                name: self.current_token.value.expect_identifier().clone(),
                            }
                            .into();
                            self.advance(); // Consume Identifier token
                            self.advance(); // Consume ":" token
                            let value = self.parse_expression()?;
                            ObjectItem::KV(KV { key, value })
                        }
                        _ => {
                            let id = Identifier {
                                node: Node::new(self.current_token.start, self.current_token.end),
                                name: self.current_token.value.expect_identifier().clone(),
                            };
                            self.advance(); // Consume Identifier token
                            ObjectItem::Identifier(id)
                        }
                    }
                }
                TokenKind::Keyword => match self.current_token.value.expect_keyword() {
                    Keyword::Async => ObjectItem::Method(self.parse_method()?),
                    Keyword::StringType
                    | Keyword::NumberType
                    | Keyword::BooleanType
                    | Keyword::Type => {
                        let key = StringLiteral {
                            node: Node::new(self.current_token.start, self.current_token.end),
                            value: self.current_token.value.expect_keyword().to_string(),
                        }
                        .into();
                        self.advance(); // Consume keyword token
                        self.expect_and_consume_token(TokenKind::Colon)?;
                        let value = self.parse_expression()?;
                        ObjectItem::KV(KV { key, value })
                    }
                    _ => throw_error!(InvalidToken),
                },
                TokenKind::OpenBracket => {
                    let start_pos = self.current_token.start;

                    self.advance(); // Consume "[" token
                    let expression = self.parse_expression()?;
                    self.expect_token_kind(TokenKind::CloseBracket)?;
                    let key = Key::ComputedProperty(ComputedProperty {
                        node: Node::new(start_pos, self.current_token.end),
                        expression,
                    });
                    self.advance(); // Consume "]" token

                    self.expect_and_consume_token(TokenKind::Colon)?;

                    let value = self.parse_expression()?;

                    ObjectItem::KV(KV { key, value })
                }
                TokenKind::Dot => throw_error!(Todo),
                _ => throw_error!(InvalidToken),
            };

            items.push(item);

            match self.current_token.kind {
                TokenKind::CloseBrace => break,
                TokenKind::Comma => self.advance(),
                _ => throw_error!(InvalidToken),
            };
        }

        let obj = ObjectExpression {
            node: Node::new(start_pos, self.current_token.end), // Include "{" and "}" tokens
            items,
        };

        self.advance(); // Consume "}" token

        Ok(obj)
    }

    /// Parses binary operations (e.g., `+`, `-`, `*`, `/`, `&&`, `||`).
    fn parse_binary_expression(
        &mut self,
        left: Option<Expression>,
        precedence: u8,
    ) -> Result<BinaryExpression, ParserErrorInfo> {
        let mut left = match left {
            Some(expr) => expr,
            None => self.parse_primary_expression()?,
        };
        let start_pos = left.node().start;

        while let Some(op_precedence) = self.current_token.kind.get_operator_precedence() {
            if op_precedence <= precedence {
                break;
            }

            let operator = self.current_token.kind.as_operator().unwrap(); // Safe unwrap because of .get_operator_precedence()
            self.advance(); // Consume operator token

            let mut right = self.parse_primary_expression()?;

            if let Some(next_precedence) = self.current_token.kind.get_operator_precedence() {
                if next_precedence > op_precedence {
                    right = self
                        .parse_binary_expression(Some(right), op_precedence)?
                        .into();
                }
            }

            left = BinaryExpression {
                node: Node::new(start_pos, right.node().end),
                left,
                right,
                operator,
            }
            .into();
        }

        if let Expression::BinaryExpression(bin_exp) = left {
            Ok(*bin_exp)
        } else {
            throw_error!(InvalidToken) // Only happens if we never enter the while loop
        }
    }

    /// Parses a function or method call
    fn parse_call_expression(
        &mut self,
        callee: Expression,
    ) -> Result<CallExpression, ParserErrorInfo> {
        self.expect_and_consume_token(TokenKind::OpenParen)?;

        let mut arguments: Vec<Expression> = Vec::new();

        loop {
            if self.current_token.is(TokenKind::CloseParen) {
                break;
            }

            let expr = self.parse_expression()?;
            arguments.push(expr);

            match self.current_token.kind {
                TokenKind::Comma => self.advance(), // Consume "," token
                TokenKind::CloseParen => break,
                _ => throw_error!(InvalidToken),
            }
        }

        let expr = CallExpression {
            node: Node::new(callee.node().start, self.current_token.end),
            callee,
            arguments,
        };

        self.advance(); // Consume ")" token
        Ok(expr)
    }

    /// Parses member access expressions (e.g., `obj.prop` or `obj[prop]`).
    fn parse_member_expression(
        &mut self,
        object: Expression,
    ) -> Result<MemberExpression, ParserErrorInfo> {
        let property: MemberProperty;
        let end_pos: usize;

        match self.current_token.kind {
            TokenKind::Dot => {
                self.advance(); // Consume "." token
                self.expect_token_kind(TokenKind::Identifier)?;

                property = MemberProperty::Identifier(Identifier {
                    node: Node::new(self.current_token.start, self.current_token.end),
                    name: self.current_token.value.expect_identifier().clone(),
                });

                end_pos = self.current_token.end;
                self.advance(); // Consume Identifier token
            }
            TokenKind::OpenBracket => {
                let bracket_start = self.current_token.start;
                self.advance(); // Consume "[" token
                let expr = self.parse_expression()?;

                end_pos = self.current_token.end;

                property = MemberProperty::Computed(ComputedProperty {
                    node: Node::new(bracket_start, self.current_token.end),
                    expression: expr,
                });

                self.expect_and_consume_token(TokenKind::CloseBracket)?;
            }
            _ => unreachable!(),
        }

        Ok(MemberExpression {
            node: Node::new(object.node().start, end_pos),
            object,
            property,
        })
    }

    /// Parses literal values (e.g., strings, numbers, booleans).
    fn parse_literal(&mut self) -> Result<Literal, ParserErrorInfo> {
        match self.current_token.kind {
            TokenKind::String => {
                let s = StringLiteral {
                    node: Node::new(self.current_token.start, self.current_token.end),
                    value: self.current_token.value.consume_string(),
                };

                self.advance(); // Consume String token
                Ok(Literal::StringLiteral(s))
            }
            TokenKind::Number => {
                let n = NumberLiteral {
                    node: Node::new(self.current_token.start, self.current_token.end),
                    value: self.current_token.value.expect_number(),
                };

                self.advance(); // Consume Number token
                Ok(Literal::NumberLiteral(n))
            }
            TokenKind::Boolean => {
                let b = BooleanLiteral {
                    node: Node::new(self.current_token.start, self.current_token.end),
                    value: self.current_token.value.expect_boolean(),
                };

                self.advance(); // Consume Boolean token
                Ok(Literal::BooleanLiteral(b))
            }
            TokenKind::Null => {
                let n = NullLiteral {
                    node: Node::new(self.current_token.start, self.current_token.end),
                };

                self.advance(); // Consume Null token
                Ok(Literal::NullLiteral(n))
            }
            _ => throw_error!(InvalidToken),
        }
    }

    /// Parses TypeScript `enum` declarations.
    fn parse_enum_declaration(
        &mut self,
        is_const: bool,
        is_declare: bool,
    ) -> Result<EnumStatement, ParserErrorInfo> {
        let start_pos = self.current_token.start;

        if self.current_token.value.expect_keyword() != Keyword::Enum {
            self.advance(); // Consume "enum" or "declare" keyword tok
        }

        self.advance(); // Consume "enum" keyword token

        self.expect_token_kind(TokenKind::Identifier)?;
        let id = Identifier {
            node: Node::new(self.current_token.start, self.current_token.end),
            name: self.current_token.value.expect_identifier().clone(),
        };
        self.advance(); // Consume Identifier token

        self.expect_and_consume_token(TokenKind::OpenBrace)?;

        let mut members = Vec::new();

        loop {
            if self.current_token.is(TokenKind::CloseBrace) {
                break;
            }
            let start = self.current_token.start;

            self.expect_token_kind(TokenKind::Identifier)?;
            let id = Identifier {
                node: Node::new(self.current_token.start, self.current_token.end),
                name: self.current_token.value.expect_identifier().clone(),
            };
            self.advance(); // Consume Identifier token

            let (end_pos, init) = if self.current_token.is(TokenKind::Equals) {
                self.advance(); // Consume "=" token
                let expr = self.parse_expression()?;
                (expr.node().end, Some(expr))
            } else {
                (id.node.end, None)
            };

            members.push(EnumMember {
                node: Node::new(start, end_pos),
                id,
                init,
            });

            match self.current_token.kind {
                TokenKind::CloseBrace => break,
                TokenKind::Comma => self.advance(), // Consume "," token
                _ => throw_error!(InvalidToken),
            };
        }

        let end_pos = self.current_token.end;
        self.advance(); // Consume "}" token

        Ok(EnumStatement {
            node: Node::new(start_pos, end_pos),
            is_declare,
            is_const,
            id,
            members,
        })
    }

    fn parse_type_parameter_declaration(
        &mut self,
    ) -> Result<TypeParameterDeclaration, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.expect_and_consume_token(TokenKind::LessThan)?;

        let mut parameters = Vec::new();

        loop {
            match self.current_token.kind {
                TokenKind::Comma if parameters.len() != 0 => {
                    self.advance(); // Consume "," token
                }
                TokenKind::GreaterThan => break,
                _ => {}
            }

            self.expect_token_kind(TokenKind::Identifier)?;
            let name = self.current_token.value.expect_identifier().clone();
            parameters.push(TypeParameter {
                node: Node::new(self.current_token.start, self.current_token.end),
                id: Identifier {
                    node: Node::new(self.current_token.start, self.current_token.end),
                    name,
                },
            });
            self.advance(); // Consume Identifier token
        }

        let end_pos = self.current_token.end;
        self.advance(); // Consume ">" token

        Ok(TypeParameterDeclaration {
            node: Node::new(start_pos, end_pos),
            parameters,
        })
    }

    /// Parses type annotations specific to TypeScript (e.g., `: string`, `: number`).
    fn parse_type_annotation(&mut self) -> Result<TypeAnnotation, ParserErrorInfo> {
        let colon_start = self.current_token.start;
        self.expect_and_consume_token(TokenKind::Colon)?;
        let t = self.parse_type_value()?;
        Ok(TypeAnnotation {
            node: Node::new(colon_start, t.node().end),
            type_value: t,
        })
    }

    fn parse_type_value(&mut self) -> Result<AstType, ParserErrorInfo> {
        let start_pos = self.current_token.start;

        match self.current_token.kind {
            TokenKind::Keyword => {
                let Some(kw) = self.current_token.value.expect_keyword().as_type_keyword() else {
                    throw_error!(InvalidToken);
                };

                let t = KeywordType {
                    node: Node::new(start_pos, self.current_token.end),
                    kind: kw,
                };
                self.advance(); // Consume Keyword token

                match self.current_token.kind {
                    TokenKind::OpenBracket => {
                        self.advance(); // Consume "[" token
                        let arr = ArrayType {
                            node: Node::new(start_pos, self.current_token.end),
                            type_value: t.into(),
                        };
                        self.expect_and_consume_token(TokenKind::CloseBracket)?;
                        Ok(arr.into())
                    }
                    _ => Ok(t.into()),
                }
            }
            TokenKind::Identifier => {
                let name = self.current_token.value.expect_identifier().clone();
                let id = Identifier {
                    node: Node::new(start_pos, self.current_token.end),
                    name,
                };
                self.advance(); // Consume Identifier token

                match self.current_token.kind {
                    TokenKind::OpenBracket => {
                        self.advance(); // Consume "[" token
                        let arr = ArrayType {
                            node: Node::new(start_pos, self.current_token.end),
                            type_value: TypeReference {
                                node: Node::new(start_pos, id.node.end),
                                type_name: id,
                                type_params: None,
                            }
                            .into(),
                        };
                        self.expect_and_consume_token(TokenKind::CloseBracket)?;
                        Ok(arr.into())
                    }
                    TokenKind::LessThan => {
                        self.advance(); // Consume "<" token

                        let mut type_params = Vec::new();

                        loop {
                            match self.current_token.kind {
                                TokenKind::GreaterThan => break,
                                TokenKind::Comma if type_params.len() != 0 => {
                                    self.advance(); // Consume "," token
                                }
                                _ => {}
                            }
                            let inner = self.parse_type_value()?;
                            type_params.push(inner);
                        }

                        let t = TypeReference {
                            node: Node::new(start_pos, self.current_token.end),
                            type_name: id,
                            type_params: Some(type_params),
                        };
                        self.expect_and_consume_token(TokenKind::GreaterThan)?;
                        Ok(t.into())
                    }
                    _ => Ok(TypeReference {
                        node: id.node.clone(),
                        type_name: id,
                        type_params: None,
                    }
                    .into()),
                }
            }
            _ => throw_error!(InvalidToken),
        }
    }

    fn parse_typeof_expression(&mut self) -> Result<TypeofExpression, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "typeof" token
        let expr = self.parse_expression()?;
        Ok(TypeofExpression {
            node: Node::new(start_pos, expr.node().end),
            expression: expr,
        })
    }

    fn parse_new_expression(&mut self) -> Result<NewExpression, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "new" token
        let expr = self.parse_expression()?;
        Ok(NewExpression {
            node: Node::new(start_pos, expr.node().end),
            expr: Box::new(expr),
        })
    }
}
