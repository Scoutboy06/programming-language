use crate::ast_types::node_objects::Node;

use super::Expression;
use parser_derive::Expr;

// es5
// interface ConditionalExpression <: Expression {
//     type: "ConditionalExpression";
//     test: Expression;
//     alternate: Expression;
//     consequent: Expression;
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct ConditionalExpression {
    pub node: Node,
    pub test: Expression,
    pub alternate: Expression,
    pub consequent: Expression,
}
