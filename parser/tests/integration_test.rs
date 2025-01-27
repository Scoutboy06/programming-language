use lexer::{Lexer, Operator, Token, TokenKind};
use parser::{
    expressions::{
        AssignmentExpression, BinaryExpression, CallExpression, ComputedProperty, Identifier,
        MemberExpression, NumberLiteral, StringLiteral, UpdateExpression, UpdateOperator,
        VariableKind,
    },
    nodes::{program::Program, Node},
    statements::{
        BlockStatement, ExpressionStatement, ForStatement, VariableDeclaration, VariableDeclarator,
        WhileStatement,
    },
    Parser,
};
use pretty_assertions::assert_eq;
mod helpers;
use helpers::NodeConstructor;

#[test]
fn lexer_works() {
    let source = "let a = 3;";
    let mut lex = Lexer::new(source);
    let mut tok: Token;
    loop {
        tok = lex.next_token();
        if tok.is(TokenKind::Eof) {
            break;
        }
    }
}

#[test]
fn empty_program() {
    let code = "";
    let mut parser = Parser::new(&code);
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
fn binary_operation() {
    let code = "let y = 6 + 5 * x";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: code.node("let y = 6 + 5 * x", 0),
            kind: VariableKind::Let,
            declarations: vec![VariableDeclarator {
                node: code.node("y = 6 + 5 * x", 0),
                type_annotation: None,
                id: Identifier {
                    node: code.node("y", 0),
                    name: "y".into(),
                },
                init: Some(
                    BinaryExpression {
                        node: code.node("6 + 5 * x", 0),
                        operator: Operator::Plus,
                        left: NumberLiteral {
                            node: code.node("6", 0),
                            value: 6.0,
                        }
                        .into(),
                        right: BinaryExpression {
                            node: code.node("5 * x", 0),
                            operator: Operator::Mult,
                            left: NumberLiteral {
                                node: code.node("5", 0),
                                value: 5.0,
                            }
                            .into(),
                            right: Identifier {
                                node: code.node("x", 0),
                                name: "x".into(),
                            }
                            .into(),
                        }
                        .into(),
                    }
                    .into(),
                ),
            }],
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn computed_member_expression() {
    let code = "console[\"log\"]";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![ExpressionStatement {
            node: Node::new(0, code.len()),
            expression: MemberExpression {
                node: Node::new(0, code.len()),
                object: Identifier {
                    node: code.node("console", 0),
                    name: "console".into(),
                }
                .into(),
                property: ComputedProperty {
                    node: code.node("[\"log\"]", 0),
                    expression: StringLiteral {
                        node: code.node("\"log\"", 0),
                        value: "\"log\"".into(),
                    }
                    .into(),
                }
                .into(),
            }
            .into(),
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn assignment_statement() {
    let code = "foo -= 50.5; bar += \"World\";";
    let mut parser = Parser::new(code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![
            ExpressionStatement {
                node: code.node("foo -= 50.5;", 0),
                expression: AssignmentExpression {
                    node: code.node("foo -= 50.5", 0),
                    left: Identifier {
                        node: code.node("foo", 0),
                        name: "foo".into(),
                    }
                    .into(),
                    right: NumberLiteral {
                        node: code.node("50.5", 0),
                        value: 50.5,
                    }
                    .into(),
                    operator: Operator::MinusEquals,
                }
                .into(),
            }
            .into(),
            ExpressionStatement {
                node: code.node("bar += \"World\";", 0),
                expression: AssignmentExpression {
                    node: code.node("bar += \"World\"", 0),
                    left: Identifier {
                        node: code.node("bar", 0),
                        name: "bar".into(),
                    }
                    .into(),
                    right: StringLiteral {
                        node: code.node("\"World\"", 0),
                        value: "\"World\"".into(),
                    }
                    .into(),
                    operator: Operator::PlusEquals,
                }
                .into(),
            }
            .into(),
        ],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn while_loop() {
    let code = "while (foo <= bar) { baz(1); }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![WhileStatement {
            node: code.between_incl(("while", 0), ("}", 0)),
            condition: BinaryExpression {
                node: code.node("foo <= bar", 0),
                left: Identifier {
                    node: code.node("foo", 0),
                    name: "foo".into(),
                }
                .into(),
                right: Identifier {
                    node: code.node("bar", 0),
                    name: "bar".into(),
                }
                .into(),
                operator: Operator::LessOrEqualsThan,
            }
            .into(),
            body: BlockStatement {
                node: code.between_incl(("{", 0), ("}", 0)),
                statements: vec![ExpressionStatement {
                    node: code.node("baz(1);", 0),
                    expression: CallExpression {
                        node: code.node("baz(1)", 0),
                        callee: Identifier {
                            node: code.node("baz", 0),
                            name: "baz".into(),
                        }
                        .into(),
                        arguments: vec![NumberLiteral {
                            node: code.node("1", 0),
                            value: 1.0,
                        }
                        .into()],
                    }
                    .into(),
                }
                .into()],
            }
            .into(),
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn update_expression() {
    let code = "i++";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: code.node("i++", 0),
        shebang: None,
        body: vec![ExpressionStatement {
            node: code.node("i++", 0),
            expression: UpdateExpression {
                node: code.node("i++", 0),
                argument: Identifier {
                    node: code.node("i", 0),
                    name: "i".into(),
                }
                .into(),
                operator: UpdateOperator::Increment,
                prefix: false,
            }
            .into(),
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn for_loop() {
    let code = "for (let i = 0; i < code.length; i++) { bar(1); }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![ForStatement {
            node: Node::new(0, code.len()),
            initializer: VariableDeclaration {
                node: code.node("let i = 0", 0),
                declarations: vec![VariableDeclarator {
                    node: code.node("i = 0", 0),
                    id: Identifier {
                        node: code.node("i", 0),
                        name: "i".into(),
                    }
                    .into(),
                    type_annotation: None,
                    init: Some(
                        NumberLiteral {
                            node: code.node("0", 0),
                            value: 0.0,
                        }
                        .into(),
                    ),
                }],
                kind: VariableKind::Let,
            }
            .into(),
            condition: BinaryExpression {
                node: code.node("i < code.length", 0),
                left: Identifier {
                    node: code.node("i", 1),
                    name: "i".into(),
                }
                .into(),
                right: MemberExpression {
                    node: code.node("code.length", 0),
                    object: Identifier {
                        node: code.node("code", 0),
                        name: "code".into(),
                    }
                    .into(),
                    property: Identifier {
                        node: code.node("length", 0),
                        name: "length".into(),
                    }
                    .into(),
                }
                .into(),
                operator: Operator::LessThan,
            }
            .into(),
            update: ExpressionStatement {
                node: code.node("i++", 0),
                expression: UpdateExpression {
                    node: code.node("i++", 0),
                    argument: Identifier {
                        node: code.node("i", 2),
                        name: "i".into(),
                    }
                    .into(),
                    operator: UpdateOperator::Increment,
                    prefix: false,
                }
                .into(),
            }
            .into(),
            body: BlockStatement {
                node: code.between_incl(("{", 0), ("}", 0)),
                statements: vec![ExpressionStatement {
                    node: code.node("bar(1);", 0),
                    expression: CallExpression {
                        node: code.node("bar(1)", 0),
                        callee: Identifier {
                            node: code.node("bar", 0),
                            name: "bar".into(),
                        }
                        .into(),
                        arguments: vec![NumberLiteral {
                            node: code.node("1", 0),
                            value: 1.0,
                        }
                        .into()],
                    }
                    .into(),
                }
                .into()],
            }
            .into(),
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}
