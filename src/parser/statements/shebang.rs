use crate::parser::Node;

#[derive(Debug, PartialEq)]
pub struct Shebang {
    node: Node,
    value: String,
}
