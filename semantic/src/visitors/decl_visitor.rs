use super::Visitor;
use crate::CheckerContext;
use parser::{
    expressions::types::TypeValue,
    nodes::program::Program,
    statements::{Statement, VariableDeclaration},
};
use std::collections::HashMap;
use string_cache::DefaultAtom as Atom;

pub struct DeclVisitor {}

impl DeclVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Visitor for DeclVisitor {
    fn visit_program(&mut self, ast: &Program, ctx: &mut CheckerContext) {
        use Statement as S;

        for stmt in ast.body.iter() {
            match stmt {
                S::VariableDeclaration(decl) => self.visit_variable_declaration(decl, ctx),
                _ => todo!(),
            }
        }
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration, ctx: &mut CheckerContext) {
        for d in decl.declarations.iter() {
            // let type_value: Option<TypeValue> = d.type_annotation.ok_or_else(|| d.init);
            // ctx.add_symbol(d.id.clone(), d., declared_at);
        }
    }
}
