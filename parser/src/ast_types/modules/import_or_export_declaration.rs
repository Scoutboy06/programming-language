use crate::ast_types::modules::{
    exports::{ExportAllDeclaration, ExportNamedDeclaration},
    imports::ImportDeclaration,
};

// es2015
// interface ImportOrExportDeclaration <: Node { }
#[derive(Debug, PartialEq, Clone)]
pub enum ImportOrExportDeclaration {
    ImportDeclaration(ImportDeclaration),
    ExportNamedDeclaration(ExportNamedDeclaration),
    ExportAllDeclaration(ExportAllDeclaration),
}
