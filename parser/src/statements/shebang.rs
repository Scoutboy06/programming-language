use crate::nodes::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct Shebang {
    node: Node,
    value: String,
}
