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
#[derive(Debug, PartialEq)]
pub struct Program {
    pub source_type: SourceType,
    pub body: Vec<ProgramBody>,
}

#[derive(Debug, PartialEq)]
pub enum SourceType {
    Script,
    Module,
}

#[derive(Debug, PartialEq)]
pub enum ProgramBody {
    Statement(Statement),
    ImportOrExportDeclaration(ImportOrExportDeclaration),
}
