use crate::ast_types::node_objects::Node;

use super::Expression;
use lexer::UpdateOperator;
use parser_derive::Expr;

// es5
// interface UpdateExpression <: Expression {
//     type: "UpdateExpression";
//     operator: UpdateOperator;
//     argument: Expression;
//     prefix: boolean;
// }
#[derive(Debug, Clone, PartialEq, Expr)]
pub struct UpdateExpression {
    pub node: Node,
    pub operator: UpdateOperator,
    pub argument: Expression,
    pub prefix: bool,
}
