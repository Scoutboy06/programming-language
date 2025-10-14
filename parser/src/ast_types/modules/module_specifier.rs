use crate::ast_types::modules::{
    exports::ExportSpecifier,
    imports::{ImportDefaultSpecifier, ImportNamespaceSpecifier, ImportSpecifier},
};

// es2015
// interface ModuleSpecifier <: Node {
//     local: Identifier;
// }
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleSpecifier {
    ImportSpecifier(Box<ImportSpecifier>),
    ImportDefaultSpecifier(Box<ImportDefaultSpecifier>),
    ImportNamespaceSpecifier(Box<ImportNamespaceSpecifier>),
    ExportSpecifier(Box<ExportSpecifier>),
}
