use lexer::{Keyword, Lexer, TokenKind, TokenValue};
use pretty_assertions::assert_eq;

fn expect_tokens(source_code: &str, expected_tokens: &[(TokenKind, TokenValue)]) {
    let lexer = Lexer::new(source_code);
    let tokens = lexer.map(|tok| (tok.kind, tok.value)).collect::<Vec<_>>();

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn empty() {
    let source_code = " ";
    expect_tokens(&source_code, &vec![]);
}

#[test]
fn numbers() {
    let source_code = "8 + 5 - 2 / 2";
    expect_tokens(
        &source_code,
        &vec![
            (TokenKind::Number, TokenValue::Number(8.0)),
            (TokenKind::Plus, TokenValue::None),
            (TokenKind::Number, TokenValue::Number(5.0)),
            (TokenKind::Minus, TokenValue::None),
            (TokenKind::Number, TokenValue::Number(2.0)),
            (TokenKind::Slash, TokenValue::None),
            (TokenKind::Number, TokenValue::Number(2.0)),
        ],
    );
}

#[test]
fn let_statement() {
    let source_code = "let x = 123.0 + 456.0;";
    expect_tokens(
        &source_code,
        &vec![
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::Let)),
            (TokenKind::Identifier, TokenValue::String("x".into())),
            (TokenKind::Equals, TokenValue::None),
            (TokenKind::Number, TokenValue::Number(123.0)),
            (TokenKind::Plus, TokenValue::None),
            (TokenKind::Number, TokenValue::Number(456.0)),
            (TokenKind::SemiColon, TokenValue::None),
        ],
    );
}

#[test]
fn function() {
    let source_code = "function sum(n1: number, n2: number): number {}";
    expect_tokens(
        &source_code,
        &vec![
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::Function)),
            (TokenKind::Identifier, TokenValue::String("sum".into())),
            (TokenKind::OpenParen, TokenValue::None),
            (TokenKind::Identifier, TokenValue::String("n1".into())),
            (TokenKind::Colon, TokenValue::None),
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::NumberType)),
            (TokenKind::Comma, TokenValue::None),
            (TokenKind::Identifier, TokenValue::String("n2".into())),
            (TokenKind::Colon, TokenValue::None),
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::NumberType)),
            (TokenKind::CloseParen, TokenValue::None),
            (TokenKind::Colon, TokenValue::None),
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::NumberType)),
            (TokenKind::OpenBrace, TokenValue::None),
            (TokenKind::CloseBrace, TokenValue::None),
        ],
    );
}

#[test]
fn string_literal() {
    let source_code = "let x = 'This is a string literal';";
    expect_tokens(
        &source_code,
        &vec![
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::Let)),
            (TokenKind::Identifier, TokenValue::String("x".into())),
            (TokenKind::Equals, TokenValue::None),
            (
                TokenKind::String,
                TokenValue::String("This is a string literal".into()),
            ),
            (TokenKind::SemiColon, TokenValue::None),
        ],
    );
}

#[test]
fn template_string_literal() {
    let source_code =
        "let x = `A ${string_type} string with ${is_nested ? `${nested_level} nestings` : ''}`;";
    expect_tokens(
        &source_code,
        &vec![
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::Let)),
            (TokenKind::Identifier, TokenValue::String("x".into())),
            (TokenKind::Equals, TokenValue::None),
            (
                TokenKind::String,
                TokenValue::String(
                    "A ${string_type} string with ${is_nested ? `${nested_level} nestings` : ''}"
                        .into(),
                ),
            ),
            (TokenKind::SemiColon, TokenValue::None),
        ],
    );
}

#[test]
fn if_statement_with_boolean() {
    let source_code = "if(false) {} else if(true) {} else {}";
    expect_tokens(
        &source_code,
        &vec![
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::If)),
            (TokenKind::OpenParen, TokenValue::None),
            (TokenKind::Boolean, TokenValue::Boolean(false)),
            (TokenKind::CloseParen, TokenValue::None),
            (TokenKind::OpenBrace, TokenValue::None),
            (TokenKind::CloseBrace, TokenValue::None),
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::Else)),
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::If)),
            (TokenKind::OpenParen, TokenValue::None),
            (TokenKind::Boolean, TokenValue::Boolean(true)),
            (TokenKind::CloseParen, TokenValue::None),
            (TokenKind::OpenBrace, TokenValue::None),
            (TokenKind::CloseBrace, TokenValue::None),
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::Else)),
            (TokenKind::OpenBrace, TokenValue::None),
            (TokenKind::CloseBrace, TokenValue::None),
        ],
    );
}

#[test]
fn if_statement_with_variables() {
    let source_code = "if (this.pos.x > window.innerWidth) {}";
    expect_tokens(
        &source_code,
        &vec![
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::If)),
            (TokenKind::OpenParen, TokenValue::None),
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::This)),
            (TokenKind::Dot, TokenValue::None),
            (TokenKind::Identifier, TokenValue::String("pos".into())),
            (TokenKind::Dot, TokenValue::None),
            (TokenKind::Identifier, TokenValue::String("x".into())),
            (TokenKind::GreaterThan, TokenValue::None),
            (TokenKind::Identifier, TokenValue::String("window".into())),
            (TokenKind::Dot, TokenValue::None),
            (
                TokenKind::Identifier,
                TokenValue::String("innerWidth".into()),
            ),
            (TokenKind::CloseParen, TokenValue::None),
            (TokenKind::OpenBrace, TokenValue::None),
            (TokenKind::CloseBrace, TokenValue::None),
        ],
    );
}
