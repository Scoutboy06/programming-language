use super::{Expression, TypeAnnotation};
use crate::{nodes::Node, statements::Parameter};

#[derive(Debug, Clone, PartialEq)]
pub struct ArrowFunctionExpression {
    pub node: Node,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Expression,
}

impl From<ArrowFunctionExpression> for Expression {
    fn from(value: ArrowFunctionExpression) -> Self {
        Expression::ArrowFunctionExpression(Box::new(value))
    }
}
