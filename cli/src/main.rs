const FILE_PATH: &str = "test.ts";

fn main() {
    let code = std::fs::read_to_string(FILE_PATH).expect("Failed to read input file");
    let mut parser = parser::Parser::new(&code);
    let result = parser.parse();

    if let Err(err) = result {
        panic!("{:?}", err);
    }

    let ast = result.unwrap();

    let semantic_result = semantic::analyze(&ast);

    for err in semantic_result.iter() {
        eprintln!("\n\n{}", &err);
    }

    if semantic_result.len() == 0 {
        println!("No errors found!");
    }
}
