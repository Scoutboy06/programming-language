use crate::{types::ExpressionType, CheckerContext};
use parser::{
    expressions::Expression,
    nodes::program::Program,
    statements::{Statement, VariableDeclaration},
};

pub mod decl_visitor;

pub trait Visitor {
    fn visit_program(&self, ast: &Program, ctx: &mut CheckerContext);
    fn visit_statement(&self, stmt: &Statement, ctx: &mut CheckerContext);
    fn visit_expression(&self, expr: &Expression, ctx: &mut CheckerContext) -> ExpressionType;
    fn visit_variable_declaration(&self, decl: &VariableDeclaration, ctx: &mut CheckerContext);
}
