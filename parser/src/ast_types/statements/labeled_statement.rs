use crate::ast_types::{identifier::Identifier, node_objects::Node, statements::Statement};

// es5
// interface LabeledStatement <: Statement {
//     type: "LabeledStatement";
//     label: Identifier;
//     body: Statement;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct LabeledStatement {
    pub node: Node,
    pub label: Identifier,
    pub body: Statement,
}
