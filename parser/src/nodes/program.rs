use super::Node;
use crate::statements::Shebang;
use crate::statements::Statement;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub node: Node,
    pub shebang: Option<Box<Shebang>>,
    pub body: Vec<Statement>,
}
