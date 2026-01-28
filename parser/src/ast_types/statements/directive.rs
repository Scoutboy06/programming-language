use crate::ast_types::{literal::Literal, node_objects::Node};

// es5
// interface Directive <: ExpressionStatement {
//     expression: Literal;
//     directive: string;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub node: Node,
    pub expression: Literal,
    pub directive: String,
}
