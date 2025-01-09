use crate::nodes::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct Shebang {
    pub node: Node,
    pub value: String,
}
