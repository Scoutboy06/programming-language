use crate::ast_types::node_objects::Node;

use super::Expression;
use parser_derive::Expr;

// es5
// interface CallExpression <: Expression {
//     type: "CallExpression";
//     callee: Expression;
//     arguments: [ Expression ];
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct CallExpression {
    pub node: Node,
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}
