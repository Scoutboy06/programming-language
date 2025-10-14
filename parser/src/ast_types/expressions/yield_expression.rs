use crate::ast_types::node_objects::Node;

use super::Expression;
use parser_derive::Expr;

// es2015
// interface YieldExpression <: Expression {
//     type: "YieldExpression";
//     argument: Expression | null;
//     delegate: boolean;
// }
#[derive(Debug, Clone, PartialEq, Expr)]
pub struct YieldExpression {
    pub node: Node,
    pub argument: Option<Expression>,
    pub delegate: bool,
}
