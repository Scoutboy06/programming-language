use crate::ast_types::node_objects::Node;

use super::Expression;
use parser_derive::Expr;

// es5
// interface NewExpression <: Expression {
//     type: "NewExpression";
//     callee: Expression;
//     arguments: [ Expression ];
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct NewExpression {
    pub node: Node,
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

// #[derive(Debug, PartialEq, Clone)]
// pub enum NewArgument {
//     Expression(Expression),
//     SpreadElement(SpreadElement),
// }
