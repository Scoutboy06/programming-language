use parser::nodes::program::Program;
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Module, TypeSection, ValType,
};

use crate::Compiler;

pub struct WasmCompiler<'a> {
    program: &'a Program,
}

impl<'a> Compiler<'a> for WasmCompiler<'a> {
    fn new(program: &'a Program) -> Self {
        Self { program }
    }

    fn compile(&self) {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        let params = vec![ValType::I32, ValType::I32];
        let results = vec![ValType::I32];
        types.ty().function(params, results);
        module.section(&types);

        let mut functions = FunctionSection::new();
        let type_index = 0;
        functions.function(type_index);
        module.section(&functions);

        let mut exports = ExportSection::new();
        exports.export("f", ExportKind::Func, 0);
        module.section(&exports);

        let mut codes = CodeSection::new();
        let locals = vec![];
        let mut f = Function::new(locals);
        f.instructions().local_get(0).local_get(1).i32_add().end();
        codes.function(&f);
        module.section(&codes);

        let wasm_bytes = module.finish();

        std::fs::write("out.wasm", wasm_bytes).expect("Failed to write to file");
    }
}
