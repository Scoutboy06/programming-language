use parser::{
    nodes::program::Program,
    statements::{BlockStatement, Statement},
};
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use crate::Compiler;

pub struct WasmCompiler<'a> {
    program: &'a Program,
    types: TypeSection,
    functions: FunctionSection,
    exports: ExportSection,
    codes: CodeSection,
    // scopes: todo!(),
}

impl<'a> Compiler<'a> for WasmCompiler<'a> {
    fn new(program: &'a Program) -> Self {
        let types = TypeSection::new();
        let functions = FunctionSection::new();
        let exports = ExportSection::new();
        let codes = CodeSection::new();

        Self {
            program,
            types,
            functions,
            exports,
            codes,
        }
    }

    fn compile(&mut self) {
        let mut module = Module::new();

        for stmt in self.program.body.iter() {
            self.enter_statement(stmt);
        }

        module.section(&self.types);
        module.section(&self.functions);
        module.section(&self.exports);
        module.section(&self.codes);
        let wasm_bytes = module.finish();

        std::fs::write("out.wasm", wasm_bytes).expect("Failed to write to file");
    }
}

impl<'a> WasmCompiler<'a> {
    fn push_scope(&mut self) {
        todo!()
    }

    fn pop_scope(&mut self) {
        todo!()
    }

    fn enter_statement(&mut self, stmt: &Statement) -> Instruction<'a> {
        match stmt {
            Statement::FunctionDeclaration(decl) => {
                self.push_scope();

                // TODO: Change this to actual data types
                let params = decl.params.iter().map(|param| ValType::I32);
                let results = vec![ValType::I32];

                let type_index = self.types.len();
                self.types.ty().function(params, results);
                self.functions.function(type_index);

                let locals = vec![];

                let mut f = Function::new(locals);

                for s in decl.body.statements.iter() {
                    self.enter_statement(s);
                }

                self.pop_scope();
                todo!()
            }
            Statement::ReturnStatement(stmt) => Instruction::Return,
            _ => todo!("{:?}", &stmt),
        }
    }
}
