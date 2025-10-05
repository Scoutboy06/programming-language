use parser::Parser;

const FILE_PATH: &str = "test.ts";

fn main() {
    let code = std::fs::read_to_string(FILE_PATH).expect("Failed to read input file");
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    if let Err(err) = result {
        panic!("{:?}", err);
    }

    let ast = result.unwrap();

    dbg!(ast);
}
