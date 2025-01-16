use crate::expressions::{
    ArrayExpression, ArrowFunctionExpression, AsUpdateOperator, AssignmentExpression,
    BinaryExpression, BooleanLiteral, CallExpression, ComputedProperty, Expression,
    FunctionExpression, Literal, MemberExpression, MemberProperty, NullLiteral, NumberLiteral,
    ObjectExpression, ParenthesisExpression, StringLiteral, Type, TypeAnnotation, TypeValue,
    UnaryExpression, UpdateExpression, KV,
};
use crate::nodes::{program::Program, Node};
use crate::statements::{
    BlockStatement, ExpressionStatement, ForStatement, FunctionDeclaration, Identifier,
    IfStatement, Parameter, ReturnStatement, Statement, VariableDeclaration, VariableDeclarator,
    VariableKind, WhileStatement,
};
use lexer::{Keyword, Lexer, Operator, Token, TokenKind, TokenValue};

pub struct Parser<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    current_token: Token,
}

#[derive(Debug, PartialEq)]
pub struct ParserError {
    kind: ErrorKind,
    token: Token,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InternalError,
    InvalidToken,
    ExpectedClosingParen,
    ExpectedClosingBracket,
    ExpectedClosingBrace,
    ExpectedIdentifier,
    ExpectedComma,
    ExpectedOpenParen,
    ExpectedFunctionName,
    ExpectedColon,
    ExpectedSemiColon,
    ExpectedArrow,
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
            if self.current_token.kind == TokenKind::Eof {
                break;
            }

            let statement = self.parse_statement(true);

