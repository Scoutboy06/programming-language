use crate::ast_types::classes::class::{MethodDefinition, MethodDefinitionKind};
use crate::ast_types::declarations::declaration::Declaration;
use crate::ast_types::declarations::function_declaration::{FunctionDeclaration, Parameter};
use crate::ast_types::declarations::variable_declaration::{
    VariableDeclaration, VariableDeclarationKind, VariableDeclarator,
};
use crate::ast_types::expressions::{
    ArrayExpression, ArrowFunctionExpression, ArrowFunctionExpressionBody, AssignmentExpression,
    BinaryExpression, CallExpression, CallExpressionCallee, Expression, FunctionExpression,
    LogicalExpression, MemberExpression, MemberExpressionProperty, NewExpression, ObjectExpression,
    ObjectExpressionProperty, ThisExpression, UnaryExpression, UpdateExpression,
};
use crate::ast_types::identifier::Identifier;
use crate::ast_types::literal::{Literal, LiteralValue};
use crate::ast_types::modules::import_or_export_declaration::ImportOrExportDeclaration;
use crate::ast_types::node_objects::Node;
use crate::ast_types::operators::{
    AssignmentOperator, BinaryOperator, LogicalOperator, Operator, TokenKindExt, UnaryOperator,
};
use crate::ast_types::patterns::pattern::Pattern;
use crate::ast_types::programs::program::{ProgramBodyItem, SourceType};
use crate::ast_types::programs::Program;
use crate::ast_types::property::{Property, PropertyKind};
use crate::ast_types::statements::{
    BlockStatement, BreakStatement, ContinueStatement, ExpressionStatement, ForInit, ForStatement,
    FunctionBody, FunctionBodyBody, IfStatement, ReturnStatement, Statement, ThrowStatement,
    WhileStatement,
};
use crate::ast_types::types::{
    ArrayType, AstType, KeywordType, TypeAnnotation, TypeParameter, TypeParameterDeclaration,
    TypeReference,
};
use crate::throw_error;
use crate::utils::parser_error::{ParserError, ParserErrorInfo};
use crate::utils::throw_error;
use lexer::{Keyword, Lexer, Token, TokenKind};

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
        let mut body: Vec<ProgramBodyItem> = Vec::new();

        // Initialize tokens
        self.advance();

        loop {
            if self.current_token.is(TokenKind::Eof) {
                break;
            }

            let statement = self.parse_statement(true);

            match statement {
                Ok(s) => body.push(ProgramBodyItem::Statement(s)),
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
            source_type: SourceType::Script,
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

    fn parse_program_body_item(&mut self) -> Result<ProgramBodyItem, ParserErrorInfo> {
        if self.current_token.is(TokenKind::Keyword)
            && matches!(
                self.current_token.value.expect_keyword(),
                Keyword::Import | Keyword::Export
            )
        {
            return Ok(ProgramBodyItem::ImportOrExportDeclaration(
                self.parse_import_or_export_declaration()?,
            ));
        }
        Ok(ProgramBodyItem::Statement(self.parse_statement(true)?))
    }

    fn parse_import_or_export_declaration(
        &mut self,
    ) -> Result<ImportOrExportDeclaration, ParserErrorInfo> {
        throw_error!(Todo)
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
                            throw_error!(Todo);
                            // return Ok(self.parse_enum_declaration(true, false)?.into());
                        }
                    }
                    let decl: Declaration =
                        self.parse_variable_declaration(include_basic_semi)?.into();
                    Ok(decl.into())
                }
                Keyword::Function => {
                    let decl: Declaration = self.parse_function_declaration()?.into();
                    Ok(decl.into())
                }
                Keyword::Return => Ok(self.parse_return_statement()?.into()),
                Keyword::If => Ok(self.parse_if_statement()?.into()),
                Keyword::While => Ok(self.parse_while_statement()?.into()),
                Keyword::For => Ok(self.parse_for_statement()?.into()),
                // Keyword::Enum => Ok(self.parse_enum_declaration(false, false)?.into()),
                // Keyword::Declare => Ok(self.parse_enum_declaration(false, true)?.into()),
                Keyword::Enum => throw_error!(Todo),
                Keyword::Declare => throw_error!(Todo),
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
                    let argument = self.parse_expression()?;
                    if self.current_token.is(TokenKind::Semi) {
                        self.advance(); // Consume ";" token
                    }
                    Ok(ThrowStatement {
                        node: Node::new(start_pos, argument.node().end),
                        argument,
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

                    let label = if self.current_token.is(TokenKind::Identifier) {
                        end_pos = self.current_token.end;
                        Some(Identifier {
                            node: Node::new(self.current_token.start, self.current_token.end),
                            name: self.current_token.value.expect_identifier().to_owned(),
                        })
                    } else {
                        None
                    };

                    if include_basic_semi && self.current_token.is(TokenKind::Semi) {
                        end_pos = self.current_token.end;
                        self.advance(); // Consume ';' token
                    }

                    Ok(ContinueStatement {
                        node: Node::new(start_pos, end_pos),
                        label,
                    }
                    .into())
                }
                Keyword::Break => {
                    let start_pos = self.current_token.start;
                    let mut end_pos = self.current_token.end;
                    self.advance(); // Consume "break" token

                    let label = if self.current_token.is(TokenKind::Identifier) {
                        end_pos = self.current_token.end;
                        Some(Identifier {
                            node: Node::new(self.current_token.start, self.current_token.end),
                            name: self.current_token.value.expect_identifier().to_owned(),
                        })
                    } else {
                        None
                    };

                    if include_basic_semi && self.current_token.is(TokenKind::Semi) {
                        end_pos = self.current_token.end;
                        self.advance(); // Consume ';' token
                    }

                    Ok(BreakStatement {
                        node: Node::new(start_pos, end_pos),
                        label,
                    }
                    .into())
                }
                _ => throw_error!(InvalidToken),
            },
            TokenKind::OpenBrace => Ok(self.parse_block_statement()?.into()),
            _ => {
                let expr = self.parse_expression()?;
                let end_pos = if self.current_token.is(TokenKind::Semi) && include_basic_semi {
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

    fn parse_pattern(&mut self) -> Result<Pattern, ParserErrorInfo> {
        throw_error!(Todo)
    }

    /// Parses an expression (e.g., arithmetic operations, logical operations, or function calls).
    fn parse_expression(&mut self) -> Result<Expression, ParserErrorInfo> {
        self.parse_expression_prec(0)
    }

    /// Precedence climbing expression parser
    fn parse_expression_prec(&mut self, min_prec: u8) -> Result<Expression, ParserErrorInfo> {
        let mut lhs = self.parse_primary_expression()?;

        loop {
            // Handle postfix update operators (++/--)
            if let Some(operator) = self.current_token.kind.as_update_op() {
                let expr = UpdateExpression {
                    node: Node::new(lhs.node().start, self.current_token.end),
                    operator,
                    argument: lhs,
                    prefix: false,
                };
                self.advance();
                lhs = expr.into();
                continue;
            }

            // Determine operator kind and precedence
            let (op_prec, is_right_assoc, op_kind) =
                if let Some(op) = self.current_token.kind.as_binary_op() {
                    (
                        op.precedence(),
                        op == BinaryOperator::Power,
                        Some(Operator::Binary(op)),
                    )
                } else if let Some(op) = self.current_token.kind.as_logical_op() {
                    (op.precedence(), false, Some(Operator::Logical(op)))
                } else if let Some(op) = self.current_token.kind.as_assignment_op() {
                    (op.precedence(), true, Some(Operator::Assignment(op)))
                } else {
                    (0, false, None)
                };

            let op_kind = match op_kind {
                Some(kind) => kind,
                None => break,
            };

            if op_prec < min_prec {
                break;
            }

            match op_kind {
                Operator::Binary(op) => {
                    self.advance();
                    let next_min_prec = if is_right_assoc { op_prec } else { op_prec + 1 };
                    let rhs = self.parse_expression_prec(next_min_prec)?;
                    let node = Node::new(lhs.node().start, rhs.node().end);
                    lhs = Expression::BinaryExpression(Box::new(BinaryExpression {
                        node,
                        left: lhs,
                        right: rhs,
                        operator: op,
                    }));
                }
                Operator::Logical(op) => {
                    self.advance();
                    let rhs = self.parse_expression_prec(op_prec + 1)?;
                    let node = Node::new(lhs.node().start, rhs.node().end);
                    lhs = LogicalExpression {
                        node,
                        left: lhs,
                        right: rhs,
                        operator: op,
                    }
                    .into();
                }
                Operator::Assignment(op) => {
                    self.advance();
                    // For assignment, left must be a pattern
                    let left_pattern = match lhs.clone() {
                        Expression::Identifier(id) => Pattern::Identifier(id),
                        // Add more conversions as needed
                        _ => throw_error!(InvalidToken),
                    };
                    let rhs = self.parse_expression_prec(op_prec)?;
                    let node = Node::new(lhs.node().start, rhs.node().end);
                    lhs = AssignmentExpression {
                        node,
                        left: left_pattern,
                        right: rhs,
                        operator: op,
                    }
                    .into();
                }
                _ => unreachable!(),
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
                    let arr_fn_expr = self.parse_arrow_function(true)?;
                    return Ok(Expression::ArrowFunctionExpression(Box::new(arr_fn_expr)));
                }

                let start_pos = self.current_token.start;
                self.advance(); // Consume "(" token

                let expression = self.parse_expression()?;

                self.expect_and_consume_token(TokenKind::CloseParen)?;

                Ok(expression)
            }
            TokenKind::OpenBracket => Ok(self.parse_array_literal()?.into()),
            TokenKind::OpenBrace => Ok(self.parse_object_literal()?.into()),
            TokenKind::Bang | TokenKind::Plus | TokenKind::Minus => {
                let operator = match self.current_token.kind {
                    TokenKind::Bang => UnaryOperator::LogicalNot,
                    TokenKind::Plus => UnaryOperator::Plus,
                    TokenKind::Minus => UnaryOperator::Minus,
                    _ => unreachable!(),
                };
                let start_pos = self.current_token.start;
                self.advance(); // Consume unary token
                let argument = self.parse_expression()?;
                Ok(UnaryExpression {
                    node: Node::new(start_pos, argument.node().end),
                    operator,
                    prefix: true,
                    argument,
                }
                .into())
            }
            TokenKind::Keyword => match self.current_token.value.expect_keyword() {
                Keyword::Function => Ok(self.parse_function_expression()?.into()),
                // Keyword::Typeof => Ok(self.parse_typeof_expression()?.into()),
                Keyword::Typeof => throw_error!(Todo),
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
                // let node = Node::new(self.current_token.start, self.current_token.end);
                // let value = self.current_token.value.consume_regex();
                // self.advance(); // Consume Regex token
                // Ok(Literal {
                //     node,
                //     value: LiteralValue::RegExp(value.into()),
                // }
                // .into())
                throw_error!(Todo)
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
            TokenKind::CloseParen => self.lexer.peek_token_at(2).is(TokenKind::EqGt),
            _ => false,
        }
    }

    /// Parses a block of code, usually enclosed by `{}`.
    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.expect_and_consume_token(TokenKind::OpenBrace)?;

        let mut body: Vec<Statement> = Vec::new();

        while self.current_token.kind != TokenKind::CloseBrace {
            let stmt = self.parse_statement(true)?;
            body.push(stmt);
        }

        let block = BlockStatement {
            node: Node::new(start_pos, self.current_token.end),
            body,
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
            Keyword::Var => VariableDeclarationKind::Var,
            Keyword::Let => VariableDeclarationKind::Let,
            Keyword::Const => VariableDeclarationKind::Const,
            _ => unreachable!(),
        };
        self.advance();

        let mut declarations = Vec::new();
        let mut end_pos;

        loop {
            let start = self.current_token.start;
            end_pos = self.current_token.end;

            self.expect_token_kind(TokenKind::Identifier)?;
            let id = Identifier {
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

            let init = if self.current_token.is(TokenKind::Eq) {
                self.advance(); // Consume "=" token
                let expr = self.parse_expression()?;
                end_pos = expr.node().end;
                Some(expr)
            } else {
                None
            };

            let decl = VariableDeclarator {
                node: Node::new(start, end_pos),
                id: id.into(),
                type_annotation,
                init,
            };

            declarations.push(decl);

            if self.current_token.kind != TokenKind::Comma {
                break;
            }
            self.advance(); // Consume "," token
        }

        if include_semi && self.current_token.is(TokenKind::Semi) {
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
    fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration, ParserErrorInfo> {
        let start_pos = self.current_token.start;

        self.advance(); // Consume "function" token

        self.expect_token_kind(TokenKind::Identifier)?;
        let id = Identifier {
            node: Node::new(self.current_token.start, self.current_token.end),
            name: self.current_token.value.expect_identifier().clone(),
        };
        self.advance(); // Consume Identifier token

        let type_parameters = if self.current_token.is(TokenKind::Lt) {
            Some(self.parse_type_parameter_declaration()?)
        } else {
            None
        };

        let generator = self.current_token.is(TokenKind::Star);
        if generator {
            self.advance(); // Consume "*" token
        }

        self.expect_token_kind(TokenKind::OpenParen)?;

        let params = self.parse_params()?;

        let return_type = if self.current_token.is(TokenKind::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        let body = self.parse_function_body()?;

        Ok(FunctionDeclaration {
            node: Node::new(start_pos, body.node.end),
            id,
            generator,
            type_parameters,
            params,
            return_type,
            body,
        })
    }

    /// Parses a function declaration, including its name, parameters, and body.
    fn parse_function_expression(&mut self) -> Result<FunctionExpression, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        let mut generator = false;
        let mut return_type: Option<TypeAnnotation> = None;
        let mut id: Option<Identifier> = None;

        self.advance(); // Consume "function" keyword token

        if self.current_token.is(TokenKind::Star) {
            generator = true;
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

        let params = self.parse_params()?;

        // Explicit return type, like "function a(): number {}"
        if self.current_token.is(TokenKind::Colon) {
            return_type = Some(self.parse_type_annotation()?);
        }

        let body = self.parse_function_body()?;

        Ok(FunctionExpression {
            node: Node::new(start_pos, body.node.end),
            id,
            params,
            return_type,
            body,
            generator,
        })
    }

    fn parse_function_body(&mut self) -> Result<FunctionBody, ParserErrorInfo> {
        let start_pos = self.current_token.start;

        match self.current_token.kind {
            TokenKind::OpenBrace => {
                let block = self.parse_block_statement()?;

                // TODO: allow for directives
                let body = block
                    .body
                    .into_iter()
                    .map(FunctionBodyBody::Statement)
                    .collect();

                Ok(FunctionBody {
                    node: Node::new(start_pos, block.node.end),
                    body,
                })
            }
            _ => throw_error!(Todo),
        }
    }

    fn parse_method_definition(&mut self) -> Result<MethodDefinition, ParserErrorInfo> {
        let start_pos = self.current_token.start;

        // let is_async = self.current_token.matches_keyword(Keyword::Async);
        // if is_async {
        //     self.advance(); // Consume "async" token
        // }

        // let is_generator = self.current_token.matches_bin_op(BinaryOperator::Mult);
        // if is_generator {
        //     self.advance(); // Consume '*' token
        // }

        let key = self.parse_expression()?;

        self.expect_and_consume_token(TokenKind::Colon)?;

        let value = self.parse_function_expression()?;

        let kind = MethodDefinitionKind::Method; // TODO: Support getters, setters, constructor

        // let type_parameters = if self.current_token.matches_bin_op(BinaryOperator::LessThan) {
        //     Some(self.parse_type_parameter_declaration()?)
        // } else {
        //     None
        // };

        // let parameters = self.parse_parameter_list()?;

        // let return_type = if self.current_token.is(TokenKind::Colon) {
        //     Some(self.parse_type_annotation()?)
        // } else {
        //     None
        // };

        let body = self.parse_block_statement()?;

        Ok(MethodDefinition {
            node: Node::new(start_pos, body.node.end),
            key,
            value,
            kind,
            computed: false, // TODO
            _static: false,  // TODO
        })
    }

    fn parse_arrow_function(
        &mut self,
        is_expression: bool,
    ) -> Result<ArrowFunctionExpression, ParserErrorInfo> {
        let start_pos = self.current_token.start;

        let id: Option<Identifier> = match self.current_token.kind {
            TokenKind::Identifier => Some(Identifier {
                node: Node::new(self.current_token.start, self.current_token.end),
                name: self.current_token.value.expect_identifier().clone(),
            }),
            _ => None,
        };

        let params = self.parse_params()?;

        let return_type = if self.current_token.is(TokenKind::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        self.expect_and_consume_token(TokenKind::EqGt)?;
        let body = match self.current_token.kind {
            TokenKind::OpenBrace => {
                ArrowFunctionExpressionBody::FunctionBody(self.parse_function_body()?)
            }
            _ => ArrowFunctionExpressionBody::Expression(self.parse_expression()?),
        };

        Ok(ArrowFunctionExpression {
            node: Node::new(start_pos, body.node().end),
            id,
            params,
            return_type,
            body,
            expression: is_expression,
        })
    }

    fn parse_params(&mut self) -> Result<Vec<Pattern>, ParserErrorInfo> {
        let mut params: Vec<Pattern> = Vec::new();

        self.expect_and_consume_token(TokenKind::OpenParen)?;

        if !self.current_token.is(TokenKind::CloseParen) {
            loop {
                let pattern = self.parse_pattern()?;
                params.push(pattern);

                if self.current_token.is(TokenKind::Comma) {
                    self.advance(); // Consume "," token

                    if self.current_token.is(TokenKind::CloseParen) {
                        break;
                    }
                    continue;
                }

                break; // If no comma, the next token should be ")"
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

        let consequent = self.parse_statement(true)?;

        let alternate: Option<Statement> = match self.current_token.kind {
            TokenKind::Keyword => match self.current_token.value.expect_keyword() {
                Keyword::Else => {
                    self.advance(); // Consume "else" keyword token
                    Some(self.parse_statement(true)?)
                }
                _ => None,
            },
            _ => None,
        };

        let end_pos = match &alternate {
            Some(stmt) => stmt.node().end,
            None => consequent.node().end,
        };

        Ok(IfStatement {
            node: Node::new(start_pos, end_pos),
            test,
            consequent,
            alternate,
        })
    }

    /// Parses a `for` loop, including `for-in` and `for-of` loops.
    fn parse_for_statement(&mut self) -> Result<ForStatement, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "for" keyword token
        self.expect_and_consume_token(TokenKind::OpenParen)?;

        throw_error!(Todo);

        // let head = self.parse_for_head()?;

        // match self.current_token.kind {
        //     TokenKind::Semi => Ok(self.parse_for_classic(start_pos, head)?.into()),
        //     TokenKind::Keyword => match self.current_token.value.expect_keyword() {
        //         Keyword::In => {
        //             self.advance(); // Consume "in" token
        //             Ok(self.parse_for_in_or_of(start_pos, head, false)?.into())
        //         }
        //         Keyword::Of => {
        //             self.advance(); // Consume "of" token
        //             Ok(self.parse_for_in_or_of(start_pos, head, true)?.into())
        //         }
        //         _ => throw_error!(InvalidToken),
        //     },
        //     _ => throw_error!(InvalidToken),
        // }
    }

    // fn parse_for_head(&mut self) -> Result<ForHead, ParserErrorInfo> {
    //     if self.current_token.is(TokenKind::Semi) {
    //         return Ok(ForHead::Empty);
    //     }
    //
    //     if self.current_token.is(TokenKind::Keyword) {
    //         let kw = self.current_token.value.expect_keyword();
    //         if matches!(kw, Keyword::Var | Keyword::Let | Keyword::Const) {
    //             let decl = self.parse_variable_declaration(false)?;
    //             return Ok(ForHead::VarDecl(decl));
    //         }
    //     }
    //
    //     let expr = self.parse_expression()?;
    //     Ok(ForHead::Expr(expr))
    // }
    //
    // fn parse_for_classic(
    //     &mut self,
    //     start_pos: usize,
    //     head: ForHead,
    // ) -> Result<ForStatement, ParserErrorInfo> {
    //     let init: Option<ForInit> = match head {
    //         ForHead::Empty => None,
    //         ForHead::VarDecl(decl) => Some(decl.into()),
    //         ForHead::Expr(expr) => Some(Statement::ExpressionStatement(Box::new(
    //             ExpressionStatement {
    //                 node: Node::new(start_pos, expr.node().end),
    //                 expression: expr,
    //             },
    //         ))),
    //     };
    //
    //     self.expect_and_consume_token(TokenKind::Semi)?;
    //
    //     let test: Option<Expression> = if !self.current_token.is(TokenKind::Semi) {
    //         Some(self.parse_expression()?.into())
    //     } else {
    //         None
    //     };
    //
    //     self.expect_and_consume_token(TokenKind::Semi)?;
    //
    //     let update: Option<Expression> = if !self.current_token.is(TokenKind::CloseParen) {
    //         Some(self.parse_expression()?.into())
    //     } else {
    //         None
    //     };
    //
    //     self.expect_and_consume_token(TokenKind::CloseParen)?;
    //     let body = self.parse_statement(true)?;
    //
    //     Ok(ForStatement {
    //         node: Node::new(start_pos, body.node().end),
    //         init,
    //         test,
    //         update,
    //         body,
    //     }
    //     .into())
    // }
    //
    // fn parse_for_in_or_of(
    //     &mut self,
    //     start: usize,
    //     head: ForHead,
    //     is_of: bool,
    // ) -> Result<ForStatement, ParserErrorInfo> {
    //     let right = self.parse_expression()?;
    //     self.expect_and_consume_token(TokenKind::CloseParen)?;
    //     let body = self.parse_statement(true)?;
    //
    //     let node = Node::new(start, body.node().end);
    //     let left = match head {
    //         ForHead::VarDecl(decl) => ForLeft::VariableDeclaration(decl),
    //         ForHead::Expr(expr) => ForLeft::Expression(expr),
    //         ForHead::Empty => throw_error!(InternalError),
    //     };
    //
    //     if is_of {
    //         Ok(ForOfStatement {
    //             node,
    //             left,
    //             right,
    //             body,
    //         }
    //         .into())
    //     } else {
    //         Ok(ForInStatement {
    //             node,
    //             left,
    //             right,
    //             body,
    //         }
    //         .into())
    //     }
    // }

    /// Parses `while` loop
    fn parse_while_statement(&mut self) -> Result<WhileStatement, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "while" keyword token

        self.expect_and_consume_token(TokenKind::OpenParen)?;

        let test = self.parse_expression()?;

        self.expect_and_consume_token(TokenKind::CloseParen)?;

        let body = self.parse_statement(true)?;

        Ok(WhileStatement {
            node: Node::new(start_pos, body.node().end),
            test,
            body,
        })
    }

    /// Parses a `return` statement.
    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        let mut end_pos = self.current_token.end;
        self.advance(); // Consume "return" token

        // TODO: Fix this case
        let argument = if !self.current_token.is(TokenKind::Semi) {
            let expr = self.parse_expression()?;
            end_pos = expr.node().end;
            Some(expr)
        } else {
            None
        };

        if self.current_token.is(TokenKind::Semi) {
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
        lhs: Pattern,
    ) -> Result<AssignmentExpression, ParserErrorInfo> {
        let operator = self.current_token.kind.as_assignment_op().unwrap();

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

        let mut elements: Vec<Option<Expression>> = Vec::new();

        loop {
            match self.current_token.kind {
                TokenKind::CloseBracket => break,
                TokenKind::Comma => {
                    self.advance();
                    elements.push(None);
                }
                _ => {
                    let expr = self.parse_expression()?;
                    elements.push(Some(expr));
                }
            }
        }

        let arr = ArrayExpression {
            node: Node::new(start_pos, self.current_token.end), // Includes "[" and "]" tokens
            elements,
        };

        self.advance(); // Consume "]" token

        Ok(arr)
    }

    /// Parses an object literal, such as { a: 4 }
    fn parse_object_literal(&mut self) -> Result<ObjectExpression, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.advance(); // Consume "{" token

        let mut properties: Vec<ObjectExpressionProperty> = Vec::new();

        loop {
            if self.current_token.is(TokenKind::CloseBrace) {
                break;
            }

            let property: ObjectExpressionProperty = match self.current_token.kind {
                TokenKind::String => {
                    let key = Expression::Literal(
                        Literal {
                            node: Node::new(self.current_token.start, self.current_token.end),
                            value: LiteralValue::String(self.current_token.value.consume_string()),
                        }
                        .into(),
                    );
                    self.advance(); // Consume String token
                    self.expect_and_consume_token(TokenKind::Colon)?;
                    let value = self.parse_expression()?;

                    Property {
                        node: Node::new(key.node().start, value.node().end),
                        key,
                        value,
                        kind: PropertyKind::Init,
                        method: false,
                        shorthand: false,
                        computed: false,
                    }
                    .into()
                }
                // TokenKind::Identifier => {
                //     match self.lexer.peek_token().kind {
                //         TokenKind::OpenParen => ObjectItem::Method(self.parse_method_definition()?),
                //         TokenKind::Colon => {
                //             let key = Identifier {
                //                 node: Node::new(self.current_token.start, self.current_token.end),
                //                 name: self.current_token.value.expect_identifier().clone(),
                //             }
                //             .into();
                //             self.advance(); // Consume Identifier token
                //             self.advance(); // Consume ":" token
                //             let value = self.parse_expression()?;
                //             ObjectItem::KV(KV { key, value })
                //         }
                //         _ => {
                //             let id = Identifier {
                //                 node: Node::new(self.current_token.start, self.current_token.end),
                //                 name: self.current_token.value.expect_identifier().clone(),
                //             };
                //             self.advance(); // Consume Identifier token
                //             ObjectItem::Identifier(id)
                //         }
                //     }
                // }
                // TokenKind::Keyword => match self.current_token.value.expect_keyword() {
                //     Keyword::Async => ObjectItem::Method(self.parse_method_definition()?),
                //     Keyword::StringType
                //     | Keyword::NumberType
                //     | Keyword::BooleanType
                //     | Keyword::Type => {
                //         let key = StringLiteral {
                //             node: Node::new(self.current_token.start, self.current_token.end),
                //             value: self.current_token.value.expect_keyword().to_string(),
                //         }
                //         .into();
                //         self.advance(); // Consume keyword token
                //         self.expect_and_consume_token(TokenKind::Colon)?;
                //         let value = self.parse_expression()?;
                //         ObjectItem::KV(KV { key, value })
                //     }
                //     _ => throw_error!(InvalidToken),
                // },
                // TokenKind::OpenBracket => {
                //     let start_pos = self.current_token.start;
                //
                //     self.advance(); // Consume "[" token
                //     let expression = self.parse_expression()?;
                //     self.expect_token_kind(TokenKind::CloseBracket)?;
                //     let key = Key::ComputedProperty(ComputedProperty {
                //         node: Node::new(start_pos, self.current_token.end),
                //         expression,
                //     });
                //     self.advance(); // Consume "]" token
                //
                //     self.expect_and_consume_token(TokenKind::Colon)?;
                //
                //     let value = self.parse_expression()?;
                //
                //     ObjectItem::KV(KV { key, value })
                // }
                // TokenKind::Dot => throw_error!(Todo),
                _ => throw_error!(InvalidToken),
            };

            properties.push(property);

            match self.current_token.kind {
                TokenKind::CloseBrace => break,
                TokenKind::Comma => self.advance(),
                _ => throw_error!(InvalidToken),
            };
        }

        let obj = ObjectExpression {
            node: Node::new(start_pos, self.current_token.end), // Include "{" and "}" tokens
            properties,
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
        todo!();
        // let mut left = match left {
        //     Some(expr) => expr,
        //     None => self.parse_primary_expression()?,
        // };
        // let start_pos = left.node().start;
        //
        // while let Some(op_precedence) = self.current_token.kind.get_precedence() {
        //     if op_precedence <= precedence {
        //         break;
        //     }
        //
        //     let operator = self.current_token.kind.as_operator().unwrap(); // Safe unwrap because of .get_operator_precedence()
        //     self.advance(); // Consume operator token
        //
        //     let mut right = self.parse_primary_expression()?;
        //
        //     if let Some(next_precedence) = self.current_token.kind.get_operator_precedence() {
        //         if next_precedence > op_precedence {
        //             right = self
        //                 .parse_binary_expression(Some(right), op_precedence)?
        //                 .into();
        //         }
        //     }
        //
        //     left = BinaryExpression {
        //         node: Node::new(start_pos, right.node().end),
        //         left,
        //         right,
        //         operator,
        //     }
        //     .into();
        // }
        //
        // if let Expression::BinaryExpression(bin_exp) = left {
        //     Ok(*bin_exp)
        // } else {
        //     throw_error!(InvalidToken) // Only happens if we never enter the while loop
        // }
    }

    /// Parses a function or method call
    fn parse_call_expression(
        &mut self,
        callee: CallExpressionCallee,
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
            optional: false, // TODO: implement optional chaining
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
        match self.current_token.kind {
            TokenKind::Dot => {
                self.advance(); // Consume "." token

                self.expect_token_kind(TokenKind::Identifier)?;
                let property = Identifier {
                    node: Node::new(self.current_token.start, self.current_token.end),
                    name: self.current_token.value.expect_identifier().clone(),
                };
                self.advance(); // Consume Identifier token

                Ok(MemberExpression {
                    node: Node::new(object.node().start, property.node.end),
                    optional: false, // TODO: implement optional chaining
                    object,
                    property: MemberExpressionProperty::Expression(property.into()),
                    computed: false,
                })
            }
            TokenKind::OpenBracket => {
                self.advance(); // Consume "[" token

                let property = self.parse_expression()?;

                self.expect_and_consume_token(TokenKind::CloseBracket)?;

                Ok(MemberExpression {
                    node: Node::new(object.node().start, property.node().end),
                    optional: false, // TODO: implement optional chaining
                    object,
                    property: MemberExpressionProperty::Expression(property),
                    computed: true,
                })
            }
            _ => unreachable!(),
        }
    }

    /// Parses literal values (e.g., strings, numbers, booleans).
    fn parse_literal(&mut self) -> Result<Literal, ParserErrorInfo> {
        let node = Node::new(self.current_token.start, self.current_token.end);
        let value: LiteralValue;

        match self.current_token.kind {
            TokenKind::String => {
                value = LiteralValue::String(self.current_token.value.consume_string());
            }
            TokenKind::Number => {
                value = LiteralValue::Number(self.current_token.value.expect_number());
            }
            TokenKind::Boolean => {
                value = LiteralValue::Boolean(self.current_token.value.expect_boolean());
            }
            TokenKind::Null => {
                value = LiteralValue::Null;
            }
            _ => throw_error!(InvalidToken),
        }

        self.advance();
        Ok(Literal { node, value })
    }

    /// Parses TypeScript `enum` declarations.
    // fn parse_enum_declaration(
    //     &mut self,
    //     is_const: bool,
    //     is_declare: bool,
    // ) -> Result<EnumStatement, ParserErrorInfo> {
    //     let start_pos = self.current_token.start;
    //
    //     if self.current_token.value.expect_keyword() != Keyword::Enum {
    //         self.advance(); // Consume "enum" or "declare" keyword tok
    //     }
    //
    //     self.advance(); // Consume "enum" keyword token
    //
    //     self.expect_token_kind(TokenKind::Identifier)?;
    //     let id = Identifier {
    //         node: Node::new(self.current_token.start, self.current_token.end),
    //         name: self.current_token.value.expect_identifier().clone(),
    //     };
    //     self.advance(); // Consume Identifier token
    //
    //     self.expect_and_consume_token(TokenKind::OpenBrace)?;
    //
    //     let mut members = Vec::new();
    //
    //     loop {
    //         if self.current_token.is(TokenKind::CloseBrace) {
    //             break;
    //         }
    //         let start = self.current_token.start;
    //
    //         self.expect_token_kind(TokenKind::Identifier)?;
    //         let id = Identifier {
    //             node: Node::new(self.current_token.start, self.current_token.end),
    //             name: self.current_token.value.expect_identifier().clone(),
    //         };
    //         self.advance(); // Consume Identifier token
    //
    //         let (end_pos, init) = if self.current_token.is(TokenKind::Equals) {
    //             self.advance(); // Consume "=" token
    //             let expr = self.parse_expression()?;
    //             (expr.node().end, Some(expr))
    //         } else {
    //             (id.node.end, None)
    //         };
    //
    //         members.push(EnumMember {
    //             node: Node::new(start, end_pos),
    //             id,
    //             init,
    //         });
    //
    //         match self.current_token.kind {
    //             TokenKind::CloseBrace => break,
    //             TokenKind::Comma => self.advance(), // Consume "," token
    //             _ => throw_error!(InvalidToken),
    //         };
    //     }
    //
    //     let end_pos = self.current_token.end;
    //     self.advance(); // Consume "}" token
    //
    //     Ok(EnumStatement {
    //         node: Node::new(start_pos, end_pos),
    //         is_declare,
    //         is_const,
    //         id,
    //         members,
    //     })
    // }

    fn parse_type_parameter_declaration(
        &mut self,
    ) -> Result<TypeParameterDeclaration, ParserErrorInfo> {
        let start_pos = self.current_token.start;
        self.expect_and_consume_token(TokenKind::Lt)?;

        let mut parameters = Vec::new();

        loop {
            match self.current_token.kind {
                TokenKind::Comma if parameters.len() != 0 => {
                    self.advance(); // Consume "," token
                }
                TokenKind::Gt => break,
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
                    TokenKind::Lt => {
                        self.advance(); // Consume "<" token

                        let mut type_params = Vec::new();

                        loop {
                            match self.current_token.kind {
                                TokenKind::Gt => break,
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
                        self.expect_and_consume_token(TokenKind::Gt)?;
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

    fn parse_new_expression(&mut self) -> Result<NewExpression, ParserErrorInfo> {
        throw_error!(Todo)
    }
}
