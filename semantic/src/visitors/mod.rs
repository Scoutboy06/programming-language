use parser::{nodes::program::Program, statements::VariableDeclaration};

use crate::CheckerContext;

pub mod decl_visitor;

pub trait Visitor {
    fn visit_program(&mut self, ast: &Program, ctx: &mut CheckerContext);
    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration, ctx: &mut CheckerContext);
}
