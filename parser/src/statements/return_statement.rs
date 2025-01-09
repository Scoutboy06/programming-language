use crate::{expressions::Expression, nodes::Node};

use super::Statement;

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub node: Node,
    pub value: Expression,
}

impl From<ReturnStatement> for Statement {
    fn from(value: ReturnStatement) -> Self {
        Statement::ReturnStatement(Box::new(value))
    }
}
