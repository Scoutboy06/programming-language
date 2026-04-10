use parser::Parser;

#[test]
fn for_loop() {
    let code = "for(let i = 0; i < 10; i++) {}";
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
fn for_loop_without_initializer() {
    let code = "for(; i < 10; i++) {}";
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
fn for_loop_without_test() {
    let code = "for(let i = 0;; i++) {}";
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
fn for_loop_without_update() {
    let code = "for(let i = 0; i < 10;) {}";
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
fn for_in_loop() {
    let code = "for(let key in obj) {}";
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
fn for_of_loop() {
    let code = "for(let key of obj) {}";
    let mut parser = Parser::new(&code);
    let result = parser.parse();
    if let Err(err) = result {
        err.print(&code);
        panic!();
    }
    let program = result.unwrap();
    insta::assert_snapshot!(format!("{:#?}", program));
}
