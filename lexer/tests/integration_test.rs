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
    use TokenKind as TK;
    use TokenValue as TV;
    let source_code = "8 + 5 - 2 / 2";
    expect_tokens(
        &source_code,
        &vec![
            (TK::Number, TV::Number(8.0)),
            (TK::Plus, TV::None),
            (TK::Number, TV::Number(5.0)),
            (TK::Minus, TV::None),
            (TK::Number, TV::Number(2.0)),
            (TK::Slash, TV::None),
            (TK::Number, TV::Number(2.0)),
        ],
    );
}

#[test]
fn let_statement() {
    use TokenKind as TK;
    use TokenValue as TV;
    let source_code = "let x = 123.0 + 456.0;";
    expect_tokens(
        &source_code,
        &vec![
            (TK::Keyword, TV::Keyword(Keyword::Let)),
            (TK::Identifier, TV::Identifier("x".into())),
            (TK::Equals, TV::None),
            (TK::Number, TV::Number(123.0)),
            (TK::Plus, TV::None),
            (TK::Number, TV::Number(456.0)),
            (TK::SemiColon, TV::None),
        ],
    );
}

#[test]
fn function() {
    use TokenKind as TK;
    use TokenValue as TV;
    let source_code = "function sum(n1: number, n2: number): number {}";
    expect_tokens(
        &source_code,
        &vec![
            (TK::Keyword, TV::Keyword(Keyword::Function)),
            (TK::Identifier, TV::Identifier("sum".into())),
            (TK::OpenParen, TV::None),
            (TK::Identifier, TV::Identifier("n1".into())),
            (TK::Colon, TV::None),
            (TK::Keyword, TV::Keyword(Keyword::NumberType)),
            (TK::Comma, TV::None),
            (TK::Identifier, TV::Identifier("n2".into())),
            (TK::Colon, TV::None),
            (TK::Keyword, TV::Keyword(Keyword::NumberType)),
            (TK::CloseParen, TV::None),
            (TK::Colon, TV::None),
            (TK::Keyword, TV::Keyword(Keyword::NumberType)),
            (TK::OpenBrace, TV::None),
            (TK::CloseBrace, TV::None),
        ],
    );
}

#[test]
fn string_literal() {
    use TokenKind as TK;
    use TokenValue as TV;
    let source_code = "let x = 'This is a string literal';";
    expect_tokens(
        &source_code,
        &vec![
            (TK::Keyword, TV::Keyword(Keyword::Let)),
            (TK::Identifier, TV::Identifier("x".into())),
            (TK::Equals, TV::None),
            (TK::String, TV::String("'This is a string literal'".into())),
            (TK::SemiColon, TV::None),
        ],
    );
}

#[test]
fn template_string_literal() {
    use TokenKind as TK;
    use TokenValue as TV;
    let source_code =
        "let x = `A ${string_type} string with ${is_nested ? `${nested_level} nestings` : ''}`;";
    expect_tokens(
        &source_code,
        &vec![
            (TK::Keyword, TV::Keyword(Keyword::Let)),
            (TK::Identifier, TV::Identifier("x".into())),
            (TK::Equals, TV::None),
            (
                TK::String,
                TV::String(
                    "`A ${string_type} string with ${is_nested ? `${nested_level} nestings` : ''}`"
                        .into(),
                ),
            ),
            (TK::SemiColon, TV::None),
        ],
    );
}

#[test]
fn if_statement_with_boolean() {
    use TokenKind as TK;
    use TokenValue as TV;
    let source_code = "if(false) {} else if(true) {} else {}";
    expect_tokens(
        &source_code,
        &vec![
            (TK::Keyword, TV::Keyword(Keyword::If)),
            (TK::OpenParen, TV::None),
            (TK::Boolean, TV::Boolean(false)),
            (TK::CloseParen, TV::None),
            (TK::OpenBrace, TV::None),
            (TK::CloseBrace, TV::None),
            (TK::Keyword, TV::Keyword(Keyword::Else)),
            (TK::Keyword, TV::Keyword(Keyword::If)),
            (TK::OpenParen, TV::None),
            (TK::Boolean, TV::Boolean(true)),
            (TK::CloseParen, TV::None),
            (TK::OpenBrace, TV::None),
            (TK::CloseBrace, TV::None),
            (TK::Keyword, TV::Keyword(Keyword::Else)),
            (TK::OpenBrace, TV::None),
            (TK::CloseBrace, TV::None),
        ],
    );
}

#[test]
fn if_statement_with_variables() {
    use TokenKind as TK;
    use TokenValue as TV;
    let source_code = "if (this.pos.x > window.innerWidth) {}";
    expect_tokens(
        &source_code,
        &vec![
            (TK::Keyword, TV::Keyword(Keyword::If)),
            (TK::OpenParen, TV::None),
            (TK::Keyword, TV::Keyword(Keyword::This)),
            (TK::Dot, TV::None),
            (TK::Identifier, TV::Identifier("pos".into())),
            (TK::Dot, TV::None),
            (TK::Identifier, TV::Identifier("x".into())),
            (TK::GreaterThan, TV::None),
            (TK::Identifier, TV::Identifier("window".into())),
            (TK::Dot, TV::None),
            (TK::Identifier, TV::Identifier("innerWidth".into())),
            (TK::CloseParen, TV::None),
            (TK::OpenBrace, TV::None),
            (TK::CloseBrace, TV::None),
        ],
    );
}
