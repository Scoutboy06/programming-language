use crate::ast_types::{expressions::Expression, patterns::pattern::Pattern};

// es2015
// interface AssignmentPattern <: Pattern {
//     type: "AssignmentPattern";
//     left: Pattern;
//     right: Expression;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentPattern {
    pub left: Pattern,
    pub right: Expression,
}
