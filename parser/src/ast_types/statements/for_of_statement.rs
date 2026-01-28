use crate::ast_types::{
    expressions::Expression,
    node_objects::Node,
    statements::{ForInOrOfLeft, Statement},
};

// es2015
// interface ForOfStatement <: ForInStatement {
//     type: "ForOfStatement";
// }
//
// es2018
// extend interface ForOfStatement {
//   await: boolean;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ForOfStatement {
    pub node: Node,
    pub left: ForInOrOfLeft,
    pub right: Expression,
    pub body: Statement,
    pub _await: bool,
}
