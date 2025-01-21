use super::{BlockStatement, Statement};
use crate::{
    expressions::{
        types::{TypeAnnotation, TypeParameterDeclaration},
        Identifier,
    },
    nodes::Node,
};
use parser_derive::Stmt;

#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct FunctionDeclaration {
    pub node: Node,
    pub is_async: bool,
    pub is_generator: bool,
    pub id: Identifier,
    pub type_parameters: Option<TypeParameterDeclaration>,
    pub params: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub node: Node,
    pub identifier: Identifier,
    pub type_annotation: Option<TypeAnnotation>,
    pub optional: bool,
}
