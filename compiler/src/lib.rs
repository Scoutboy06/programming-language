mod targets;

use parser::{nodes::program::Program, Parser};

#[derive(Debug, Clone, Copy)]
pub enum CompilerTarget {
    Wasm,
}

pub trait Compiler<'a> {
    fn new(program: &'a Program) -> Self;
    fn compile(&mut self, out_path: &str);
}

pub fn compile(code: &str, target: CompilerTarget, out_path: &str) {
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    if let Err(err) = result {
        panic!("{:?}", err);
    }

    let ast = result.unwrap();

    let semantic_result = semantic::analyze(&ast);

    for err in semantic_result.iter() {
        eprintln!("\n\n{}", &err);
    }

    if semantic_result.len() != 0 {
        return;
    }

    match target {
        CompilerTarget::Wasm => {
            let mut compiler = targets::wasm::WasmCompiler::new(&ast);
            compiler.compile(out_path);
        }
    }
}
