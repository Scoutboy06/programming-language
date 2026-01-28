use crate::ast_types::node_objects::Node;

// es2020
// interface ChainExpression <: Expression {
//   type: "ChainExpression";
//   expression: ChainElement;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ChainExpression {
    pub node: Node,
    pub expression: ChainElement,
}

// es2020
// interface ChainElement <: Node {
//   optional: boolean;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ChainElement {
    pub node: Node,
    pub optional: bool,
}
