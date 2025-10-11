use lexer::Operator;
use parser::{
    expressions::{BinaryExpression, BooleanLiteral, Identifier, NumberLiteral},
    nodes::{program::Program, Node},
    statements::{BlockStatement, IfStatement, ReturnStatement},
    Parser,
};
use pretty_assertions::assert_eq;
mod helpers;
use helpers::NodeConstructor;

#[test]
fn if_statement() {
    let code = "if (val && true || false) {}";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![IfStatement {
            node: Node::new(0, code.len()),
            condition: BinaryExpression {
                node: code.between(("(", 0), (")", 0)),
                left: BinaryExpression {
                    node: code.node("val && true", 0),
                    left: Identifier {
                        node: code.node("val", 0),
                        name: "val".into(),
                    }
                    .into(),
                    right: BooleanLiteral {
                        node: code.node("true", 0),
                        value: true,
                    }
                    .into(),
                    operator: Operator::LogicalAnd,
                }
                .into(),
                right: BooleanLiteral {
                    node: code.node("false", 0),
                    value: false,
                }
                .into(),
                operator: Operator::LogicalOr,
            }
            .into(),
            body: BlockStatement {
                node: code.node("{}", 0),
                statements: Vec::new(),
            }
            .into(),
            consequent: None,
        }
        .into()],
    };

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

    assert_eq!(result.unwrap(), expected);
}

#[test]
fn if_else_statement() {
    let code = "if (val && true) {
        return true;
    } else {
        return 50.5;
    }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![IfStatement {
            node: Node::new(0, code.len()),
            condition: BinaryExpression {
                node: code.node("val && true", 0),
                left: Identifier {
                    node: code.node("val", 0),
                    name: "val".into(),
                }
                .into(),
                right: BooleanLiteral {
                    node: code.node("true", 0),
                    value: true,
                }
                .into(),
                operator: Operator::LogicalAnd,
            }
            .into(),
            body: BlockStatement {
                node: code.between_incl(("{", 0), ("}", 0)),
                statements: vec![ReturnStatement {
                    node: code.node("return true;", 0),
                    value: BooleanLiteral {
                        node: code.node("true", 1),
                        value: true,
                    }
                    .into(),
                }
                .into()],
            }
            .into(),
            consequent: Some(
                BlockStatement {
                    node: code.between_incl(("{", 1), ("}", 1)),
                    statements: vec![ReturnStatement {
                        node: code.node("return 50.5;", 0),
                        value: NumberLiteral {
                            node: code.node("50.5", 0),
                            value: 50.5,
                        }
                        .into(),
                    }
                    .into()],
                }
                .into(),
            ),
        }
        .into()],
    };

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

    assert_eq!(result.unwrap(), expected);
}

#[test]
fn if_if_else_else_statement() {
    let code = "if(foo && true) {}
    else if(false || bar) {}
    else {}";
    let mut parser = Parser::new(code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![IfStatement {
            node: Node::new(0, code.len()),
            condition: BinaryExpression {
                node: code.node("foo && true", 0),
                left: Identifier {
                    node: code.node("foo", 0),
                    name: "foo".into(),
                }
                .into(),
                right: BooleanLiteral {
                    node: code.node("true", 0),
                    value: true,
                }
                .into(),
                operator: Operator::LogicalAnd,
            }
            .into(),
            body: BlockStatement {
                node: code.node("{}", 0),
                statements: vec![],
            }
            .into(),
            consequent: Some(
                IfStatement {
                    node: code.between_incl(("if", 1), ("else {}", 0)),
                    condition: BinaryExpression {
                        node: code.node("false || bar", 0),
                        left: BooleanLiteral {
                            node: code.node("false", 0),
                            value: false,
                        }
                        .into(),
                        right: Identifier {
                            node: code.node("bar", 0),
                            name: "bar".into(),
                        }
                        .into(),
                        operator: Operator::LogicalOr,
                    }
                    .into(),
                    body: BlockStatement {
                        node: code.node("{}", 1),
                        statements: vec![],
                    }
                    .into(),
                    consequent: Some(
                        BlockStatement {
                            node: code.node("{}", 2),
                            statements: vec![],
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into()],
    };

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

    assert_eq!(result.unwrap(), expected);
}

#[test]
fn if_else_without_curly_braces() {
    let code = "
    if(foo)
        return true;
    else
        return false;";
    let mut parser = Parser::new(code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![IfStatement {
            node: code.between_incl(("if", 0), ("false;", 0)),
            condition: Identifier {
                node: code.node("foo", 0),
                name: "foo".into(),
            }
            .into(),
            body: ReturnStatement {
                node: code.node("return true;", 0),
                value: BooleanLiteral {
                    node: code.node("true", 0),
                    value: true,
                }
                .into(),
            }
            .into(),
            consequent: Some(
                ReturnStatement {
                    node: code.node("return false;", 0),
                    value: BooleanLiteral {
                        node: code.node("false", 0),
                        value: false,
                    }
                    .into(),
                }
                .into(),
            ),
        }
        .into()],
    };

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

    assert_eq!(result.unwrap(), expected);
}
