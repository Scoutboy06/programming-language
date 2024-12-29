use crate::expressions::{
    ArrayExpression, BinaryExpression, BinaryOperation, BooleanLiteral, CallExpression, Expression,
    Literal, MemberExpression, NullLiteral, NumberLiteral, ObjectExpression, StringLiteral,
    UnaryExpression, KV,
};
use crate::nodes::{program::Program, Node};
use crate::statements::{
    BlockStatement, FunctionDeclaration, Identifier, Statement, VariableDeclaration,
    VariableDeclarator, VariableKind,
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

            let statement = self.parse_statement();

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
        self.prev_token_end = self.current_token.end;

        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_token_kind(&self, kind: TokenKind) -> Result<(), ErrorKind> {
        if self.current_token.kind == kind {
            Ok(())
        } else {
            Err(ErrorKind::InvalidToken)
        }
    }

    fn expect_and_consume_token(&mut self, kind: TokenKind) -> Result<(), ErrorKind> {
        if self.current_token.kind == kind {
            self.advance();
            Ok(())
        } else {
            Err(ErrorKind::InvalidToken)
        }
    }

    /// Parses a single statement (e.g., variable declarations, control flow statements, function definitions).
    fn parse_statement(&mut self) -> Result<Statement, ErrorKind> {
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
            _ => {
                let expr = self.parse_expression()?;
                Ok(Statement::ExpressionStatement(expr.into()))
            }
        }
    }

    /// Parses an expression (e.g., arithmetic operations, logical operations, or function calls).
    fn parse_expression(&mut self) -> Result<Expression, ErrorKind> {
        let mut lhs = self.parse_primary_expression()?;

        loop {
            match self.current_token.kind {
                TokenKind::OpenParen => {
                    lhs = Expression::CallExpression(Box::new(self.parse_call_expression(lhs)?));
                }
                TokenKind::Dot | TokenKind::OpenBracket => {
                    let mem_exp = self.parse_member_expression(lhs)?;
                    lhs = Expression::MemberExpression(Box::new(mem_exp));
                }
                _ if self.current_token.kind.is_operator() => todo!(),
                _ => break,
            };
        }

        Ok(lhs)
    }

    /// Parses literal values, such as numbers, strings, booleans, null, arrays, or objects.
    fn parse_primary_expression(&mut self) -> Result<Expression, ErrorKind> {
        match self.current_token.kind {
            TokenKind::String | TokenKind::Boolean | TokenKind::Number | TokenKind::Null => {
                Ok(Expression::Literal(Box::new(self.parse_literal()?)))
            }
            TokenKind::Identifier => {
                let expr = Expression::Identifier(
                    Identifier {
                        node: Node::new(self.current_token.start, self.current_token.end),
                        name: self.current_token.value.expect_string().clone(),
                    }
                    .into(),
                );
                self.advance(); // Consume Identifier token
                Ok(expr)
            }
            TokenKind::OpenParen => {
                self.advance(); // Consume "(" token
                let expr = self.parse_expression()?;
                if self.current_token.kind != TokenKind::CloseParen {
                    return Err(ErrorKind::ExpectedClosingParen);
                }
                self.advance(); // Consume ")" token
                Ok(expr)
            }
            TokenKind::OpenBracket => {
                let arr = self.parse_array_literal()?;
                Ok(Expression::ArrayExpression(Box::new(arr)))
            }
            TokenKind::OpenBrace => {
                let obj = self.parse_object_literal()?;
                Ok(Expression::ObjectExpression(Box::new(obj)))
            }
            _ => Err(ErrorKind::InvalidToken),
        }
    }

    /// Parses a block of code, usually enclosed by `{}`.
    fn parse_block_statement(&mut self) -> Result<BlockStatement, ErrorKind> {
        todo!()
    }

    /// Parses a variable declaration, including `let`, `const`, or `var` keywords.
    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration, ErrorKind> {
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

            self.expect_token_kind(TokenKind::Identifier)?;
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

    /// Parses a function declaration, including its name, parameters, and body.
    fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration, ErrorKind> {
        todo!()
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
    fn parse_if_statement(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses a `for` loop, including `for-in` and `for-of` loops.
    fn parse_for_statement(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses `while` loop
    fn parse_while_statement(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses a `switch` statement, including its cases and default clause.
    fn parse_switch_statement(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses a `return` statement.
    fn parse_return_statement(&mut self) -> Result<(), ErrorKind> {
        todo!()
    }

    /// Parses an assignment operation, such as `=` or compound assignments (e.g., `+=`, `-=`).
    fn parse_assignment_statement(&mut self) -> Result<(), ErrorKind> {
        todo!()
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

            self.expect_and_consume_token(TokenKind::Colon)?;

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
    fn parse_binary_expression(&mut self) -> Result<BinaryExpression, ErrorKind> {
        todo!()
    }

    /// Parses unary operations (e.g., `!`, `-`).
    fn parse_unary_expression(&mut self) -> Result<UnaryExpression, ErrorKind> {
        todo!()
    }

    /// Parses a function or method call
    fn parse_call_expression(&mut self, callee: Expression) -> Result<CallExpression, ErrorKind> {
        self.expect_and_consume_token(TokenKind::OpenParen)?;

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
        match self.current_token.kind {
            TokenKind::Dot => {}
            TokenKind::OpenBracket => todo!("Computed property"),
            _ => unreachable!(),
        }

        todo!()
    }

    /// Parses literal values (e.g., strings, numbers, booleans).
    fn parse_literal(&mut self) -> Result<Literal, ErrorKind> {
        match self.current_token.kind {
            TokenKind::String => {
                let s = StringLiteral {
                    node: Node::new(self.current_token.start, self.current_token.end),
                    value: self.current_token.value.expect_string().clone(),
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
    fn parse_type_annotation(&mut self) -> Result<(), ErrorKind> {
        todo!()
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
