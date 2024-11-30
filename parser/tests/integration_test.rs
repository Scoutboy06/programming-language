use lexer::{ArithmeticOperator, Lexer, Token, TokenKind};
use parser::{
    expressions::{BinaryExpression, BinaryOperation, Expression, Literal, NumberLiteral},
    nodes::{program::Program, Node},
    statements::{Identifier, Statement, VariableDeclaration, VariableDeclarator, VariableKind},
    Parser,
};
use pretty_assertions::assert_eq;

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
                    node: Node::new(4, source_code.len()),
                    id: Identifier::new("a".into(), 4, 5),
                    init: Some(Expression::Literal(Box::new(Literal::Number(
                        NumberLiteral {
                            node: Node::new(8, source_code.len()),
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
                        BinaryExpression::BinaryOperation(BinaryOperation {
                            node: Node::new(8, 17),
                            operator: ArithmeticOperator::Plus,
                            left: NumberLiteral {
                                node: Node::new(8, 9),
                                value: 6.0,
                            }
                            .into(),
                            right: Box::new(BinaryExpression::BinaryOperation(BinaryOperation {
                                node: Node::new(12, 17),
                                operator: ArithmeticOperator::Mult,
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
                            })),
                        })
                        .into(),
                    )),
                }],
            }
            .into(),
        )],
    };

    assert_eq!(result, Ok(expected));
}
