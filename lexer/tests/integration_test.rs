use lexer::{Keyword, Lexer, TokenKind, TokenValue};
use pretty_assertions::assert_eq;
use string_cache::DefaultAtom as Atom;

fn expect_tokens(source_code: &str, expected_tokens: &[(TokenKind, TokenValue)]) {
    let mut lexer = Lexer::new(source_code);

    for i in 0.. {
        let token = lexer.next_token();
        // dbg!(&token);

        if token.kind == TokenKind::Eof {
            assert_eq!(token.value, TokenValue::None);
            assert_eq!(i, expected_tokens.len());
            break;
        }

        assert_eq!(token.kind, expected_tokens[i].0);
        assert_eq!(token.value, expected_tokens[i].1);
    }
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
            (TokenKind::Identifier, TokenValue::String(Atom::from("x"))),
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
            (TokenKind::Identifier, TokenValue::String(Atom::from("sum"))),
            (TokenKind::OpenParen, TokenValue::None),
            (TokenKind::Identifier, TokenValue::String(Atom::from("n1"))),
            (TokenKind::Colon, TokenValue::None),
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::NumberType)),
            (TokenKind::Comma, TokenValue::None),
            (TokenKind::Identifier, TokenValue::String(Atom::from("n2"))),
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
            (TokenKind::Identifier, TokenValue::String(Atom::from("x"))),
            (TokenKind::Equals, TokenValue::None),
            (
                TokenKind::String,
                TokenValue::String(Atom::from("'This is a string literal'")),
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
            (TokenKind::Identifier, TokenValue::String(Atom::from("x"))),
            (TokenKind::Equals, TokenValue::None),
            (
                TokenKind::String,
                TokenValue::String(Atom::from(
                    "`A ${string_type} string with ${is_nested ? `${nested_level} nestings` : ''}`",
                )),
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
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::False)),
            (TokenKind::CloseParen, TokenValue::None),
            (TokenKind::OpenBrace, TokenValue::None),
            (TokenKind::CloseBrace, TokenValue::None),
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::Else)),
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::If)),
            (TokenKind::OpenParen, TokenValue::None),
            (TokenKind::Keyword, TokenValue::Keyword(Keyword::True)),
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
            (TokenKind::Identifier, TokenValue::String(Atom::from("pos"))),
            (TokenKind::Dot, TokenValue::None),
            (TokenKind::Identifier, TokenValue::String(Atom::from("x"))),
            (TokenKind::GreaterThan, TokenValue::None),
            (
                TokenKind::Identifier,
                TokenValue::String(Atom::from("window")),
            ),
            (TokenKind::Dot, TokenValue::None),
            (
                TokenKind::Identifier,
                TokenValue::String(Atom::from("innerWidth")),
            ),
            (TokenKind::CloseParen, TokenValue::None),
            (TokenKind::OpenBrace, TokenValue::None),
            (TokenKind::CloseBrace, TokenValue::None),
        ],
    );
}
