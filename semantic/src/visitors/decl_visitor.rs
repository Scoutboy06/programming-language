use super::Visitor;
use crate::{
    symbol::{ExprType, ExprTypeIncludes, ObjectType},
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

    fn visit_variable_declaration(&self, decl: &VariableDeclaration, ctx: &mut CheckerContext) {
        for d in decl.declarations.iter() {
            let annotated_type = d.type_annotation.as_ref().map(|ann| &ann.type_value);
            let ann_expr_kind = annotated_type.map(|t| ExprType::from_type_value(t, ctx));
            let init_type = d
                .init
                .as_ref()
                .map(|expr| self.visit_expression(expr, ann_expr_kind.as_ref(), ctx));

            ctx.symbols.add(
                d.id.name.to_owned(),
                init_type.to_owned(),
                annotated_type.cloned(),
                d.node,
            );
        }
    }

    fn visit_expression(
        &self,
        expr: &Expression,
        expected_type: Option<&ExprType>,
        ctx: &mut CheckerContext,
    ) -> ExprType {
        use Expression as E;
        match expr {
            E::Literal(lit) => {
                let t = match **lit {
                    Literal::BooleanLiteral(_) => ExprType::Boolean,
                    Literal::NullLiteral(_) => ExprType::Null,
                    Literal::NumberLiteral(_) => ExprType::Number,
                    Literal::StringLiteral(_) => ExprType::String,
                };
                if expected_type.is_some_and(|ex| t != *ex) {
                    ctx.report_error(
                        "Mismatched types".to_string(),
                        lit.node().clone(),
                        ErrorSeverity::Critical,
                    );
                }
                t
            }
            E::Identifier(id) => {
                // Check if referenced variable exists
                let Some(symbol) = ctx.get_symbol(id.name.clone()) else {
                    ctx.report_error(
                        "Unknown variable".to_string(),
                        id.node,
                        ErrorSeverity::Critical,
                    );
                    return ExprType::Unknown;
                };

                // Check if referenced variable has a value
                let Some(type_val) = symbol.unfolded_type.as_ref().cloned() else {
                    ctx.report_error(
                        "Variable used before initialization".into(),
                        symbol.declared_at,
                        ErrorSeverity::Critical,
                    );
                    return ExprType::Unknown;
                };

                if expected_type.is_some_and(|t| type_val != *t) {
                    ctx.report_error(
                        "Mismatched types".to_string(),
                        id.node.clone(),
                        ErrorSeverity::Critical,
                    );
                }

                type_val
            }
            E::ObjectExpression(obj) => ExprType::Object(Box::new(self.visit_object_expression(
                obj,
                expected_type,
                ctx,
            ))),
            _ => todo!(),
        }
    }

    fn visit_object_expression(
        &self,
        obj: &parser::expressions::ObjectExpression,
        expected_type: Option<&ExprType>,
        ctx: &mut CheckerContext,
    ) -> ObjectType {
        let (expected_key_type, expected_value_type) = match expected_type {
            Some(t) => match t {
                ExprType::Object(obj) => (Some(&obj.key_type), Some(&obj.value_type)),
                _ => {
                    ctx.report_error(
                        "Mismatched types".to_string(),
                        obj.node.clone(),
                        ErrorSeverity::Critical,
                    );
                    (None, None)
                }
            },
            _ => (None, None),
        };
        let mut key_type = ExprType::Unknown;
        let mut value_type = ExprType::Unknown;

        for item in obj.items.iter() {
            match item {
                ObjectItem::KV(kv) => {
                    match &kv.key {
                        Key::Identifier(_) | Key::StringLiteral(_) => {
                            if expected_key_type.is_some_and(|t| !t.includes(&ExprType::String)) {
                                ctx.report_error(
                                    "Mismatched types".to_string(),
                                    kv.key.node().clone(),
                                    ErrorSeverity::Critical,
                                );
                            }
                            key_type.extend(&ExprType::String);
                        }
                        Key::ComputedProperty(key) => {
                            let expr_kind =
                                self.visit_expression(&key.expression, expected_key_type, ctx);
                            key_type.extend(&expr_kind);
                        }
                    }
                    let val_kind = self.visit_expression(&kv.value, expected_value_type, ctx);
                    value_type.extend(&val_kind);
                }
                ObjectItem::Identifier(id) => {
                    key_type.extend(&ExprType::String);

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

        ObjectType {
            key_type,
            value_type,
        }
    }
}
