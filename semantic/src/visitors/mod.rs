use crate::{symbol::ExprType, CheckerContext};
use parser::{
    expressions::{ArrayExpression, Expression, ObjectExpression},
    nodes::program::Program,
    statements::{Statement, VariableDeclaration},
};

pub mod decl_visitor;

pub trait Visitor {
    fn visit_program(&self, ast: &Program, ctx: &mut CheckerContext);
    fn visit_statement(&self, stmt: &Statement, ctx: &mut CheckerContext);
    fn visit_variable_declaration(&self, decl: &VariableDeclaration, ctx: &mut CheckerContext);
    fn visit_expression(
        &self,
        expr: &Expression,
        expected_type: Option<&ExprType>,
        ctx: &mut CheckerContext,
    ) -> ExprType;
    fn visit_object_expression(
        &self,
        obj: &ObjectExpression,
        expected_type: Option<&ExprType>,
        ctx: &mut CheckerContext,
    ) -> ExprType;
    fn visit_array_expression(
        &self,
        arr: &ArrayExpression,
        expected_type: Option<&ExprType>,
        ctx: &mut CheckerContext,
    ) -> ExprType;
}
