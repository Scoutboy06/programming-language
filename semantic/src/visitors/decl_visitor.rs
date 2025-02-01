use super::Visitor;
use crate::{types::ExpressionType, CheckerContext, CompilationError, ErrorSeverity};
use parser::{
    expressions::{
        types::{TypeReference, TypeValue},
        Expression, Literal,
    },
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

    fn visit_expression(&self, expr: &Expression, ctx: &mut CheckerContext) -> ExpressionType {
        dbg!(&expr);
        use Expression as E;
        match expr {
            E::Literal(lit) => match **lit {
                Literal::BooleanLiteral(_) => ExpressionType::Boolean,
                Literal::NullLiteral(_) => ExpressionType::Null,
                Literal::NumberLiteral(_) => ExpressionType::Number,
                Literal::StringLiteral(_) => ExpressionType::String,
            },
            E::Identifier(id) => {
                // let symbol = ctx.get_symbol(id.name.clone());
                let Some(symbol) = ctx.get_symbol(id.name.clone()) else {
                    ctx.report_error(CompilationError {
                        message: "Uninitialized variable".into(),
                        node: id.node,
                        severity: ErrorSeverity::Critical,
                    });
                    return ExpressionType::Unknown;
                };

                let Some(type_val) = symbol.type_value.as_ref() else {
                    ctx.report_error(CompilationError {
                        message: "Variable used before initialization".into(),
                        node: symbol.declared_at,
                        severity: ErrorSeverity::Critical,
                    });
                    return ExpressionType::Unknown;
                };

                match type_val {
                    TypeValue::TypeReference(_type_reference) => todo!(),
                    TypeValue::KeywordType(_keyword_type) => todo!(),
                    TypeValue::ArrayType(_array_type) => todo!(),
                    TypeValue::TypeLiteral(_type_literal) => todo!(),
                }
            }
            _ => todo!(),
        }
    }

    fn visit_variable_declaration(&self, decl: &VariableDeclaration, ctx: &mut CheckerContext) {
        for d in decl.declarations.iter() {
            let ann_type = d.type_annotation.as_ref().map(|ann| &ann.type_value);
            let init_type = d.init.as_ref().map(|expr| self.visit_expression(expr, ctx));

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
