use lexer::{ArithmeticOperator, Lexer, Token, TokenKind};
use parser::{
    expressions::{BinaryExpression, BinaryOperation, Expression, Literal, NumberLiteral},
    nodes::{program::Program, Node},
    statements::{Identifier, Statement, VariableDeclaration, VariableDeclarator, VariableKind},
    Parser,
};
use pretty_assertions::assert_eq;
use string_cache::Atom;

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
fn variable_decl() {
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
                    id: Identifier::new(Atom::from("a"), 4, 5),
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
fn binary_operation() {
    let source_code = "let y = 6 + 5 * x";
    let mut parser = Parser::new(&source_code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, source_code.len()),
        shebang: None,
        body: vec![Statement::VariableDeclaration(Box::new(
            VariableDeclaration {
                node: Node::new(0, 17),
                kind: VariableKind::Let,
                declarations: vec![VariableDeclarator {
                    node: Node::new(4, 17),
                    id: Identifier::new("y".into(), 4, 5),
                    init: Some(Expression::BinaryExpression(Box::new(
                        BinaryExpression::BinaryOperation(BinaryOperation {
                            node: Node::new(8, 17),
                            operator: ArithmeticOperator::Plus,
                            left: Box::new(BinaryExpression::Literal(Literal::Number(
                                NumberLiteral {
                                    node: Node::new(8, 9),
                                    value: 6.0,
                                },
                            ))),
                            right: Box::new(BinaryExpression::BinaryOperation(BinaryOperation {
                                node: Node::new(12, 17),
                                operator: ArithmeticOperator::Mult,
                                left: Box::new(NumberLiteral::as_bin_expression(
                                    Node::new(12, 13),
                                    5.0,
                                )),
                                right: Box::new(BinaryExpression::Identifier(Identifier::new(
                                    "x".into(),
                                    16,
                                    17,
                                ))),
                            })),
                        }),
                    ))),
                }],
            },
        ))],
    };

    assert_eq!(result, Ok(expected));
}
