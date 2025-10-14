use parser_derive::Stmt;

use crate::ast_types::{
    declarations::{
        function_declaration::FunctionDeclaration, variable_declaration::VariableDeclaration,
    },
    statements::Statement,
};

// es5
// interface Declaration <: Statement { }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub enum Declaration {
    FunctionDeclaration(Box<FunctionDeclaration>),
    VariableDeclaration(Box<VariableDeclaration>),
}
