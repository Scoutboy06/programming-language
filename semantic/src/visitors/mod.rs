use crate::{types::ExpressionType, CheckerContext};
use parser::{
    expressions::Expression,
    nodes::program::Program,
    statements::{Statement, VariableDeclaration},
};

pub mod decl_visitor;

pub trait Visitor {
    fn visit_program(&mut self, ast: &Program, ctx: &mut CheckerContext);
    fn visit_statement(&mut self, stmt: &Statement, ctx: &mut CheckerContext);
    fn visit_expression(&mut self, expr: &Expression, ctx: &mut CheckerContext) -> ExpressionType;
    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration, ctx: &mut CheckerContext);
}
