use super::Expression;
use crate::ast_types::node_objects::Node;
use lexer::UnaryOperator;
use parser_derive::Expr;

// es5
// interface UnaryExpression <: Expression {
//     type: "UnaryExpression";
//     operator: UnaryOperator;
//     prefix: boolean;
//     argument: Expression;
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct UnaryExpression {
    pub node: Node,
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Expression,
}