            match statement {
                Ok(s) => body.push(s),
                Err(err) => {
                    return Err(ParserError {
                        kind: err,
                        token: self.current_token.clone(),
                    })
                }
            }
        }

        Ok(Program {
            node: Node::new(0, source_len),
            shebang: None,
            body,
        })
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect_token_kind(&self, kind: TokenKind, err: ErrorKind) -> Result<(), ErrorKind> {
        if self.current_token.kind == kind {
            Ok(())
        } else {
            Err(err)
        }
    }

    fn expect_and_consume_token(
        &mut self,
        kind: TokenKind,
        err: ErrorKind,
    ) -> Result<(), ErrorKind> {
        if self.current_token.kind == kind {
            self.advance();
            Ok(())
        } else {
            Err(err)
        }
    }

    /// Parses a single statement (e.g., variable declarations, control flow statements, function definitions).
    fn parse_statement(&mut self, include_basic_semi: bool) -> Result<Statement, ErrorKind> {
        match self.current_token.kind {
            TokenKind::Keyword => match self.current_token.value.expect_keyword() {
                Keyword::Var | Keyword::Let | Keyword::Const => {
                    let var_dec = self.parse_variable_declaration(include_basic_semi)?;
                    Ok(Statement::VariableDeclaration(var_dec.into()))
                }
                Keyword::Function => {
                    let fn_dec = self.parse_function_declaration(false)?;
                    Ok(Statement::FunctionDeclaration(fn_dec.into()))
                }
                Keyword::Return => {
                    let rt_stmt = self.parse_return_statement()?;
                    Ok(Statement::ReturnStatement(rt_stmt.into()))
                }
                Keyword::If => {
                    let if_stmt = self.parse_if_statement()?;
                    Ok(Statement::IfStatement(if_stmt.into()))
                }
                Keyword::While => Ok(self.parse_while_statement()?.into()),
                Keyword::For => Ok(self.parse_for_statement()?.into()),
                _ => todo!(),
            },
            TokenKind::OpenBrace => Ok(self.parse_block_statement()?.into()),
            _ => {
                let expr = self.parse_expression()?;
                let end_pos =
                    if self.current_token.kind == TokenKind::SemiColon && include_basic_semi {
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
    fn parse_expression(&mut self) -> Result<Expression, ErrorKind> {
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
    fn parse_primary_expression(&mut self) -> Result<Expression, ErrorKind> {
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

                if self.current_token.kind == TokenKind::Dot {
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

                self.expect_and_consume_token(
                    TokenKind::CloseParen,
                    ErrorKind::ExpectedClosingParen,
                )?;

                Ok(paren_expr.into())
            }
            TokenKind::OpenBracket => {
                let arr = self.parse_array_literal()?;
                Ok(Expression::ArrayExpression(Box::new(arr)))
            }
            TokenKind::OpenBrace => {
                let obj = self.parse_object_literal()?;
                Ok(Expression::ObjectExpression(Box::new(obj)))
            }
            TokenKind::Keyword => match self.current_token.value.expect_keyword() {
                Keyword::Function => {
                    let fn_expr = self.parse_function_expression(false)?;
                    Ok(Expression::FunctionExpression(Box::new(fn_expr)))
                }
                _ => Err(ErrorKind::InvalidToken),
            },
            _ => Err(ErrorKind::InvalidToken),
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
            TokenKind::CloseParen => self.lexer.peek_token_at(2).kind == TokenKind::ArrowFn,
            _ => false,
        }
    }

    /// Parses a block of code, usually enclosed by `{}`.
    fn parse_block_statement(&mut self) -> Result<BlockStatement, ErrorKind> {
        let start_pos = self.current_token.start;
        self.expect_and_consume_token(TokenKind::OpenBrace, ErrorKind::InternalError)?;

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
    ) -> Result<VariableDeclaration, ErrorKind> {
        let start_pos = self.current_token.start;

        let kind = match self.current_token.value.expect_keyword() {
            Keyword::Var => VariableKind::Var,
            Keyword::Let => VariableKind::Let,
            Keyword::Const => VariableKind::Const,
            _ => unreachable!(),
        };
        self.advance();

        let mut declarations = Vec::new();
        let mut end_pos = start_pos;

        loop {
            let start = self.current_token.start;

            self.expect_token_kind(TokenKind::Identifier, ErrorKind::ExpectedIdentifier)?;
            let identifier = Identifier {
                node: Node::new(start, self.current_token.end),
                name: self.current_token.value.expect_identifier().clone(),
            };
            self.advance();

            match self.current_token.kind {
                TokenKind::Comma => {
                    declarations.push(VariableDeclarator {
                        node: Node::new(start, self.current_token.end),
                        id: identifier,
                        init: None,
                    });
                    end_pos = self.current_token.end;
                    self.advance();
                    continue;
                }
                TokenKind::Colon => todo!("Type annotation"),
                TokenKind::Equals => self.advance(),
                _ => break,
            }

            let expr = self.parse_expression()?;
            let decl = VariableDeclarator {
                node: Node::new(start, expr.node().end),
                id: identifier,
                init: Some(expr),
            };

            end_pos = decl.node.end;
            declarations.push(decl);

            if self.current_token.kind != TokenKind::Comma {
                break;
            }
        }

        if include_semi && self.current_token.kind == TokenKind::SemiColon {
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
    ) -> Result<FunctionDeclaration, ErrorKind> {
        let start_pos = self.current_token.start;

        self.advance(); // Consume "function" token

        self.expect_token_kind(TokenKind::Identifier, ErrorKind::ExpectedFunctionName)?;
        let id = Identifier {
            node: Node::new(self.current_token.start, self.current_token.end),
            name: self.current_token.value.expect_identifier().clone(),
        };
        self.advance(); // Consume Identifier token

        let is_generator = self.current_token.kind == TokenKind::Asterisk;
        if is_generator {
            self.advance(); // Consume "*" token
        }

        self.expect_token_kind(TokenKind::OpenParen, ErrorKind::ExpectedOpenParen)?;

        let params = self.parse_parameter_list()?;

        let return_type = if self.current_token.kind == TokenKind::Colon {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        let body = self.parse_block_statement()?;

        Ok(FunctionDeclaration {
            node: Node::new(start_pos, body.node.end),
            id,
            params,
            return_type,
            body,
            is_generator,
            is_async,
        })
    }

    /// Parses a function declaration, including its name, parameters, and body.
    fn parse_function_expression(
        &mut self,
        is_async: bool,
    ) -> Result<FunctionExpression, ErrorKind> {
        let start_pos = self.current_token.start;
        let mut is_generator = false;
        let mut return_type: Option<TypeAnnotation> = None;
        let mut id: Option<Identifier> = None;

        self.advance(); // Consume "function" keyword token

        if self.current_token.kind == TokenKind::Asterisk {
            is_generator = true;
            self.advance(); // Consume "*" token
        }

        if self.current_token.kind == TokenKind::Identifier {
            id = Some(Identifier {
                node: Node::new(self.current_token.start, self.current_token.end),
                name: self.current_token.value.expect_identifier().clone(),
            });
            self.advance(); // Consume Identifier token
        }

        self.expect_token_kind(TokenKind::OpenParen, ErrorKind::ExpectedOpenParen)?;

        let params = self.parse_parameter_list()?;

        // Explicit return type, like "function a(): number {}"
        if self.current_token.kind == TokenKind::Colon {
            let colon_start = self.current_token.start;
            self.advance(); // Consume ":" token
            let t = self.parse_type()?;
            return_type = Some(TypeAnnotation {
                node: Node::new(colon_start, t.node.end),
                type_value: t,
            });
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

    fn parse_arrow_function(&mut self) -> Result<ArrowFunctionExpression, ErrorKind> {
        let start_pos = self.current_token.start;
        let parameters = self.parse_parameter_list()?;

        let return_type = if self.current_token.kind == TokenKind::Colon {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        self.expect_and_consume_token(TokenKind::ArrowFn, ErrorKind::ExpectedArrow)?;
        let body = self.parse_statement(false)?;

        Ok(ArrowFunctionExpression {
            node: Node::new(start_pos, body.node().end),
            parameters,
            return_type,
            body,
        })
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>, ErrorKind> {
        self.expect_and_consume_token(TokenKind::OpenParen, ErrorKind::InternalError)?;

        let mut params: Vec<Parameter> = Vec::new();

        while self.current_token.kind != TokenKind::CloseParen {
            let start_pos = self.current_token.start;
            self.expect_token_kind(TokenKind::Identifier, ErrorKind::ExpectedIdentifier)?;
            let id = self.current_token.value.expect_identifier();
            let identifier = Identifier {
                node: Node::new(self.current_token.start, self.current_token.end),
                name: id.clone(),
            };
            let mut end_pos = self.current_token.end;
            self.advance(); // Consume Identifier token

            let optional = if self.current_token.kind == TokenKind::QuestionMark {
                end_pos = self.current_token.end;
                self.advance(); // Consume "?" token
                true
            } else {
                false
            };

            let type_annotation = if self.current_token.kind == TokenKind::Colon {
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
                _ => return Err(ErrorKind::InvalidToken),
            }
        }

        self.advance(); // Consume ")" token

        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Type, ErrorKind> {
        let start_pos = self.current_token.start;

        let value: TypeValue = match self.current_token.kind {
            TokenKind::Keyword => {
                let kw = self
                    .current_token
                    .value
                    .expect_keyword()
                    .as_type_keyword()
                    .ok_or(ErrorKind::InvalidToken)?;
                TypeValue::KeywordType(kw)
            }
            TokenKind::Identifier => {
                TypeValue::TypeReference(self.current_token.value.expect_identifier().clone())
            }
            _ => return Err(ErrorKind::InvalidToken),
        };

        let end_pos = self.current_token.end;
        self.advance(); // Consume type token

        if self.current_token.kind == TokenKind::LessThan {
            todo!("Generic types");
        }

        Ok(Type {
            node: Node::new(start_pos, end_pos),
            value,
        })
    }

    /// Parses a class declaration, including its methods and properties.
    fn parse_class_declaration(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses an `import` statement
    fn parse_import_declaration(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses an `export` statement
    fn parse_export_declaration(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses an `if` statement, including `else if` and `else` clauses.
    fn parse_if_statement(&mut self) -> Result<IfStatement, ErrorKind> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "if" keyword token
        self.expect_and_consume_token(TokenKind::OpenParen, ErrorKind::ExpectedOpenParen)?;

        let condition = self.parse_expression()?;

        self.expect_and_consume_token(TokenKind::CloseParen, ErrorKind::ExpectedClosingParen)?;

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
            condition,
            body,
            consequent,
        })
    }

    /// Parses a `for` loop, including `for-in` and `for-of` loops.
    fn parse_for_statement(&mut self) -> Result<ForStatement, ErrorKind> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "for" keyword token

        self.expect_and_consume_token(TokenKind::OpenParen, ErrorKind::ExpectedOpenParen)?;

        let initializer = self.parse_statement(false)?;

        self.expect_and_consume_token(TokenKind::SemiColon, ErrorKind::ExpectedSemiColon)?;

        let condition = self.parse_expression()?;

        self.expect_and_consume_token(TokenKind::SemiColon, ErrorKind::ExpectedSemiColon)?;

        let update = self.parse_statement(false)?;

        self.expect_and_consume_token(TokenKind::CloseParen, ErrorKind::ExpectedClosingParen)?;

        let body = self.parse_statement(false)?;

        Ok(ForStatement {
            node: Node::new(start_pos, body.node().end),
            initializer,
            condition,
            update,
            body,
        })
    }

    /// Parses `while` loop
    fn parse_while_statement(&mut self) -> Result<WhileStatement, ErrorKind> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "while" keyword token

        self.expect_and_consume_token(TokenKind::OpenParen, ErrorKind::ExpectedOpenParen)?;

        let condition = self.parse_expression()?;

        self.expect_and_consume_token(TokenKind::CloseParen, ErrorKind::ExpectedClosingParen)?;

        let body = self.parse_statement(true)?;

        Ok(WhileStatement {
            node: Node::new(start_pos, body.node().end),
            condition,
            body,
        })
    }

    /// Parses a `switch` statement, including its cases and default clause.
    fn parse_switch_statement(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses a `return` statement.
    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ErrorKind> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "return" token

        let expr = self.parse_expression()?;
        let mut end_pos = expr.node().end;

        if self.current_token.kind == TokenKind::SemiColon {
            end_pos = self.current_token.end;
            self.advance();
        }

        Ok(ReturnStatement {
            node: Node::new(start_pos, end_pos),
            value: expr,
        })
    }

    /// Parses an assignment operation, such as `=` or compound assignments (e.g., `+=`, `-=`).
    fn parse_assignment_expression(
        &mut self,
        lhs: Expression,
    ) -> Result<AssignmentExpression, ErrorKind> {
        let operator: Operator = if self.current_token.kind.is_assignment_operator() {
            Ok(self.current_token.kind.as_operator().unwrap())
        } else {
            Err(ErrorKind::InternalError)
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
    fn parse_array_literal(&mut self) -> Result<ArrayExpression, ErrorKind> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "[" token

        let mut items: Vec<Expression> = Vec::new();

        loop {
            let expr = self.parse_expression()?;
            items.push(expr);

            match self.current_token.kind {
                TokenKind::CloseBracket => break,
                TokenKind::Comma => continue,
                _ => return Err(ErrorKind::ExpectedClosingBracket),
            };
        }

        let arr = ArrayExpression {
            node: Node::new(start_pos, self.current_token.end), // Includes "[" and "]" tokens
            items,
        };

        self.advance(); // Consume "]" token

        Ok(arr)
    }

    /// Parses an object literal, such as { a: 4 }
    fn parse_object_literal(&mut self) -> Result<ObjectExpression, ErrorKind> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "{" token

        let mut items: Vec<KV> = Vec::new();

        loop {
            if self.current_token.kind == TokenKind::CloseBracket {
                break;
            }

            let key = self.parse_primary_expression()?;

            self.expect_and_consume_token(TokenKind::Colon, ErrorKind::ExpectedComma)?;

            let value = self.parse_expression()?;

            items.push(KV { key, value });

            match self.current_token.kind {
                TokenKind::CloseBrace => break,
                TokenKind::Comma => {
                    self.advance();
                    continue;
                }
                _ => return Err(ErrorKind::InvalidToken),
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
    ) -> Result<BinaryExpression, ErrorKind> {
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
            Err(ErrorKind::InvalidToken) // Only happens if we never enter the while loop
        }
    }

    /// Parses unary operations (e.g., `!`, `-`).
    fn parse_unary_expression(&mut self) -> Result<UnaryExpression, ErrorKind> {
        todo!()
    }

    /// Parses a function or method call
    fn parse_call_expression(&mut self, callee: Expression) -> Result<CallExpression, ErrorKind> {
        self.expect_and_consume_token(TokenKind::OpenParen, ErrorKind::InternalError)?;

        let mut arguments: Vec<Expression> = Vec::new();

        loop {
            if self.current_token.kind == TokenKind::CloseParen {
                break;
            }

            let expr = self.parse_expression()?;
            arguments.push(expr);

            match self.current_token.kind {
                TokenKind::Comma => {
                    self.advance(); // Consume "," token
                }
                TokenKind::CloseParen => break,
                _ => return Err(ErrorKind::ExpectedClosingParen),
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
    ) -> Result<MemberExpression, ErrorKind> {
        let property: MemberProperty;
        let end_pos: usize;

        match self.current_token.kind {
            TokenKind::Dot => {
                self.advance(); // Consume "." token
                self.expect_token_kind(TokenKind::Identifier, ErrorKind::ExpectedIdentifier)?;

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

                self.expect_and_consume_token(
                    TokenKind::CloseBracket,
                    ErrorKind::ExpectedClosingBracket,
                )?;
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
    fn parse_literal(&mut self) -> Result<Literal, ErrorKind> {
        match self.current_token.kind {
            TokenKind::String => {
                let s = StringLiteral {
                    node: Node::new(self.current_token.start, self.current_token.end),
                    value: self.current_token.value.consume_string(), // WARN: must not be used again
                };

                self.advance(); // Consume String token
                Ok(Literal::String(s))
            }
            TokenKind::Number => {
                let n = NumberLiteral {
                    node: Node::new(self.current_token.start, self.current_token.end),
                    value: self.current_token.value.expect_number(),
                };

                self.advance(); // Consume Number token
                Ok(Literal::Number(n))
            }
            TokenKind::Boolean => {
                let b = BooleanLiteral {
                    node: Node::new(self.current_token.start, self.current_token.end),
                    value: self.current_token.value.expect_boolean(),
                };

                self.advance(); // Consume Boolean token
                Ok(Literal::Boolean(b))
            }
            TokenKind::Null => {
                let n = NullLiteral {
                    node: Node::new(self.current_token.start, self.current_token.end),
                };

                self.advance(); // Consume Null token
                Ok(Literal::Null(n))
            }
            _ => Err(ErrorKind::InvalidToken),
        }
    }

    /// Parses type annotations specific to TypeScript (e.g., `: string`, `: number`).
    fn parse_type_annotation(&mut self) -> Result<TypeAnnotation, ErrorKind> {
        let colon_start = self.current_token.start;
        self.expect_and_consume_token(TokenKind::Colon, ErrorKind::ExpectedColon)?;
        let t = self.parse_type()?;
        Ok(TypeAnnotation {
            node: Node::new(colon_start, t.node.end),
            type_value: t,
        })
    }

    /// Parses generic type parameters (e.g., `<T>`, `<T, U>`).
    fn parse_generic_parameters(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses TypeScript `interface` declarations.
    fn parse_interface_declaration(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses TypeScript `type` alias declarations.
    fn parse_type_alias(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses TypeScript `enum` declarations.
    fn parse_enum_declaration(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses decorators used in TypeScript (e.g., `@Component`).
    fn parse_decorator(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses optional chaining expressions (e.g., `obj?.prop`).
    fn parse_optional_chain(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses nullish coalescing (`??`).
    fn parse_nullish_coalescing(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses type assertions (e.g., `value as Type`).
    fn parse_type_assertion(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses abstract members in classes.
    fn parse_abstract_members(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses module declarations (e.g., namespace or module).
    fn parse_module(&mut self) -> Result<(), ParserError> {
        todo!()
    }
}
