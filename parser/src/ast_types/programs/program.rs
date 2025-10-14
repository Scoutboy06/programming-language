use crate::ast_types::statements::Statement;

// es5
// interface Program <: Node {
//     type: "Program";
//     body: [ Directive | Statement ];
// }
//
// es2015
// extend interface Program {
//     sourceType: "script" | "module";
//     body: [ Statement | ImportOrExportDeclaration ];
// }
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub source_type: SourceType,
    pub body: Vec<ProgramBody>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SourceType {
    Script,
    Module,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProgramBody {
    Statement(Statement),
    ImportOrExportDeclaration(ImportOrExportDeclaration),
}
