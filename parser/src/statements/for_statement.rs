use super::Statement;
use crate::{expressions::Expression, nodes::Node};

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub node: Node,
    pub initializer: Statement,
    pub condition: Expression,
    pub update: Statement,
    pub body: Statement,
}

impl From<ForStatement> for Statement {
    fn from(value: ForStatement) -> Self {
        Statement::ForStatement(Box::new(value))
    }
}
