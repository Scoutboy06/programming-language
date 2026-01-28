use crate::ast_types::{
    node_objects::Node,
    statements::{Directive, Statement},
};

// es5
// interface BlockStatement <: Statement {
//     type: "BlockStatement";
//     body: [ Statement ];
// }
#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub node: Node,
    pub body: Vec<Statement>,
}

// es5
// interface FunctionBody <: BlockStatement {
//     body: [ Directive | Statement ];
// }
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    pub node: Node,
    pub body: Vec<FunctionBodyBody>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionBodyBody {
    Directive(Directive),
    Statement(Statement),
}

// es2022
// interface StaticBlock <: BlockStatement {
//     type: "StaticBlock";
// }
#[derive(Debug, Clone, PartialEq)]
pub struct StaticBlock {
    pub node: Node,
    pub body: Vec<Statement>,
}
