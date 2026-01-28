use crate::ast_types::{expressions::Expression, node_objects::Node, statements::Statement};

// es5
// interface IfStatement <: Statement {
//     type: "IfStatement";
//     test: Expression;
//     consequent: Statement;
//     alternate: Statement | null;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub node: Node,
    pub test: Expression,
    pub consequent: Statement,
    pub alternate: Option<Statement>,
}
