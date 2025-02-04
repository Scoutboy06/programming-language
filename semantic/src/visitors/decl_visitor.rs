use super::Visitor;
use crate::{symbol::SymbolKind, CheckerContext, CompilationError, ErrorSeverity};
use parser::{
    expressions::{types::TypeValue, Expression, Literal},
    nodes::program::Program,
    statements::{Statement, VariableDeclaration},
};

pub struct DeclVisitor {}

impl DeclVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> Visitor for DeclVisitor {
    fn visit_program(&self, ast: &Program, ctx: &mut CheckerContext) {
        ast.body
            .iter()
            .for_each(|stmt| self.visit_statement(stmt, ctx));
    }

    fn visit_statement(&self, stmt: &Statement, ctx: &mut CheckerContext) {
        use Statement as S;

        match stmt {
            S::VariableDeclaration(decl) => self.visit_variable_declaration(decl, ctx),
            _ => todo!(),
        }
    }

    fn visit_expression(&self, expr: &Expression, ctx: &mut CheckerContext) -> SymbolKind {
        use Expression as E;
        match expr {
            E::Literal(lit) => match **lit {
                Literal::BooleanLiteral(_) => SymbolKind::Boolean,
                Literal::NullLiteral(_) => SymbolKind::Null,
                Literal::NumberLiteral(_) => SymbolKind::Number,
                Literal::StringLiteral(_) => SymbolKind::String,
            },
            E::Identifier(id) => {
                // Check if referenced variable exists
                let Some(symbol) = ctx.get_symbol(id.name.clone()) else {
                    ctx.report_error(CompilationError {
                        message: "Unknown variable".into(),
                        node: id.node,
                        severity: ErrorSeverity::Critical,
                    });
                    return SymbolKind::Unknown;
                };

                // Check if referenced variable has a value
                let Some(type_val) = symbol.unfolded_type.as_ref() else {
                    ctx.report_error(CompilationError {
                        message: "Variable used before initialization".into(),
                        node: symbol.declared_at,
                        severity: ErrorSeverity::Critical,
                    });
                    return SymbolKind::Unknown;
                };

                type_val.to_owned()
            }
            _ => todo!(),
        }
    }

    fn visit_variable_declaration(&self, decl: &VariableDeclaration, ctx: &mut CheckerContext) {
        for d in decl.declarations.iter() {
            let annotated_type = d.type_annotation.as_ref().map(|ann| &ann.type_value);
            let init_type = d.init.as_ref().map(|expr| self.visit_expression(expr, ctx));

            let matches = annotated_type
                .zip(init_type.as_ref())
                .map(|(ann, init)| init.matches(ann))
                .unwrap_or(true);

            if !matches {
                ctx.report_error(CompilationError {
                    message: "Mismatched types".to_string(),
                    node: d.init.as_ref().unwrap().node().clone(),
                    severity: ErrorSeverity::Critical,
                });
            }

            ctx.symbols.add(
                d.id.name.to_owned(),
                init_type.to_owned(),
                annotated_type.cloned(),
                d.node,
            );
        }
    }
}
