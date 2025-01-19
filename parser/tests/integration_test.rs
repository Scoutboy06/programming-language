use lexer::{Lexer, Operator, Token, TokenKind, TypeKeyword};
use parser::{
    expressions::{
        types::{
            ArrayType, KeywordType, TypeAnnotation, TypeParameter, TypeParameterDeclaration,
            TypeReference, TypeValue,
        },
        ArrayExpression, ArrowFunctionExpression, AssignmentExpression, BinaryExpression,
        BooleanLiteral, CallExpression, ComputedProperty, FunctionExpression, MemberExpression,
        NumberLiteral, ParenthesisExpression, StringLiteral, UpdateExpression, UpdateOperator,
    },
    nodes::{program::Program, Node},
    statements::{
        BlockStatement, EnumMember, EnumStatement, ExpressionStatement, ForStatement,
        FunctionDeclaration, Identifier, IfStatement, Parameter, ReturnStatement,
        VariableDeclaration, VariableDeclarator, VariableKind, WhileStatement,
    },
    Parser,
};
use pretty_assertions::assert_eq;

trait NodeConstructor {
    fn find_n(&self, target: &str, n: usize) -> Option<usize>;
    fn node(&self, target: &str, n: usize) -> Node;
    fn between(&self, left: (&str, usize), right: (&str, usize)) -> Node;
    fn between_incl(&self, left: (&str, usize), right: (&str, usize)) -> Node;
}

impl NodeConstructor for str {
    /// Searches for the n-th occurrence of `target`, and returns the index
    fn find_n(&self, target: &str, n: usize) -> Option<usize> {
        let mut index: Option<usize> = None;
        let bytes = self.as_bytes();
        let target_bytes = target.as_bytes();
        let mut count = 0;

        'outer: for i in 0..bytes.len() {
            for j in 0..target_bytes.len() {
                let b = bytes[i + j];
                if b != target_bytes[j] {
                    continue 'outer;
                }
            }

            count += 1;
            if count == n + 1 {
                index = Some(i);
                break 'outer;
            }
        }

        index
    }

    // Searches for the n-th occurrence of `target`, and returns a Node that spans inside it
    fn node(&self, target: &str, n: usize) -> Node {
        let index = self.find_n(target, n);

        assert!(
            index.is_some(),
            "Target not found in NodeConstructor::node()\n  target: {}\n  n: {}",
            target,
            n
        );

        Node::new(index.unwrap(), index.unwrap() + target.len())
    }

    /// Searches for the n-th occurrence of `left` and `right`, and returns a Node that spans between them.
    fn between(&self, left: (&str, usize), right: (&str, usize)) -> Node {
        let left_bytes = left.0.as_bytes();
        let right_bytes = right.0.as_bytes();

        assert!(self.len() > left_bytes.len() + right_bytes.len());

        let left_index = self.find_n(left.0, left.1).expect(&format!(
            "Could not find left value\n  left: {}\n  n: {}",
            left.0, left.1
        ));
        let right_index = self.find_n(right.0, right.1).expect(&format!(
            "Could not find right value\n  right: {}\n  n: {}",
            right.0, right.1
        ));

        assert!(
            left_index < right_index,
            "Left index and right index are wrong"
        );

        Node::new(left_index + left.0.len(), right_index)
    }

    /// Searches for the n-th occurrence of `left` and `right`, and returns a Node that spans between *and including* them
    fn between_incl(&self, left: (&str, usize), right: (&str, usize)) -> Node {
        let between = self.between(left, right);

        Node::new(between.start - left.0.len(), between.end + right.0.len())
    }
}

#[test]
fn helper_find_n() {
    let code = "let a = 50.5";

    assert_eq!(code.find_n("a", 0), Some(4));
    assert_eq!(code.find_n("=", 0), Some(6));
    assert_eq!(code.find_n("5", 0), Some(8));
    assert_eq!(code.find_n("5", 1), Some(11));
}

#[test]
fn helper_node() {
    let code = "{ hello() }";

    assert_eq!(code.node("hello", 0), Node::new(2, 7));
    assert_eq!(code.node("()", 0), Node::new(7, 9));
    assert_eq!(code.node("l", 0), Node::new(4, 5));
    assert_eq!(code.node("l", 1), Node::new(5, 6));
    assert_eq!(code.node(code, 0), Node::new(0, code.len()));
}

