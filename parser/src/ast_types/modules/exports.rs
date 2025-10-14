use crate::ast_types::{
    classes::{class::ClassBody, class_declaration::ClassDeclaration},
    declarations::{declaration::Declaration, function_declaration::FunctionDeclaration},
    expressions::Expression,
    identifier::Identifier,
    literal::Literal,
    node_objects::Node,
    patterns::pattern::Pattern,
    statements::FunctionBody,
};

// es2015
// interface ExportNamedDeclaration <: ImportOrExportDeclaration {
//     type: "ExportNamedDeclaration";
//     declaration: Declaration | null;
//     specifiers: [ ExportSpecifier ];
//     source: Literal | null;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ExportNamedDeclaration {
    pub node: Node,
    pub declaration: Option<Declaration>,
    pub specifiers: Vec<ExportSpecifier>,
    pub source: Option<Literal>,
}

// es2015
// interface ExportSpecifier <: ModuleSpecifier {
//     type: "ExportSpecifier";
//     exported: Identifier;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ExportSpecifier {
    pub node: Node,
    pub local: Identifier,
    pub exported: Identifier,
}

// es2015
// interface AnonymousDefaultExportedFunctionDeclaration <: Function {
//     type: "FunctionDeclaration";
//     id: null;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct AnonymousDefaultExportedFunctionDeclaration {
    pub node: Node,
    pub params: Vec<Pattern>,
    pub body: FunctionBody,
    pub generator: bool,
}

// es2015
// interface AnonymousDefaultExportedClassDeclaration <: Class {
//     type: "ClassDeclaration";
//     id: null;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct AnonymousDefaultExportedClassDeclaration {
    pub node: Node,
    pub super_class: Option<Expression>,
    pub body: ClassBody,
}

// es2015
// interface ExportDefaultDeclaration <: ImportOrExportDeclaration {
//     type: "ExportDefaultDeclaration";
//     declaration: AnonymousDefaultExportedFunctionDeclaration | FunctionDeclaration | AnonymousDefaultExportedClassDeclaration | ClassDeclaration | Expression;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ExportDefaultDeclaration {
    pub node: Node,
    pub declaration: ExportDefaultDeclarationDeclaration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExportDefaultDeclarationDeclaration {
    AnonymousDefaultExportedFunctionDeclaration(AnonymousDefaultExportedFunctionDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    AnonymousDefaultExportedClassDeclaration(AnonymousDefaultExportedClassDeclaration),
    ClassDeclaration(ClassDeclaration),
    Expression(Expression),
}

// es2015
// interface ExportAllDeclaration <: ImportOrExportDeclaration {
//     type: "ExportAllDeclaration";
//     source: Literal;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ExportAllDeclaration {
    pub node: Node,
    pub source: Literal,
}
