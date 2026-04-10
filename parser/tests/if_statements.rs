use parser::Parser;

#[test]
fn if_statement() {
    let code = "if (val && true || false) {}";
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
fn if_else_statement() {
    let code = "if (val && true) {
         return true;
     } else {
         return 50.5;
     }";
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
fn if_if_else_else_statement() {
    let code = "if(foo && true) {}
     else if(false || bar) {}
     else {}";
    let mut parser = Parser::new(code);
    let result = parser.parse();

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

    let program = result.unwrap();
    insta::assert_snapshot!(format!("{:#?}", program));
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

    if let Err(err) = result {
        err.print(&code);
        panic!();
    }

    let program = result.unwrap();
    insta::assert_snapshot!(format!("{:#?}", program));
}
