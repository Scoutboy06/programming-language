use crate::ast_types::node_objects::Node;

use super::Expression;
use parser_derive::Expr;

// es5
// interface SequenceExpression <: Expression {
//     type: "SequenceExpression";
//     expressions: [ Expression ];
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct SequenceExpression {
    pub node: Node,
    pub expressions: Vec<Expression>,
}
