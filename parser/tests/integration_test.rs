use lexer::{Lexer, Operator, Token, TokenKind, TypeKeyword};
use parser::{
    expressions::{
        AssignmentExpression, BinaryExpression, BooleanLiteral, CallExpression, ComputedProperty,
        Expression, Literal, MemberExpression, MemberProperty, NumberLiteral, StringLiteral, Type,
        TypeAnnotation, TypeValue,
    },
    nodes::{program::Program, Node},
    statements::{
        BlockStatement, ExpressionStatement, FunctionDeclaration, Identifier, IfStatement,
        Parameter, ReturnStatement, Statement, VariableDeclaration, VariableDeclarator,
        VariableKind,
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
    let source_code = "let a = 50.5";

    assert_eq!(source_code.find_n("a", 0), Some(4));
    assert_eq!(source_code.find_n("=", 0), Some(6));
    assert_eq!(source_code.find_n("5", 0), Some(8));
    assert_eq!(source_code.find_n("5", 1), Some(11));
}

#[test]
fn helper_node() {
    let source_code = "{ hello() }";

    assert_eq!(source_code.node("hello", 0), Node::new(2, 7));
    assert_eq!(source_code.node("()", 0), Node::new(7, 9));
    assert_eq!(source_code.node("l", 0), Node::new(4, 5));
    assert_eq!(source_code.node("l", 1), Node::new(5, 6));
    assert_eq!(
        source_code.node(source_code, 0),
        Node::new(0, source_code.len())
    );
}

#[test]
fn helper_between() {
    let source_code = "function a() { hello() }";

    assert_eq!(source_code.between(("f", 0), ("n", 1)), Node::new(1, 7));
    assert_eq!(source_code.between(("(", 0), (")", 0)), Node::new(11, 11));
    assert_eq!(source_code.between(("(", 1), (")", 1)), Node::new(21, 21));
    assert_eq!(source_code.between(("{", 0), ("}", 0)), Node::new(14, 23));
}

#[test]
fn helper_between_incl() {
    let source_code = "function a() { hello() }";

    assert_eq!(
        source_code.between_incl(("f", 0), ("n", 1)),
        Node::new(0, 8)
    );
    assert_eq!(
        source_code.between_incl(("(", 0), (")", 0)),
        Node::new(10, 12)
    );
    assert_eq!(
        source_code.between_incl(("(", 1), (")", 1)),
        Node::new(20, 22)
    );
    assert_eq!(
        source_code.between_incl(("{", 0), ("}", 0)),
        Node::new(13, 24)
    );
}

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
                    node: source_code.between_incl(("a", 0), ("50.5", 0)),
                    id: Identifier {
                        node: source_code.node("a", 0),
                        name: "a".into(),
                    },
                    init: Some(Expression::Literal(Box::new(Literal::Number(
                        NumberLiteral {
                            node: source_code.node("50.5", 0),
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
                            operator: Operator::Plus,
                            left: NumberLiteral {
                                node: Node::new(8, 9),
                                value: 6.0,
                            }
                            .into(),
                            right: Expression::BinaryExpression(
                                BinaryExpression {
                                    node: Node::new(12, 17),
                                    operator: Operator::Mult,
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
        body: vec![Statement::ExpressionStatement(Box::new(
            ExpressionStatement {
                node: Node::new(0, source_code.len()),
                expression: Expression::CallExpression(Box::new(CallExpression {
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
                })),
            },
        ))],
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
        body: vec![Statement::ExpressionStatement(Box::new(
            ExpressionStatement {
                node: Node::new(0, source_code.len()),
                expression: Expression::CallExpression(
                    CallExpression {
                        node: Node::new(0, source_code.len()),
                        callee: Expression::MemberExpression(
                            MemberExpression {
                                node: source_code.node("console.log", 0),
                                object: Expression::Identifier(
                                    Identifier {
                                        node: source_code.node("console", 0),
                                        name: "console".into(),
                                    }
                                    .into(),
                                ),
                                property: MemberProperty::Identifier(Identifier {
                                    node: source_code.node("log", 0),
                                    name: "log".into(),
                                }),
                            }
                            .into(),
                        ),
                        arguments: vec![NumberLiteral {
                            node: source_code.node("50.5", 0),
                            value: 50.5,
                        }
                        .into()],
                    }
                    .into(),
                ),
            },
        ))],
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
        body: vec![
            Statement::ExpressionStatement(Box::new(ExpressionStatement {
                node: Node::new(0, source_code.len()),
                expression: Expression::MemberExpression(
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
                ),
            }))
            .into(),
        ],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn function_declaration() {
    let source_code = "function add(n1: number, n2: number): number {
    return n1 + n2;
}";
    let mut parser = Parser::new(&source_code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, source_code.len()),
        shebang: None,
        body: vec![Statement::FunctionDeclaration(Box::new(
            FunctionDeclaration {
                node: Node::new(0, source_code.len()),
                id: Some(Identifier {
                    node: source_code.node("add", 0),
                    name: "add".into(),
                }),
                is_expression: false,
                is_generator: false,
                is_async: false,
                params: vec![
                    Parameter {
                        node: source_code.node("n1: number", 0),
                        identifier: Identifier {
                            node: source_code.node("n1", 0),
                            name: "n1".into(),
                        },
                        type_annotation: TypeAnnotation {
                            node: source_code.node(": number", 0),
                            type_value: Type {
                                node: source_code.node("number", 0),
                                value: TypeValue::KeywordType(TypeKeyword::Number),
                            },
                        },
                        optional: false,
                    },
                    Parameter {
                        node: source_code.node("n2: number", 0),
                        identifier: Identifier {
                            node: source_code.node("n2", 0),
                            name: "n2".into(),
                        },
                        type_annotation: TypeAnnotation {
                            node: source_code.node(": number", 1),
                            type_value: Type {
                                node: source_code.node("number", 1),
                                value: TypeValue::KeywordType(TypeKeyword::Number),
                            },
                        },
                        optional: false,
                    },
                ],
                return_type: Some(TypeAnnotation {
                    node: source_code.node(": number", 2),
                    type_value: Type {
                        node: source_code.node("number", 2),
                        value: TypeValue::KeywordType(TypeKeyword::Number),
                    },
                }),
                body: BlockStatement {
                    node: source_code.between_incl(("{", 0), ("}", 0)),
                    statements: vec![Statement::ReturnStatement(Box::new(ReturnStatement {
                        node: source_code.node("return n1 + n2;", 0),
                        value: Expression::BinaryExpression(Box::new(BinaryExpression {
                            node: source_code.node("n1 + n2", 0),
                            operator: Operator::Plus,
                            left: Identifier {
                                node: source_code.node("n1", 1),
                                name: "n1".into(),
                            }
                            .into(),
                            right: Identifier {
                                node: source_code.node("n2", 1),
                                name: "n2".into(),
                            }
                            .into(),
                        })),
                    }))],
                },
            },
        ))],
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
            Statement::ExpressionStatement(Box::new(ExpressionStatement {
                node: code.node("foo -= 50.5;", 0),
                expression: Expression::AssignmentExpression(
                    AssignmentExpression {
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
                ),
            })),
            Statement::ExpressionStatement(Box::new(ExpressionStatement {
                node: code.node("bar += \"World\";", 0),
                expression: Expression::AssignmentExpression(
                    AssignmentExpression {
                        node: code.node("bar += \"World\"", 0),
                        left: Identifier {
                            node: code.node("bar", 0),
                            name: "bar".into(),
                        }
                        .into(),
                        right: StringLiteral {
                            node: code.node("\"World\"", 0),
                            value: "World".into(),
                        }
                        .into(),
                        operator: Operator::PlusEquals,
                    }
                    .into(),
                ),
            })),
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
        body: vec![Statement::IfStatement(Box::new(IfStatement {
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
        }))],
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
        body: vec![Statement::IfStatement(Box::new(IfStatement {
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
            consequent: Some(Statement::BlockStatement(Box::new(BlockStatement {
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
            }))),
        }))],
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
