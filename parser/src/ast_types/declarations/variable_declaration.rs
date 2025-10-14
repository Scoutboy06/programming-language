use parser_derive::Stmt;

use crate::ast_types::{node_objects::Node, patterns::pattern::Pattern};

// es5
// interface VariableDeclaration <: Declaration {
//     type: "VariableDeclaration";
//     declarations: [ VariableDeclarator ];
//     kind: "var";
// }
#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct VariableDeclaration {
    pub node: Node,
    pub declarations: Vec<VariableDeclarator>,
    pub kind: VariableKind,
}

// es5
// interface VariableDeclarator <: Node {
//     type: "VariableDeclarator";
//     id: Pattern;
//     init: Expression | null;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarator {
    pub node: Node,
    pub id: Pattern,
    pub type_annotation: Option<TypeAnnotation>,
    pub init: Option<Expression>,
}
