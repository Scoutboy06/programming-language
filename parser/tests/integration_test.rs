use lexer::{ArithmeticOperator, Lexer, Token, TokenKind};
use parser::{
    expressions::{
        BinaryExpression, CallExpression, ComputedProperty, Expression, Literal, MemberExpression,
        MemberProperty, NumberLiteral, StringLiteral,
    },
    nodes::{program::Program, Node},
    statements::{Identifier, Statement, VariableDeclaration, VariableDeclarator, VariableKind},
    Parser,
};
use pretty_assertions::assert_eq;

#[test]
fn lexer_works() {
    let source = "let a = 3;";
    let mut lex = Lexer::new(source);
    let mut tok: Token;
    loop {
        tok = lex.next_token();
        if tok.kind == TokenKind::Eof {
            break;
        }
    }
}

#[test]
fn empty_program() {
    let source_code = "";
    let mut parser = Parser::new(&source_code);
    let result = parser.parse();

    assert_eq!(
        result,
        Ok(Program {
            node: Node::new(0, 0),
            shebang: None,
            body: vec![],
        })
    );
}

#[test]
fn assignment_number_literal() {
    let source_code = "let a = 50.5";
    let mut parser = Parser::new(&source_code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, source_code.len()),
        shebang: None,
        body: vec![Statement::VariableDeclaration(Box::new(
            VariableDeclaration {
                node: Node::new(0, source_code.len()),
                declarations: vec![VariableDeclarator {
                    node: Node::new(4, source_code.len()),
                    id: Identifier::new("a".into(), 4, 5),
                    init: Some(Expression::Literal(Box::new(Literal::Number(
                        NumberLiteral {
                            node: Node::new(8, source_code.len()),
                            value: 50.5,
                        },
                    )))),
                }],
                kind: VariableKind::Let,
            },
        ))],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn assignment_paren_literal() {
    let source_code = "const a = (50.5)";
    let mut parser = Parser::new(&source_code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, source_code.len()),
        shebang: None,
        body: vec![Statement::VariableDeclaration(Box::new(
            VariableDeclaration {
                node: Node::new(0, source_code.len()),
                declarations: vec![VariableDeclarator {
                    node: Node::new(6, 16),
                    id: Identifier::new("a".into(), 6, 7),
                    init: Some(
                        NumberLiteral {
                            node: Node::new(11, 15),
                            value: 50.5,
                        }
                        .into(),
                    ),
                }],
                kind: VariableKind::Const,
            },
        ))],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn binary_operation() {
    let source_code = "let y = 6 + 5 * x";
    let mut parser = Parser::new(&source_code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, source_code.len()),
        shebang: None,
        body: vec![Statement::VariableDeclaration(
            VariableDeclaration {
                node: Node::new(0, 17),
                kind: VariableKind::Let,
                declarations: vec![VariableDeclarator {
                    node: Node::new(4, 17),
                    id: Identifier {
                        node: Node::new(4, 5),
                        name: "y".into(),
                    },
                    init: Some(Expression::BinaryExpression(
                        BinaryExpression {
                            node: Node::new(8, 17),
                            operator: ArithmeticOperator::Plus,
                            left: NumberLiteral {
                                node: Node::new(8, 9),
                                value: 6.0,
                            }
                            .into(),
                            right: Expression::BinaryExpression(
                                BinaryExpression {
                                    node: Node::new(12, 17),
                                    operator: ArithmeticOperator::Mult,
                                    left: NumberLiteral {
                                        node: Node::new(12, 13),
                                        value: 5.0,
                                    }
                                    .into(),
                                    right: Identifier {
                                        node: Node::new(16, 17),
                                        name: "x".into(),
                                    }
                                    .into(),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    )),
                }],
            }
            .into(),
        )],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn function_call() {
    let source_code = "my_func(50.5, \"abc123\")";
    let mut parser = Parser::new(&source_code);
    let result = parser.parse();

    let expected = Ok(Program {
        node: Node::new(0, source_code.len()),
        shebang: None,
        body: vec![Statement::ExpressionStatement(
            Expression::CallExpression(
                CallExpression {
                    node: Node::new(0, source_code.len()),
                    callee: Expression::Identifier(
                        Identifier {
                            node: Node::new(0, 7),
                            name: "my_func".into(),
                        }
                        .into(),
                    ),
                    arguments: vec![
                        NumberLiteral {
                            node: Node::new(8, 12),
                            value: 50.5,
                        }
                        .into(),
                        StringLiteral {
                            node: Node::new(14, 22),
                            value: "abc123".into(),
                        }
                        .into(),
                    ],
                }
                .into(),
            )
            .into(),
        )],
    });

    assert_eq!(result, expected);
}

#[test]
fn member_expression_function_call() {
    let source_code = "console.log(50.5)";
    let mut parser = Parser::new(&source_code);
    let result = parser.parse();

    let expected = Ok(Program {
        node: Node::new(0, source_code.len()),
        shebang: None,
        body: vec![Statement::ExpressionStatement(
            Expression::CallExpression(
                CallExpression {
                    node: Node::new(0, 17),
                    callee: Expression::MemberExpression(
                        MemberExpression {
                            node: Node::new(0, 11),
                            object: Expression::Identifier(
                                Identifier {
                                    node: Node::new(0, 7),
                                    name: "console".into(),
                                }
                                .into(),
                            ),
                            property: MemberProperty::Identifier(Identifier {
                                node: Node::new(8, 11),
                                name: "log".into(),
                            }),
                        }
                        .into(),
                    ),
                    arguments: vec![NumberLiteral {
                        node: Node::new(12, 16),
                        value: 50.5,
                    }
                    .into()],
                }
                .into(),
            )
            .into(),
        )],
    });

    assert_eq!(result, expected);
}

#[test]
fn computed_member_expression() {
    let source_code = "console[\"log\"]";
    let mut parser = Parser::new(&source_code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, source_code.len()),
        shebang: None,
        body: vec![Statement::ExpressionStatement(
            Expression::MemberExpression(
                MemberExpression {
                    node: Node::new(0, source_code.len()),
                    object: Identifier {
                        node: Node::new(0, 7),
                        name: "console".into(),
                    }
                    .into(),
                    property: ComputedProperty {
                        node: Node::new(7, source_code.len()),
                        expression: StringLiteral {
                            node: Node::new(8, source_code.len() - 1),
                            value: "log".into(),
                        }
                        .into(),
                    }
                    .into(),
                }
                .into(),
            )
            .into(),
        )
        .into()],
    };

    assert_eq!(result, Ok(expected));
}
