use lexer::TypeKeyword;
use parser::{
    expressions::{
        types::{KeywordType, TypeAnnotation},
        ArrayExpression, BooleanLiteral, CallExpression, ComputedProperty, Identifier,
        MemberExpression, Method, NumberLiteral, ObjectExpression, ParenthesisExpression,
        StringLiteral, VariableKind, KV,
    },
    nodes::{program::Program, Node},
    statements::{
        BlockStatement, ExpressionStatement, Parameter, VariableDeclaration, VariableDeclarator,
    },
    Parser,
};
use pretty_assertions::assert_eq;
mod helpers;
use helpers::NodeConstructor;

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

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

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

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

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

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

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

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

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

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

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

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

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
            declarations: [VariableDeclarator {
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
                        ]
                        .to_vec(),
                    }
                    .into(),
                ),
            }]
            .to_vec(),
        }
        .into()],
    };

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

    assert_eq!(result, Ok(expected));
}

#[test]
fn object_method() {
    let code = "var obj = { print(name: string) { console.log(name); } };";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: code.between_incl(("var", 0), ("};", 0)),
            kind: VariableKind::Var,
            declarations: vec![VariableDeclarator {
                node: code.between_incl(("obj", 0), ("}", 1)),
                id: Identifier {
                    node: code.node("obj", 0),
                    name: "obj".into(),
                },
                type_annotation: None,
                init: Some(
                    ObjectExpression {
                        node: code.between_incl(("{", 0), ("}", 1)),
                        items: [Method {
                            node: code.between_incl(("print", 0), ("}", 0)),
                            is_async: false,
                            is_generator: false,
                            id: Identifier {
                                node: code.node("print", 0),
                                name: "print".into(),
                            },
                            type_parameters: None,
                            parameters: [Parameter {
                                node: code.node("name: string", 0),
                                identifier: Identifier {
                                    node: code.node("name", 0),
                                    name: "name".into(),
                                },
                                type_annotation: Some(TypeAnnotation {
                                    node: code.node(": string", 0),
                                    type_value: KeywordType {
                                        node: code.node("string", 0),
                                        kind: TypeKeyword::String,
                                    }
                                    .into(),
                                }),
                                optional: false,
                            }
                            .into()]
                            .to_vec(),
                            return_type: None,
                            body: BlockStatement {
                                node: code.between_incl(("{", 1), ("}", 0)),
                                statements: [ExpressionStatement {
                                    node: code.node("console.log(name);", 0),
                                    expression: CallExpression {
                                        node: code.node("console.log(name)", 0),
                                        callee: MemberExpression {
                                            node: code.node("console.log", 0),
                                            object: Identifier {
                                                node: code.node("console", 0),
                                                name: "console".into(),
                                            }
                                            .into(),
                                            property: Identifier {
                                                node: code.node("log", 0),
                                                name: "log".into(),
                                            }
                                            .into(),
                                        }
                                        .into(),
                                        arguments: [Identifier {
                                            node: code.node("name", 1),
                                            name: "name".into(),
                                        }
                                        .into()]
                                        .to_vec(),
                                    }
                                    .into(),
                                }
                                .into()]
                                .to_vec(),
                            },
                        }
                        .into()]
                        .to_vec(),
                    }
                    .into(),
                ),
            }],
        }
        .into()],
    };

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

    assert_eq!(result, Ok(expected));
}
