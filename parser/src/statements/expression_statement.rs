use super::Statement;
use crate::{expressions::Expression, nodes::Node};

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub node: Node,
    pub expression: Expression,
}

impl From<ExpressionStatement> for Statement {
    fn from(value: ExpressionStatement) -> Self {
        Statement::ExpressionStatement(Box::new(value))
    }
}
