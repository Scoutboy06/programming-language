use lexer::{Operator, TypeKeyword};
use parser::{
    expressions::{
        types::{
            ArrayType, KeywordType, TypeAnnotation, TypeParameter, TypeParameterDeclaration,
            TypeReference,
        },
        ArrayExpression, BinaryExpression,
    },
    nodes::{program::Program, Node},
    statements::{
        BlockStatement, FunctionDeclaration, Identifier, Parameter, ReturnStatement,
        VariableDeclaration, VariableDeclarator, VariableKind,
    },
    Parser,
};
use pretty_assertions::assert_eq;
mod helpers;
use helpers::*;

#[test]
fn assignment_with_simple_type() {
    let code = "var el1: number, el2: Foo;";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: Node::new(0, code.len()),
            kind: VariableKind::Var,
            declarations: vec![
                VariableDeclarator {
                    node: code.node("el1: number", 0),
                    id: Identifier {
                        node: code.node("el1", 0),
                        name: "el1".into(),
                    },
                    type_annotation: Some(TypeAnnotation {
                        node: code.node(": number", 0),
                        type_value: KeywordType {
                            node: code.node("number", 0),
                            kind: TypeKeyword::Number,
                        }
                        .into(),
                    }),
                    init: None,
                },
                VariableDeclarator {
                    node: code.node("el2: Foo", 0),
                    id: Identifier {
                        node: code.node("el2", 0),
                        name: "el2".into(),
                    },
                    type_annotation: Some(TypeAnnotation {
                        node: code.node(": Foo", 0),
                        type_value: TypeReference {
                            node: code.node("Foo", 0),
                            type_name: Identifier {
                                node: code.node("Foo", 0),
                                name: "Foo".into(),
                            },
                            type_params: None,
                        }
                        .into(),
                    }),
                    init: None,
                },
            ],
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn array_type() {
    let code = "const num: number[], foo: Foo[];";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: Node::new(0, code.len()),
            kind: VariableKind::Const,
            declarations: vec![
                VariableDeclarator {
                    node: code.node("num: number[]", 0),
                    id: Identifier {
                        node: code.node("num", 0),
                        name: "num".into(),
                    },
                    type_annotation: Some(TypeAnnotation {
                        node: code.node(": number[]", 0),
                        type_value: ArrayType {
                            node: code.node("number[]", 0),
                            type_value: KeywordType {
                                node: code.node("number", 0),
                                kind: TypeKeyword::Number,
                            }
                            .into(),
                        }
                        .into(),
                    }),
                    init: None,
                },
                VariableDeclarator {
                    node: code.node("foo: Foo[]", 0),
                    id: Identifier {
                        node: code.node("foo", 0),
                        name: "foo".into(),
                    },
                    type_annotation: Some(TypeAnnotation {
                        node: code.node(": Foo[]", 0),
                        type_value: ArrayType {
                            node: code.node("Foo[]", 0),
                            type_value: TypeReference {
                                node: code.node("Foo", 0),
                                type_name: Identifier {
                                    node: code.node("Foo", 0),
                                    name: "Foo".into(),
                                },
                                type_params: None,
                            }
                            .into(),
                        }
                        .into(),
                    }),
                    init: None,
                },
            ],
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn type_params() {
    let code = "let grid: Array<Array<number>> = [];";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![VariableDeclaration {
            node: Node::new(0, code.len()),
            kind: VariableKind::Let,
            declarations: vec![VariableDeclarator {
                node: code.between_incl(("grid", 0), ("[]", 0)),
                id: Identifier {
                    node: code.node("grid", 0),
                    name: "grid".into(),
                },
                type_annotation: Some(TypeAnnotation {
                    node: code.node(": Array<Array<number>>", 0),
                    type_value: TypeReference {
                        node: code.node("Array<Array<number>>", 0),
                        type_name: Identifier {
                            node: code.node("Array", 0),
                            name: "Array".into(),
                        },
                        type_params: Some(vec![TypeReference {
                            node: code.node("Array<number>", 0),
                            type_name: Identifier {
                                node: code.node("Array", 1),
                                name: "Array".into(),
                            },
                            type_params: Some(vec![KeywordType {
                                node: code.node("number", 0),
                                kind: TypeKeyword::Number,
                            }
                            .into()]),
                        }
                        .into()]),
                    }
                    .into(),
                }),
                init: Some(
                    ArrayExpression {
                        node: code.node("[]", 0),
                        items: vec![],
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
fn generics_on_function_declaration() {
    let code = "function add<T>(el1: T, el2: T): T {
        return el1 + el2;
    }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![FunctionDeclaration {
            node: Node::new(0, code.len()),
            is_async: false,
            is_generator: false,
            id: Identifier {
                node: code.node("add", 0),
                name: "add".into(),
            },
            type_parameters: Some(TypeParameterDeclaration {
                node: code.node("<T>", 0),
                parameters: vec![TypeParameter {
                    node: code.node("T", 0),
                    id: Identifier {
                        node: code.node("T", 0),
                        name: "T".into(),
                    },
                }],
            }),
            params: vec![
                Parameter {
                    node: code.node("el1: T", 0),
                    identifier: Identifier {
                        node: code.node("el1", 0),
                        name: "el1".into(),
                    },
                    type_annotation: Some(TypeAnnotation {
                        node: code.node(": T", 0),
                        type_value: TypeReference {
                            node: code.node("T", 1),
                            type_name: Identifier {
                                node: code.node("T", 1),
                                name: "T".into(),
                            },
                            type_params: None,
                        }
                        .into(),
                    }),
                    optional: false,
                },
                Parameter {
                    node: code.node("el2: T", 0),
                    identifier: Identifier {
                        node: code.node("el2", 0),
                        name: "el2".into(),
                    },
                    type_annotation: Some(TypeAnnotation {
                        node: code.node(": T", 1),
                        type_value: TypeReference {
                            node: code.node("T", 2),
                            type_name: Identifier {
                                node: code.node("T", 2),
                                name: "T".into(),
                            },
                            type_params: None,
                        }
                        .into(),
                    }),
                    optional: false,
                },
            ],
            return_type: Some(TypeAnnotation {
                node: code.node(": T", 2),
                type_value: TypeReference {
                    node: code.node("T", 3),
                    type_name: Identifier {
                        node: code.node("T", 3),
                        name: "T".into(),
                    },
                    type_params: None,
                }
                .into(),
            }),
            body: BlockStatement {
                node: code.between_incl(("{", 0), ("}", 0)),
                statements: vec![ReturnStatement {
                    node: code.node("return el1 + el2;", 0),
                    value: BinaryExpression {
                        node: code.node("el1 + el2", 0),
                        left: Identifier {
                            node: code.node("el1", 1),
                            name: "el1".into(),
                        }
                        .into(),
                        right: Identifier {
                            node: code.node("el2", 1),
                            name: "el2".into(),
                        }
                        .into(),
                        operator: Operator::Plus,
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
