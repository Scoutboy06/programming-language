use super::{BlockStatement, Identifier, Statement};
use crate::{
    expressions::types::{TypeAnnotation, TypeParameterDeclaration},
    nodes::Node,
};

#[derive(Debug, PartialEq, Clone)]
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

impl From<FunctionDeclaration> for Statement {
    fn from(value: FunctionDeclaration) -> Self {
        Statement::FunctionDeclaration(Box::new(value))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub node: Node,
    pub identifier: Identifier,
    pub type_annotation: Option<TypeAnnotation>,
    pub optional: bool,
}
