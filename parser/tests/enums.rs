use parser::Parser;

#[test]
fn enum_statement() {
    let code = "enum Foo { Bar, Baz }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

    let program = result.unwrap();
    insta::assert_snapshot!(format!("{:#?}", program));
}

#[test]
fn enum_with_initialized_values() {
    let code = "enum Color { White = 1, Black = 0 }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

    let program = result.unwrap();
    insta::assert_snapshot!(format!("{:#?}", program));
}

#[test]
fn const_enum_statement() {
    let code = "const enum Foo { Bar, Baz }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

    let program = result.unwrap();
    insta::assert_snapshot!(format!("{:#?}", program));
}

#[test]
fn ambient_enum_statement() {
    let code = "declare enum Foo { Bar, Baz }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

    let program = result.unwrap();
    insta::assert_snapshot!(format!("{:#?}", program));
}