#[test]
fn helper_between() {
    let code = "function a() { hello() }";

    assert_eq!(code.between(("f", 0), ("n", 1)), Node::new(1, 7));
    assert_eq!(code.between(("(", 0), (")", 0)), Node::new(11, 11));
    assert_eq!(code.between(("(", 1), (")", 1)), Node::new(21, 21));
    assert_eq!(code.between(("{", 0), ("}", 0)), Node::new(14, 23));
}

#[test]
fn helper_between_incl() {
    let code = "function a() { hello() }";

    assert_eq!(code.between_incl(("f", 0), ("n", 1)), Node::new(0, 8));
    assert_eq!(code.between_incl(("(", 0), (")", 0)), Node::new(10, 12));
    assert_eq!(code.between_incl(("(", 1), (")", 1)), Node::new(20, 22));
    assert_eq!(code.between_incl(("{", 0), ("}", 0)), Node::new(13, 24));
}

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

    assert_eq!(result, Ok(expected));
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

    assert_eq!(result, Ok(expected));
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

    assert_eq!(result, Ok(expected));
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

#[test]
fn enum_statement() {
    let code = "enum Foo { Bar, Baz }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![EnumStatement {
            node: Node::new(0, code.len()),
            is_declare: false,
            is_const: false,
            id: Identifier {
                node: code.node("Foo", 0),
                name: "Foo".into(),
            },
            members: vec![
                EnumMember {
                    node: code.node("Bar", 0),
                    id: Identifier {
                        node: code.node("Bar", 0),
                        name: "Bar".into(),
                    },
                    init: None,
                },
                EnumMember {
                    node: code.node("Baz", 0),
                    id: Identifier {
                        node: code.node("Baz", 0),
                        name: "Baz".into(),
                    },
                    init: None,
                },
            ],
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn enum_with_initialized_values() {
    let code = "enum Color { White = 1, Black = 0 }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![EnumStatement {
            node: Node::new(0, code.len()),
            is_declare: false,
            is_const: false,
            id: Identifier {
                node: code.node("Color", 0),
                name: "Color".into(),
            },
            members: vec![
                EnumMember {
                    node: code.node("White = 1", 0),
                    id: Identifier {
                        node: code.node("White", 0),
                        name: "White".into(),
                    },
                    init: Some(
                        NumberLiteral {
                            node: code.node("1", 0),
                            value: 1.0,
                        }
                        .into(),
                    ),
                },
                EnumMember {
                    node: code.node("Black = 0", 0),
                    id: Identifier {
                        node: code.node("Black", 0),
                        name: "Black".into(),
                    },
                    init: Some(
                        NumberLiteral {
                            node: code.node("0", 0),
                            value: 0.0,
                        }
                        .into(),
                    ),
                },
            ],
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn const_enum_statement() {
    let code = "const enum Foo { Bar, Baz }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![EnumStatement {
            node: Node::new(0, code.len()),
            is_declare: false,
            is_const: true,
            id: Identifier {
                node: code.node("Foo", 0),
                name: "Foo".into(),
            },
            members: vec![
                EnumMember {
                    node: code.node("Bar", 0),
                    id: Identifier {
                        node: code.node("Bar", 0),
                        name: "Bar".into(),
                    },
                    init: None,
                },
                EnumMember {
                    node: code.node("Baz", 0),
                    id: Identifier {
                        node: code.node("Baz", 0),
                        name: "Baz".into(),
                    },
                    init: None,
                },
            ],
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn ambient_enum_statement() {
    let code = "declare enum Foo { Bar, Baz }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![EnumStatement {
            node: Node::new(0, code.len()),
            is_declare: true,
            is_const: false,
            id: Identifier {
                node: code.node("Foo", 0),
                name: "Foo".into(),
            },
            members: vec![
                EnumMember {
                    node: code.node("Bar", 0),
                    id: Identifier {
                        node: code.node("Bar", 0),
                        name: "Bar".into(),
                    },
                    init: None,
                },
                EnumMember {
                    node: code.node("Baz", 0),
                    id: Identifier {
                        node: code.node("Baz", 0),
                        name: "Baz".into(),
                    },
                    init: None,
                },
            ],
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
