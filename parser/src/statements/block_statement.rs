use super::Statement;
use crate::nodes::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub node: Node,
    pub statements: Vec<Statement>,
}

impl From<BlockStatement> for Statement {
    fn from(value: BlockStatement) -> Self {
        Statement::BlockStatement(Box::new(value))
    }
}
