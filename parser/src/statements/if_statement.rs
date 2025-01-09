use crate::{expressions::Expression, nodes::Node};

use super::{BlockStatement, Statement};

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub node: Node,
    pub condition: Expression,
    pub body: BlockStatement,
    pub consequent: Option<Statement>,
}

impl From<IfStatement> for Statement {
    fn from(value: IfStatement) -> Self {
        Statement::IfStatement(Box::new(value))
    }
}
