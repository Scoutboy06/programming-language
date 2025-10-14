use crate::ast_types::node_objects::Node;

use super::Expression;
use parser_derive::Expr;

// es2015
// interface Super <: Node {
//     type: "Super";
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct Super {
    pub node: Node,
}
