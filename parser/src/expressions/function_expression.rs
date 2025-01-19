use super::{types::TypeAnnotation, Expression};
use crate::{
    nodes::Node,
    statements::{BlockStatement, Identifier, Parameter},
};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionExpression {
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: BlockStatement,
    pub is_generator: bool,
    pub is_async: bool,
}

impl From<FunctionExpression> for Expression {
    fn from(value: FunctionExpression) -> Self {
        Expression::FunctionExpression(Box::new(value))
    }
}
