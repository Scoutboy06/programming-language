use lexer::{Operator, TypeKeyword};
use parser::{
    expressions::{
        types::{KeywordType, TypeAnnotation},
        ArrowFunctionExpression, BinaryExpression, CallExpression, FunctionExpression, Identifier,
        MemberExpression, NumberLiteral, StringLiteral, VariableKind,
    },
    nodes::{program::Program, Node},
    statements::{
        BlockStatement, ExpressionStatement, FunctionDeclaration, Parameter, ReturnStatement,
        VariableDeclaration, VariableDeclarator,
    },
    Parser,
};
use pretty_assertions::assert_eq;
mod helpers;
use helpers::*;

#[test]
fn function_call() {
    let code = "my_func(50.5, \"abc123\")";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Ok(Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![ExpressionStatement {
            node: Node::new(0, code.len()),
            expression: CallExpression {
                node: Node::new(0, code.len()),
                callee: Identifier {
                    node: code.node("my_func", 0),
                    name: "my_func".into(),
                }
                .into(),
                arguments: vec![
                    NumberLiteral {
                        node: code.node("50.5", 0),
                        value: 50.5,
                    }
                    .into(),
                    StringLiteral {
                        node: code.node("\"abc123\"", 0),
                        value: "\"abc123\"".into(),
                    }
                    .into(),
                ],
            }
            .into(),
        }
        .into()],
    });

    assert_eq!(result, expected);
}

#[test]
fn member_expression_function_call() {
    let code = "console.log(50.5)";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Ok(Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![ExpressionStatement {
            node: Node::new(0, code.len()),
            expression: CallExpression {
                node: Node::new(0, code.len()),
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
                arguments: vec![NumberLiteral {
                    node: code.node("50.5", 0),
                    value: 50.5,
                }
                .into()],
            }
            .into(),
        }
        .into()],
    });

    assert_eq!(result, expected);
}

