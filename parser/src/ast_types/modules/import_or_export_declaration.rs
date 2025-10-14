use crate::ast_types::modules::{
    exports::{ExportAllDeclaration, ExportNamedDeclaration},
    imports::ImportDeclaration,
};

// es2015
// interface ImportOrExportDeclaration <: Node { }
pub enum ImportOrExportDeclaration {
    ImportDeclaration(ImportDeclaration),
    ExportNamedDeclaration(ExportNamedDeclaration),
    ExportAllDeclaration(ExportAllDeclaration),
}
