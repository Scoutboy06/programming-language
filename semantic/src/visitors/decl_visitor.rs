use super::Visitor;
use crate::{types::ExpressionType, CheckerContext, CompilationError, ErrorSeverity};
use parser::{
    expressions::{Expression, Literal},
    nodes::program::Program,
    statements::{Statement, VariableDeclaration},
};

pub struct DeclVisitor {}

impl DeclVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Visitor for DeclVisitor {
    fn visit_program(&mut self, ast: &Program, ctx: &mut CheckerContext) {
        ast.body
            .iter()
            .for_each(|stmt| self.visit_statement(stmt, ctx));
    }

    fn visit_statement(&mut self, stmt: &Statement, ctx: &mut CheckerContext) {
        use Statement as S;

        match stmt {
            S::VariableDeclaration(decl) => self.visit_variable_declaration(decl, ctx),
            _ => todo!(),
        }
    }

    fn visit_expression(&mut self, expr: &Expression, _ctx: &mut CheckerContext) -> ExpressionType {
        use Expression as E;
        match expr {
            E::Literal(lit) => match **lit {
                Literal::BooleanLiteral(_) => ExpressionType::Boolean,
                Literal::NullLiteral(_) => ExpressionType::Null,
                Literal::NumberLiteral(_) => ExpressionType::Number,
                Literal::StringLiteral(_) => ExpressionType::String,
            },
            _ => todo!(),
        }
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration, ctx: &mut CheckerContext) {
        for d in decl.declarations.iter() {
            let ann_type = d.type_annotation.as_ref().map(|ann| &ann.type_value);
            let init_type = d.init.as_ref().map(|expr| self.visit_expression(expr, ctx));
            // dbg!(&ann_type);
            // dbg!(&init_type);

            let matches = ann_type
                .zip(init_type)
                .map(|(ann, init)| init.matches(ann))
                .unwrap_or(true);

            if !matches {
                ctx.report_error(CompilationError {
                    message: "Mismatched types".into(),
                    node: d.init.as_ref().unwrap().node().clone(),
                    severity: ErrorSeverity::Critical,
                });
            }

            ctx.add_symbol(d.id.name.clone(), ann_type.cloned(), d.node);
        }
    }
}
