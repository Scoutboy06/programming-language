use parser::{
    expressions::{
        ArrayExpression, BooleanLiteral, ComputedProperty, Identifier, NumberLiteral,
        ObjectExpression, ParenthesisExpression, StringLiteral, VariableKind, KV,
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

#[test]
fn object_literal() {
    let code = "var obj = { k1: 101, k2: \"2\", k3: true };";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: code.between_incl(("var", 0), ("};", 0)),
            kind: VariableKind::Var,
            declarations: vec![VariableDeclarator {
                node: code.between_incl(("obj", 0), ("}", 0)),
                id: Identifier {
                    node: code.node("obj", 0),
                    name: "obj".into(),
                },
                type_annotation: None,
                init: Some(
                    ObjectExpression {
                        node: code.between_incl(("{", 0), ("}", 0)),
                        items: vec![
                            KV {
                                key: Identifier {
                                    node: code.node("k1", 0),
                                    name: "k1".into(),
                                }
                                .into(),
                                value: NumberLiteral {
                                    node: code.node("101", 0),
                                    value: 101.0,
                                }
                                .into(),
                            }
                            .into(),
                            KV {
                                key: Identifier {
                                    node: code.node("k2", 0),
                                    name: "k2".into(),
                                }
                                .into(),
                                value: StringLiteral {
                                    node: code.node("\"2\"", 0),
                                    value: "\"2\"".into(),
                                }
                                .into(),
                            }
                            .into(),
                            KV {
                                key: Identifier {
                                    node: code.node("k3", 0),
                                    name: "k3".into(),
                                }
                                .into(),
                                value: BooleanLiteral {
                                    node: code.node("true", 0),
                                    value: true,
                                }
                                .into(),
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
fn object_shorthand_property() {
    let code = "var obj = { name, age };";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: code.between_incl(("var", 0), ("};", 0)),
            kind: VariableKind::Var,
            declarations: vec![VariableDeclarator {
                node: code.between_incl(("obj", 0), ("}", 0)),
                id: Identifier {
                    node: code.node("obj", 0),
                    name: "obj".into(),
                },
                type_annotation: None,
                init: Some(
                    ObjectExpression {
                        node: code.between_incl(("{", 0), ("}", 0)),
                        items: [
                            Identifier {
                                node: code.node("name", 0),
                                name: "name".into(),
                            }
                            .into(),
                            Identifier {
                                node: code.node("age", 0),
                                name: "age".into(),
                            }
                            .into(),
                        ]
                        .to_vec(),
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
fn object_computed_property() {
    let code = "var obj = { [key]: value, [123]: 456, [\"hello\"]: \"world\" };";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: code.between_incl(("var", 0), ("};", 0)),
            kind: VariableKind::Var,
            declarations: vec![VariableDeclarator {
                node: code.between_incl(("obj", 0), ("}", 0)),
                id: Identifier {
                    node: code.node("obj", 0),
                    name: "obj".into(),
                },
                type_annotation: None,
                init: Some(
                    ObjectExpression {
                        node: code.between_incl(("{", 0), ("}", 0)),
                        items: vec![
                            KV {
                                key: ComputedProperty {
                                    node: code.node("[key]", 0),
                                    expression: Identifier {
                                        node: code.node("key", 0),
                                        name: "key".into(),
                                    }
                                    .into(),
                                }
                                .into(),
                                value: Identifier {
                                    node: code.node("value", 0),
                                    name: "value".into(),
                                }
                                .into(),
                            }
                            .into(),
                            KV {
                                key: ComputedProperty {
                                    node: code.node("[123]", 0),
                                    expression: NumberLiteral {
                                        node: code.node("123", 0),
                                        value: 123.0,
                                    }
                                    .into(),
                                }
                                .into(),
                                value: NumberLiteral {
                                    node: code.node("456", 0),
                                    value: 456.0,
                                }
                                .into(),
                            }
                            .into(),
                            KV {
                                key: ComputedProperty {
                                    node: code.node("[\"hello\"]", 0),
                                    expression: StringLiteral {
                                        node: code.node("\"hello\"", 0),
                                        value: "\"hello\"".into(),
                                    }
                                    .into(),
                                }
                                .into(),
                                value: StringLiteral {
                                    node: code.node("\"world\"", 0),
                                    value: "\"world\"".into(),
                                }
                                .into(),
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
