use super::Node;
use crate::parser::{Shebang, Statement};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub node: Node,
    pub shebang: Option<Box<Shebang>>,
    pub body: Vec<Statement>,
}
