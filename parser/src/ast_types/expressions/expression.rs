#[derive(Debug, PartialEq, Clone)]
pub enum Expression {}

impl Expression {
    pub fn node(&self) -> &Node {
        match self {}
    }
}
