use lexer::Operator;
use parser::{
    expressions::{
        literal::VariableKind, BinaryExpression, Identifier, NumberLiteral, UpdateExpression,
        UpdateOperator,
    },
    nodes::{program::Program, Node},
    statements::{
        BlockStatement, ExpressionStatement, ForInStatement, ForOfStatement, ForStatement,
        VariableDeclaration, VariableDeclarator,
    },
    Parser,
};
use pretty_assertions::assert_eq;
mod helpers;
use helpers::NodeConstructor;

#[test]
fn for_loop() {
    let code = "for(let i = 0; i < 10; i++) {}";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Ok(Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![ForStatement {
            node: code.node(&code, 0),
            initializer: Some(
                VariableDeclaration {
                    node: code.node("let i = 0", 0),
                    declarations: vec![VariableDeclarator {
                        node: code.node("i = 0", 0),
                        id: Identifier {
                            node: code.node("i", 0),
                            name: "i".into(),
                        },
                        init: Some(
                            NumberLiteral {
                                node: code.node("0", 0),
                                value: 0.0,
                            }
                            .into(),
                        ),
                        type_annotation: None,
                    }],
                    kind: VariableKind::Let,
                }
                .into(),
            ),
            condition: Some(
                BinaryExpression {
                    node: code.node("i < 10", 0),
                    operator: Operator::LessThan,
                    left: Identifier {
                        node: code.node("i", 1),
                        name: "i".into(),
                    }
                    .into(),
                    right: NumberLiteral {
                        node: code.node("10", 0),
                        value: 10.0,
                    }
                    .into(),
                }
                .into(),
            ),
            update: Some(
                ExpressionStatement {
                    node: code.node("i++", 0),
                    expression: UpdateExpression {
                        node: code.node("i++", 0),
                        operator: UpdateOperator::Increment,
                        argument: Identifier {
                            node: code.node("i", 2),
                            name: "i".into(),
                        }
                        .into(),
                        prefix: false,
                    }
                    .into(),
                }
                .into(),
            ),
            body: BlockStatement {
                node: code.node("{}", 0),
                statements: Vec::new(),
            }
            .into(),
        }
        .into()],
    });

    assert_eq!(result, expected);
}

#[test]
fn for_loop_without_initializer() {
    let code = "for(; i < 10; i++) {}";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Ok(Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![ForStatement {
            node: code.node(&code, 0),
            initializer: None,
            condition: Some(
                BinaryExpression {
                    node: code.node("i < 10", 0),
                    operator: Operator::LessThan,
                    left: Identifier {
                        node: code.node("i", 0),
                        name: "i".into(),
                    }
                    .into(),
                    right: NumberLiteral {
                        node: code.node("10", 0),
                        value: 10.0,
                    }
                    .into(),
                }
                .into(),
            ),
            update: Some(
                ExpressionStatement {
                    node: code.node("i++", 0),
                    expression: UpdateExpression {
                        node: code.node("i++", 0),
                        operator: UpdateOperator::Increment,
                        argument: Identifier {
                            node: code.node("i", 1),
                            name: "i".into(),
                        }
                        .into(),
                        prefix: false,
                    }
                    .into(),
                }
                .into(),
            ),
            body: BlockStatement {
                node: code.node("{}", 0),
                statements: Vec::new(),
            }
            .into(),
        }
        .into()],
    });

    assert_eq!(result, expected);
}

#[test]
fn for_loop_without_test() {
    let code = "for(let i = 0;; i++) {}";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Ok(Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![ForStatement {
            node: code.node(&code, 0),
            initializer: Some(
                VariableDeclaration {
                    node: code.node("let i = 0", 0),
                    declarations: vec![VariableDeclarator {
                        node: code.node("i = 0", 0),
                        id: Identifier {
                            node: code.node("i", 0),
                            name: "i".into(),
                        },
                        init: Some(
                            NumberLiteral {
                                node: code.node("0", 0),
                                value: 0.0,
                            }
                            .into(),
                        ),
                        type_annotation: None,
                    }],
                    kind: VariableKind::Let,
                }
                .into(),
            ),
            condition: None,
            update: Some(
                ExpressionStatement {
                    node: code.node("i++", 0),
                    expression: UpdateExpression {
                        node: code.node("i++", 0),
                        operator: UpdateOperator::Increment,
                        argument: Identifier {
                            node: code.node("i", 1),
                            name: "i".into(),
                        }
                        .into(),
                        prefix: false,
                    }
                    .into(),
                }
                .into(),
            ),
            body: BlockStatement {
                node: code.node("{}", 0),
                statements: Vec::new(),
            }
            .into(),
        }
        .into()],
    });

    assert_eq!(result, expected);
}

#[test]
fn for_loop_without_update() {
    let code = "for(let i = 0; i < 10;) {}";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Ok(Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![ForStatement {
            node: code.node(&code, 0),
            initializer: Some(
                VariableDeclaration {
                    node: code.node("let i = 0", 0),
                    declarations: vec![VariableDeclarator {
                        node: code.node("i = 0", 0),
                        id: Identifier {
                            node: code.node("i", 0),
                            name: "i".into(),
                        },
                        init: Some(
                            NumberLiteral {
                                node: code.node("0", 0),
                                value: 0.0,
                            }
                            .into(),
                        ),
                        type_annotation: None,
                    }],
                    kind: VariableKind::Let,
                }
                .into(),
            ),
            condition: Some(
                BinaryExpression {
                    node: code.node("i < 10", 0),
                    operator: Operator::LessThan,
                    left: Identifier {
                        node: code.node("i", 1),
                        name: "i".into(),
                    }
                    .into(),
                    right: NumberLiteral {
                        node: code.node("10", 0),
                        value: 10.0,
                    }
                    .into(),
                }
                .into(),
            ),
            update: None,
            body: BlockStatement {
                node: code.node("{}", 0),
                statements: Vec::new(),
            }
            .into(),
        }
        .into()],
    });

    assert_eq!(result, expected);
}

#[test]
fn for_in_loop() {
    let code = "for(let key in obj) {}";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Ok(Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![ForInStatement {
            node: code.node(&code, 0),
            left: Identifier {
                node: code.node("let key", 0),
                name: "key".into(),
            }
            .into(),
            right: Identifier {
                node: code.node("obj", 0),
                name: "obj".into(),
            }
            .into(),
            body: BlockStatement {
                node: code.node("{}", 0),
                statements: Vec::new(),
            }
            .into(),
        }
        .into()],
    });

    assert_eq!(result, expected);
}

#[test]
fn for_of_loop() {
    let code = "for(let key of obj) {}";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Ok(Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![ForOfStatement {
            node: code.node(&code, 0),
            left: Identifier {
                node: code.node("let key", 0),
                name: "key".into(),
            }
            .into(),
            right: Identifier {
                node: code.node("obj", 0),
                name: "obj".into(),
            }
            .into(),
            body: BlockStatement {
                node: code.node("{}", 0),
                statements: Vec::new(),
            }
            .into(),
        }
        .into()],
    });

    assert_eq!(result, expected);
}
