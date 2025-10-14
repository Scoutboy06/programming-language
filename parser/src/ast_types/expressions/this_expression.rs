use crate::ast_types::node_objects::Node;

use super::Expression;
use parser_derive::Expr;

// es5
// interface ThisExpression <: Expression {
//     type: "ThisExpression";
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct ThisExpression {
    pub node: Node,
}
