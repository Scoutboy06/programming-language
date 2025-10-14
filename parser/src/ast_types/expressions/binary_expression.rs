use crate::ast_types::node_objects::Node;

use super::Expression;
use lexer::BinaryOperator;
use parser_derive::Expr;

// es5
// interface BinaryExpression <: Expression {
//     type: "BinaryExpression";
//     operator: BinaryOperator;
//     left: Expression;
//     right: Expression;
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct BinaryExpression {
    pub node: Node,
    pub operator: BinaryOperator,
    pub left: Expression,
    pub right: Expression,
}
