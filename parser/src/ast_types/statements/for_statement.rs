use parser_derive::Stmt;

use super::Statement;
use crate::ast_types::node_objects::Node;

// es5
// interface ForStatement <: Statement {
//     type: "ForStatement";
//     init: VariableDeclaration | Expression | null;
//     test: Expression | null;
//     update: Expression | null;
//     body: Statement;
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ForStatement {
    pub node: Node,
    pub init: Option<ForInit>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Statement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForInit {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}