#[test]
fn function_declaration() {
    let code = "function add(n1: number, n2: number): number {
        return n1 + n2;
    }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![FunctionDeclaration {
            node: Node::new(0, code.len()),
            is_generator: false,
            is_async: false,
            id: Identifier {
                node: code.node("add", 0),
                name: "add".into(),
            },
            type_parameters: None,
            params: vec![
                Parameter {
                    node: code.node("n1: number", 0),
                    identifier: Identifier {
                        node: code.node("n1", 0),
                        name: "n1".into(),
                    },
                    type_annotation: Some(TypeAnnotation {
                        node: code.node(": number", 0),
                        type_value: KeywordType {
                            node: code.node("number", 0),
                            kind: TypeKeyword::Number,
                        }
                        .into(),
                    }),
                    optional: false,
                },
                Parameter {
                    node: code.node("n2: number", 0),
                    identifier: Identifier {
                        node: code.node("n2", 0),
                        name: "n2".into(),
                    },
                    type_annotation: Some(TypeAnnotation {
                        node: code.node(": number", 1),
                        type_value: KeywordType {
                            node: code.node("number", 1),
                            kind: TypeKeyword::Number,
                        }
                        .into(),
                    }),
                    optional: false,
                },
            ],
            return_type: Some(TypeAnnotation {
                node: code.node(": number", 2),
                type_value: KeywordType {
                    node: code.node("number", 2),
                    kind: TypeKeyword::Number,
                }
                .into(),
            }),
            body: BlockStatement {
                node: code.between_incl(("{", 0), ("}", 0)),
                statements: vec![ReturnStatement {
                    node: code.node("return n1 + n2;", 0),
                    value: BinaryExpression {
                        node: code.node("n1 + n2", 0),
                        operator: Operator::Plus,
                        left: Identifier {
                            node: code.node("n1", 1),
                            name: "n1".into(),
                        }
                        .into(),
                        right: Identifier {
                            node: code.node("n2", 1),
                            name: "n2".into(),
                        }
                        .into(),
                    }
                    .into(),
                }
                .into()],
            },
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn function_expression() {
    let code = "const sum = function(n1: number, n2: number): number {
        return n1 + n2;
    }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: Node::new(0, code.len()),
            kind: VariableKind::Const,
            declarations: vec![VariableDeclarator {
                node: code.between_incl(("sum", 0), ("}", 0)),
                id: Identifier {
                    node: code.node("sum", 0),
                    name: "sum".into(),
                },
                type_annotation: None,
                init: Some(
                    FunctionExpression {
                        node: code.between_incl(("function", 0), ("}", 0)),
                        is_async: false,
                        is_generator: false,
                        id: None,
                        params: vec![
                            Parameter {
                                node: code.node("n1: number", 0),
                                identifier: Identifier {
                                    node: code.node("n1", 0),
                                    name: "n1".into(),
                                },
                                type_annotation: Some(TypeAnnotation {
                                    node: code.node(": number", 0),
                                    type_value: KeywordType {
                                        node: code.node("number", 0),
                                        kind: TypeKeyword::Number,
                                    }
                                    .into(),
                                }),
                                optional: false,
                            },
                            Parameter {
                                node: code.node("n2: number", 0),
                                identifier: Identifier {
                                    node: code.node("n2", 0),
                                    name: "n2".into(),
                                },
                                type_annotation: Some(TypeAnnotation {
                                    node: code.node(": number", 1),
                                    type_value: KeywordType {
                                        node: code.node("number", 1),
                                        kind: TypeKeyword::Number,
                                    }
                                    .into(),
                                }),
                                optional: false,
                            },
                        ],
                        return_type: Some(TypeAnnotation {
                            node: code.node(": number", 2),
                            type_value: KeywordType {
                                node: code.node("number", 2),
                                kind: TypeKeyword::Number,
                            }
                            .into(),
                        }),
                        body: BlockStatement {
                            node: code.between_incl(("{", 0), ("}", 0)),
                            statements: vec![ReturnStatement {
                                node: code.node("return n1 + n2;", 0),
                                value: BinaryExpression {
                                    node: code.node("n1 + n2", 0),
                                    left: Identifier {
                                        node: code.node("n1", 1),
                                        name: "n1".into(),
                                    }
                                    .into(),
                                    right: Identifier {
                                        node: code.node("n2", 1),
                                        name: "n2".into(),
                                    }
                                    .into(),
                                    operator: Operator::Plus,
                                }
                                .into(),
                            }
                            .into()],
                        }
                        .into(),
                    }
                    .into(),
                ),
            }
            .into()],
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn arrow_function() {
    let code = "const sum = (n1: number, n2: number): number => n1 + n2;";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: Node::new(0, code.len()),
            kind: VariableKind::Const,
            declarations: vec![VariableDeclarator {
                node: code.between_incl(("sum", 0), ("n2", 1)),
                id: Identifier {
                    node: code.node("sum", 0),
                    name: "sum".into(),
                },
                type_annotation: None,
                init: Some(
                    ArrowFunctionExpression {
                        node: code.between_incl(("(n1", 0), ("n2", 1)),
                        parameters: vec![
                            Parameter {
                                node: code.node("n1: number", 0),
                                identifier: Identifier {
                                    node: code.node("n1", 0),
                                    name: "n1".into(),
                                },
                                type_annotation: Some(TypeAnnotation {
                                    node: code.node(": number", 0),
                                    type_value: KeywordType {
                                        node: code.node("number", 0),
                                        kind: TypeKeyword::Number,
                                    }
                                    .into(),
                                }),
                                optional: false,
                            },
                            Parameter {
                                node: code.node("n2: number", 0),
                                identifier: Identifier {
                                    node: code.node("n2", 0),
                                    name: "n2".into(),
                                },
                                type_annotation: Some(TypeAnnotation {
                                    node: code.node(": number", 1),
                                    type_value: KeywordType {
                                        node: code.node("number", 1),
                                        kind: TypeKeyword::Number,
                                    }
                                    .into(),
                                }),
                                optional: false,
                            },
                        ],
                        return_type: Some(TypeAnnotation {
                            node: code.node(": number", 2),
                            type_value: KeywordType {
                                node: code.node("number", 2),
                                kind: TypeKeyword::Number,
                            }
                            .into(),
                        }),
                        body: ExpressionStatement {
                            node: code.node("n1 + n2", 0),
                            expression: BinaryExpression {
                                node: code.node("n1 + n2", 0),
                                left: Identifier {
                                    node: code.node("n1", 1),
                                    name: "n1".into(),
                                }
                                .into(),
                                right: Identifier {
                                    node: code.node("n2", 1),
                                    name: "n2".into(),
                                }
                                .into(),
                                operator: Operator::Plus,
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
