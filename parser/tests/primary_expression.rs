use insta::assert_snapshot;
use parser::Parser;

#[test]
fn assignment_number_literal() {
    let code = "let a = 50.5";
    let mut parser = Parser::new(&code);
    let result = parser.parse();
    if let Err(err) = result {
        err.print(&code);
        panic!();
    }
    let program = result.unwrap();
    assert_snapshot!(format!("{:#?}", program));
}

#[test]
fn assignment_paren_literal() {
    let code = "const a = (50.5)";
    let mut parser = Parser::new(&code);
    let result = parser.parse();
    if let Err(err) = result {
        err.print(&code);
        panic!();
    }
    let program = result.unwrap();
    assert_snapshot!(format!("{:#?}", program));
}

#[test]
fn array_literal() {
    let code = "const nums = [1, 2, 3, 4];";
    let mut parser = Parser::new(&code);
    let result = parser.parse();
    if let Err(err) = result {
        err.print(&code);
        panic!();
    }
    let program = result.unwrap();
    assert_snapshot!(format!("{:#?}", program));
}

#[test]
fn nested_array() {
    let code = "const nums = [[1], [2]];";
    let mut parser = Parser::new(&code);
    let result = parser.parse();
    if let Err(err) = result {
        err.print(&code);
        panic!();
    }
    let program = result.unwrap();
    assert_snapshot!(format!("{:#?}", program));
}

#[test]
fn object_literal() {
    let code = "var obj = { k1: 101, k2: \"2\", k3: true };";
    let mut parser = Parser::new(&code);
    let result = parser.parse();
    if let Err(err) = result {
        err.print(&code);
        panic!();
    }
    let program = result.unwrap();
    assert_snapshot!(format!("{:#?}", program));
}

#[test]
fn object_shorthand_property() {
    let code = "var obj = { name, age };";
    let mut parser = Parser::new(&code);
    let result = parser.parse();
    if let Err(err) = result {
        err.print(&code);
        panic!();
    }
    let program = result.unwrap();
    assert_snapshot!(format!("{:#?}", program));
}

#[test]
fn object_computed_property() {
    let code = "var obj = { [key]: value, [123]: 456, [\"hello\"]: \"world\" };";
    let mut parser = Parser::new(&code);
    let result = parser.parse();
    if let Err(err) = result {
        err.print(&code);
        panic!();
    }
    let program = result.unwrap();
    assert_snapshot!(format!("{:#?}", program));
}

#[test]
fn object_method() {
    let code = "var obj = { print(name) {} };";
    let mut parser = Parser::new(&code);
    let result = parser.parse();
    if let Err(err) = result {
        err.print(&code);
        panic!();
    }
    let program = result.unwrap();
    assert_snapshot!(format!("{:#?}", program));
}
