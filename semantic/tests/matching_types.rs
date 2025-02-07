use parser::Parser;
use pretty_assertions::assert_eq;
use semantic::{analyze, ErrorSeverity};

#[test]
fn string_number_mismatch() {
    let code = "let foo: string = 123;";
    let mut parser = Parser::new(&code);
    let ast = parser.parse().unwrap();
    let errors = analyze(&ast);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].severity, ErrorSeverity::Critical);
}

#[test]
fn number_number_match() {
    let code = "let foo: string = 123;";
    let mut parser = Parser::new(&code);
    let ast = parser.parse().unwrap();
    let errors = analyze(&ast);
    assert_eq!(errors.len(), 1);
}

#[test]
fn string_string_match() {
    let code = "let foo: string = \"abc\";";
    let mut parser = Parser::new(&code);
    let ast = parser.parse().unwrap();
    let errors = analyze(&ast);
    assert_eq!(errors.len(), 0);
}

#[test]
fn boolean_boolean_match() {
    let code = "let foo: boolean = true;";
    let mut parser = Parser::new(&code);
    let ast = parser.parse().unwrap();
    let errors = analyze(&ast);
    assert_eq!(errors.len(), 0);
}

#[test]
fn boolean_string_mismatch() {
    let code = "let foo: boolean = \"abc\";";
    let mut parser = Parser::new(&code);
    let ast = parser.parse().unwrap();
    let errors = analyze(&ast);
    assert_eq!(errors.len(), 1);
}

#[test]
fn string_boolean_mismatch() {
    let code = "let foo: string = true;";
    let mut parser = Parser::new(&code);
    let ast = parser.parse().unwrap();
    let errors = analyze(&ast);
    assert_eq!(errors.len(), 1);
}

#[test]
fn variable_reference_type_match() {
    let code = r#"let foo = "abc"
                        let bar: string = foo
                        
                        let n: number = 123
                        let m: number = n"#;
    let mut parser = Parser::new(&code);
    let ast = parser.parse().unwrap();
    let errors = analyze(&ast);
    assert_eq!(errors, []);
}

#[test]
fn variable_reference_type_mismatch() {
    let code = r#"let foo = 123
                        let bar: string = foo

                        let fizz = "abc"
                        let buzz: number = fizz"#;
    let mut parser = Parser::new(&code);
    let ast = parser.parse().unwrap();
    let errors = analyze(&ast);
    assert!(errors.len() == 2);
}

#[test]
fn record_string_number_match() {
    let code = r#"let obj: Record<string, number> = { days: 12, months: 2 }"#;
    let mut parser = Parser::new(&code);
    let ast = parser.parse().unwrap();
    let errors = analyze(&ast);
    assert_eq!(errors, []);
}

#[test]
fn record_string_boolean_mismatch() {
    let code = r#"let obj: Record<string, boolean> = { days: 12, months: 2 }"#;
    let mut parser = Parser::new(&code);
    let ast = parser.parse().unwrap();
    let errors = analyze(&ast);
    assert_eq!(errors.len(), 2);
}

#[test]
fn array_string_match() {
    let code = r#"let arr: number[] = [123, 456, 789];"#;
    let mut parser = Parser::new(&code);
    let ast = parser.parse().unwrap();
    let errors = analyze(&ast);
    assert_eq!(errors, []);
}

#[test]
fn array_string_mismatch() {
    let code = r#"let arr: string[] = ["abc", 123, "def", 456];"#;
    let mut parser = Parser::new(&code);
    let ast = parser.parse().unwrap();
    let errors = analyze(&ast);
    assert_eq!(errors.len(), 2);
}
