use std::{fs, path::Path};

use parser::Parser;

#[test]
fn parse_jquery() {
    let testing = Path::new("../jquery-3.7.1.js");
    // dbg!(&testing.as_os_str());

    // Use the absolute path to the file from the root of the crate
    let base_path = Path::new(env!("CARGO_MANIFEST_DIR")); // This points to the root of the crate
    let file_path = base_path.join(testing);

    // dbg!(&file_path);

    let source_code = fs::read_to_string(file_path).expect("Failed to open file");

    let mut parser = Parser::new(&source_code);
    let result = parser.parse();

    if let Err(err) = result {
        err.print(&source_code);
        panic!();
    }
    // assert!(result.is_ok());
}
