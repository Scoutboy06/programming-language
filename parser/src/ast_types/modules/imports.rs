use crate::ast_types::{identifier::Identifier, literal::Literal, node_objects::Node};

// es2015
// interface ImportDeclaration <: ImportOrExportDeclaration {
//     type: "ImportDeclaration";
//     specifiers: [ ImportSpecifier | ImportDefaultSpecifier | ImportNamespaceSpecifier ];
//     source: Literal;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ImportDeclaration {
    pub node: Node,
    pub specifiers: Vec<ImportDeclarationSpecifier>,
    pub source: Literal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportDeclarationSpecifier {
    ImportSpecifier(ImportSpecifier),
    ImportDefaultSpecifier(ImportDefaultSpecifier),
    ImportNamespaceSpecifier(ImportNamespaceSpecifier),
}

// es2015
// interface ImportSpecifier <: ModuleSpecifier {
//     type: "ImportSpecifier";
//     imported: Identifier;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ImportSpecifier {
    pub node: Node,
    pub local: Identifier,
    pub imported: Identifier,
}

// es2015
// interface ImportDefaultSpecifier <: ModuleSpecifier {
//     type: "ImportDefaultSpecifier";
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ImportDefaultSpecifier {
    pub node: Node,
    pub local: Identifier,
}

// es2015
// interface ImportNamespaceSpecifier <: ModuleSpecifier {
//     type: "ImportNamespaceSpecifier";
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ImportNamespaceSpecifier {
    pub node: Node,
    pub local: Identifier,
}
