use parser_derive::FromVariants;

use crate::ast_types::{
    declarations::{
        function_declaration::FunctionDeclaration, variable_declaration::VariableDeclaration,
    },
    node_objects::Node,
};

// es5
// interface Declaration <: Statement { }
#[derive(Debug, Clone, PartialEq, FromVariants)]
pub enum Declaration {
    FunctionDeclaration(Box<FunctionDeclaration>),
    VariableDeclaration(Box<VariableDeclaration>),
}

impl Declaration {
    pub fn node(&self) -> &Node {
        match self {
            Self::FunctionDeclaration(f) => &f.node,
            Self::VariableDeclaration(v) => &v.node,
        }
    }
}
