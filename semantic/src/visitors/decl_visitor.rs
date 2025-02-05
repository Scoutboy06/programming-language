use super::Visitor;
use crate::{
    symbol::{ObjectType, SymbolKind},
    CheckerContext, ErrorSeverity,
};
use parser::{
    expressions::{Expression, Key, Literal, ObjectItem},
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
                    ctx.report_error(
                        "Unknown variable".to_string(),
                        id.node,
                        ErrorSeverity::Critical,
                    );
                    return SymbolKind::Unknown;
                };

                // Check if referenced variable has a value
                let Some(type_val) = symbol.unfolded_type.as_ref() else {
                    ctx.report_error(
                        "Variable used before initialization".into(),
                        symbol.declared_at,
                        ErrorSeverity::Critical,
                    );
                    return SymbolKind::Unknown;
                };

                type_val.to_owned()
            }
            E::ObjectExpression(obj) => {
                let mut key_type = SymbolKind::Unknown;
                let mut value_type = SymbolKind::Unknown;

                for item in obj.items.iter() {
                    match item {
                        ObjectItem::KV(kv) => {
                            match &kv.key {
                                Key::Identifier(_) | Key::StringLiteral(_) => {
                                    key_type.extend(&SymbolKind::String)
                                }
                                Key::ComputedProperty(key) => {
                                    let expr_kind = self.visit_expression(&key.expression, ctx);
                                    key_type.extend(&expr_kind);
                                }
                            }
                            let val_kind = self.visit_expression(&kv.value, ctx);
                            value_type.extend(&val_kind);
                        }
                        ObjectItem::Identifier(id) => {
                            key_type.extend(&SymbolKind::String);

                            let Some(sym) = ctx.get_symbol(id.name.clone()) else {
                                ctx.report_error(
                                    "Unknown variable".to_string(),
                                    id.node,
                                    ErrorSeverity::Critical,
                                );
                                continue;
                            };

                            let Some(t) = &sym.unfolded_type else {
                                ctx.report_error(
                                    "Variable used before initialization".to_string(),
                                    id.node,
                                    ErrorSeverity::Critical,
                                );
                                continue;
                            };

                            value_type.extend(&t);
                        }
                        _ => todo!(),
                    };
                }

                dbg!(&key_type);
                dbg!(&value_type);

                SymbolKind::Object(Box::new(ObjectType {
                    key_type,
                    value_type,
                }))
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
                .map(|(ann, init)| init.matches(ann, ctx))
                .unwrap_or(true);

            if !matches {
                ctx.report_error(
                    "Mismatched types".to_string(),
                    d.init.as_ref().unwrap().node().clone(),
                    ErrorSeverity::Critical,
                );
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
