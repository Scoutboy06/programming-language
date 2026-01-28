use crate::ast_types::{
    expressions::Expression, node_objects::Node, operators::AssignmentOperator,
    patterns::pattern::Pattern,
};

// es5
// interface AssignmentExpression <: Expression {
//     type: "AssignmentExpression";
//     operator: AssignmentOperator;
//     left: Pattern | Expression;
//     right: Expression;
// }
//
// es2015
// extend interface AssignmentExpression {
//     left: Pattern;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpression {
    pub node: Node,
    pub operator: AssignmentOperator,
    pub left: Pattern,
    pub right: Expression,
}
