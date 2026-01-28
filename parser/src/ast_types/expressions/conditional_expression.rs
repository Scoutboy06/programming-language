use crate::ast_types::{expressions::Expression, node_objects::Node};

// es5
// interface ConditionalExpression <: Expression {
//     type: "ConditionalExpression";
//     test: Expression;
//     alternate: Expression;
//     consequent: Expression;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalExpression {
    pub node: Node,
    pub test: Expression,
    pub alternate: Expression,
    pub consequent: Expression,
}
