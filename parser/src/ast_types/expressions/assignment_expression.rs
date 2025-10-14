use crate::ast_types::{node_objects::Node, patterns::pattern::Pattern};

use super::Expression;
use lexer::AssignmentOperator;
use parser_derive::Expr;

// es5
// interface AssignmentExpression <: Expression {
//     type: "AssignmentExpression";
//     operator: AssignmentOperator;
//     left: Pattern | Expression;
//     right: Expression;
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct AssignmentExpression {
    pub node: Node,
    pub operator: AssignmentOperator,
    pub left: Pattern,
    pub right: Expression,
}
