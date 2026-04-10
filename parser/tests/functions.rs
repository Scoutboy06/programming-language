use insta::assert_snapshot;
use parser::Parser;

#[test]
fn function_call() {
    let code = "my_func(50.5, \"abc123\")";
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
fn member_expression_function_call() {
    let code = "console.log(50.5)";
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
fn function_declaration() {
    let code = "function add(n1: number, n2: number): number {
        return n1 + n2;
    }";
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
fn function_expression() {
    let code = "const sum = function(n1: number, n2: number): number {
        return n1 + n2;
    }";
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
fn arrow_function() {
    let code = "const sum = (n1: number, n2: number): number => n1 + n2;";
    let mut parser = Parser::new(&code);
    let result = parser.parse();
    if let Err(err) = result {
        err.print(&code);
        panic!();
    }
    let program = result.unwrap();
    assert_snapshot!(format!("{:#?}", program));
}
