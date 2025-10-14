use parser_derive::Stmt;

use crate::ast_types::{literal::Literal, node_objects::Node, statements::Statement};

// es5
// interface Directive <: ExpressionStatement {
//     expression: Literal;
//     directive: string;
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct Directive {
    pub node: Node,
    pub expression: Literal,
    pub directive: String,
}
