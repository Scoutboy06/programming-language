use crate::ast_types::{
    identifier::Identifier,
    node_objects::Node,
    patterns::pattern::Pattern,
    statements::{FunctionBody, Statement},
};
use parser_derive::Stmt;

// es5
// interface FunctionDeclaration <: Function, Declaration {
//     type: "FunctionDeclaration";
//     id: Identifier;
// }
#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct FunctionDeclaration {
    pub node: Node,
    pub id: Identifier,
    pub generator: bool,
    pub type_parameters: Option<TypeParameterDeclaration>,
    pub params: Vec<Pattern>,
    pub return_type: Option<TypeAnnotation>,
    pub body: FunctionBody,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub node: Node,
    pub identifier: Identifier,
    pub type_annotation: Option<TypeAnnotation>,
    pub optional: bool,
}
