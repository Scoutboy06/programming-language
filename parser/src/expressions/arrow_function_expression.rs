use super::{types::TypeAnnotation, Expression};
use crate::{
    nodes::Node,
    statements::{Parameter, Statement},
};
use parser_derive::Expr;

#[derive(Debug, Clone, PartialEq, Expr)]
pub struct ArrowFunctionExpression {
    pub node: Node,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Statement,
}
