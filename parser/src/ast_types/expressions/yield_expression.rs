use crate::ast_types::node_objects::Node;

use super::Expression;

// es2015
// interface YieldExpression <: Expression {
//     type: "YieldExpression";
//     argument: Expression | null;
//     delegate: boolean;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct YieldExpression {
    pub node: Node,
    pub argument: Option<Expression>,
    pub delegate: bool,
}
