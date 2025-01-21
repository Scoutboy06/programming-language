use parser::{
    expressions::{
        ArrayExpression, Identifier, NumberLiteral, ParenthesisExpression, VariableKind,
    },
    nodes::{program::Program, Node},
    statements::{VariableDeclaration, VariableDeclarator},
    Parser,
};
use pretty_assertions::assert_eq;
mod helpers;
use helpers::*;

#[test]
fn assignment_number_literal() {
    let code = "let a = 50.5";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: Node::new(0, code.len()),
            declarations: vec![VariableDeclarator {
                node: code.between_incl(("a", 0), ("50.5", 0)),
                id: Identifier {
                    node: code.node("a", 0),
                    name: "a".into(),
                },
                type_annotation: None,
                init: Some(
                    NumberLiteral {
                        node: code.node("50.5", 0),
                        value: 50.5,
                    }
                    .into(),
                ),
            }],
            kind: VariableKind::Let,
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn assignment_paren_literal() {
    let code = "const a = (50.5)";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: Node::new(0, code.len()),
            declarations: vec![VariableDeclarator {
                node: code.node("a = (50.5)", 0),
                id: Identifier {
                    node: code.node("a", 0),
                    name: "a".into(),
                },
                type_annotation: None,
                init: Some(
                    ParenthesisExpression {
                        node: code.node("(50.5)", 0),
                        expression: NumberLiteral {
                            node: code.node("50.5", 0),
                            value: 50.5,
                        }
                        .into(),
                    }
                    .into(),
                ),
            }],
            kind: VariableKind::Const,
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn array_literal() {
    let code = "const nums = [1, 2, 3, 4];";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: Node::new(0, code.len()),
            kind: VariableKind::Const,
            declarations: vec![VariableDeclarator {
                node: code.node("nums = [1, 2, 3, 4]", 0),
                id: Identifier {
                    node: code.node("nums", 0),
                    name: "nums".into(),
                },
                type_annotation: None,
                init: Some(
                    ArrayExpression {
                        node: code.between_incl(("[", 0), ("]", 0)),
                        items: vec![
                            NumberLiteral {
                                node: code.node("1", 0),
                                value: 1.0,
                            }
                            .into(),
                            NumberLiteral {
                                node: code.node("2", 0),
                                value: 2.0,
                            }
                            .into(),
                            NumberLiteral {
                                node: code.node("3", 0),
                                value: 3.0,
                            }
                            .into(),
                            NumberLiteral {
                                node: code.node("4", 0),
                                value: 4.0,
                            }
                            .into(),
                        ],
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
fn nested_array() {
    let code = "const nums = [[1], [2]];";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: Node::new(0, code.len()),
            kind: VariableKind::Const,
            declarations: vec![VariableDeclarator {
                node: code.node("nums = [[1], [2]]", 0),
                id: Identifier {
                    node: code.node("nums", 0),
                    name: "nums".into(),
                },
                type_annotation: None,
                init: Some(
                    ArrayExpression {
                        node: code.between_incl(("[[", 0), ("]]", 0)),
                        items: vec![
                            ArrayExpression {
                                node: code.node("[1]", 0),
                                items: vec![NumberLiteral {
                                    node: code.node("1", 0),
                                    value: 1.0,
                                }
                                .into()],
                            }
                            .into(),
                            ArrayExpression {
                                node: code.node("[2]", 0),
                                items: vec![NumberLiteral {
                                    node: code.node("2", 0),
                                    value: 2.0,
                                }
                                .into()],
                            }
                            .into(),
                        ],
                    }
                    .into(),
                ),
            }],
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}
