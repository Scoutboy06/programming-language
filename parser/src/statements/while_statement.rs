use super::Statement;
use crate::{expressions::Expression, nodes::Node};

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub node: Node,
    pub condition: Expression,
    pub body: Statement,
}

impl From<WhileStatement> for Statement {
    fn from(value: WhileStatement) -> Self {
        Statement::WhileStatement(Box::new(value))
    }
}
