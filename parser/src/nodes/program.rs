use super::Node;
use crate::statements::{Directive, Statement};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub node: Node,
    pub body: Vec<ProgramBody>,
}

#[derive(Debug, PartialEq)]
pub enum ProgramBody {
    Directive(Directive),
    Statement(Statement),
}
