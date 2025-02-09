use parser::{
    expressions::{
        ArrayExpression, Expression, Identifier, Key, Literal, ObjectExpression, ObjectItem,
    },
    nodes::program::Program,
    statements::{Statement, VariableDeclaration},
};

use crate::{
    errors::{ErrorData, ErrorSeverity},
    types::{ExprType, ObjectType},
    CheckerContext,
};

pub struct BodyVisitor<'a> {
    ctx: &'a mut CheckerContext,
}

impl<'a> BodyVisitor<'a> {
    pub fn visit_program(ast: &Program, ctx: &'a mut CheckerContext) {
        let mut visitor = Self { ctx };
        ast.body
            .iter()
            .for_each(|stmt| visitor.visit_statement(stmt));
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        use Statement as S;
        match stmt {
            S::VariableDeclaration(decl) => self.visit_variable_declaration(decl),
            _ => todo!(),
        }
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) {
        for d in decl.declarations.iter() {
            let symbol_name = d.id.name.to_owned();
            let expected_type = {
                let symbol = self.ctx.get_symbol(symbol_name.to_owned());
                symbol.unwrap().unfolded_type.clone()
            };

            if let Some(init) = &d.init {
                let init_t = self.visit_expression(init, expected_type.as_ref());

                let symbol = self.ctx.get_symbol_mut(symbol_name).unwrap();
                if symbol.unfolded_type.is_none() {
                    symbol.unfolded_type = Some(init_t);
                }
            }
        }
    }

    fn visit_expression(
        &mut self,
        expr: &Expression,
        expected_type: Option<&ExprType>,
    ) -> ExprType {
        use Expression as E;
        match expr {
            E::Literal(lit) => self.visit_literal(lit, expected_type),
            E::ObjectExpression(obj) => self.visit_object_expression(obj, expected_type),
            E::ArrayExpression(arr) => self.visit_array_expression(arr, expected_type),
            E::Identifier(id) => self.visit_identifier(id, expected_type),
            _ => todo!(),
        }
    }

    fn visit_literal(&mut self, lit: &Literal, expected_type: Option<&ExprType>) -> ExprType {
        let expr_type = match lit {
            Literal::BooleanLiteral(_) => ExprType::Boolean,
            Literal::StringLiteral(_) => ExprType::String,
            Literal::NullLiteral(_) => ExprType::Null,
            Literal::NumberLiteral(_) => ExprType::Number,
        };

        if expected_type.is_some_and(|t| expr_type != *t) {
            self.ctx.report_error(
                ErrorData::TypeMismatch {
                    expected_type: expected_type.unwrap().to_owned(),
                    received_type: expr_type.to_owned(),
                },
                lit.node().clone(),
                ErrorSeverity::Critical,
            );
        }

        expr_type
    }

    fn visit_identifier(&mut self, id: &Identifier, expected_type: Option<&ExprType>) -> ExprType {
        let Some(symbol) = self.ctx.get_symbol(id.name.clone()).cloned() else {
            self.ctx.report_error(
                ErrorData::UnknownVariable {
                    id: id.name.to_owned(),
                },
                id.node.clone(),
                ErrorSeverity::Critical,
            );
            return ExprType::Unknown;
        };

        let Some(t) = &symbol.unfolded_type else {
            self.ctx.report_error(
                ErrorData::UseBeforeInit {
                    id: id.name.to_owned(),
                },
                id.node,
                ErrorSeverity::Critical,
            );
            return ExprType::Unknown;
        };

        if expected_type.is_some_and(|expected_type| expected_type != t) {
            self.ctx.report_error(
                ErrorData::TypeMismatch {
                    expected_type: expected_type.unwrap().to_owned(),
                    received_type: t.to_owned(),
                },
                id.node.to_owned(),
                ErrorSeverity::Critical,
            );
        }

        t.to_owned()
    }

    fn visit_object_expression(
        &mut self,
        obj: &ObjectExpression,
        expected_type: Option<&ExprType>,
    ) -> ExprType {
        let (expected_key_type, expected_value_type) =
            expected_type.map_or((None, None), |t| match t {
                ExprType::Object(obj) => (Some(&obj.key_type), Some(&obj.value_type)),
                _ => {
                    self.ctx.report_error(
                        ErrorData::TypeMismatch {
                            expected_type: expected_type.unwrap().to_owned(),
                            received_type: ExprType::Object(todo!()),
                        },
                        obj.node.clone(),
                        ErrorSeverity::Critical,
                    );
                    (None, None)
                }
            });
        let mut key_type = ExprType::Unknown;
        let mut value_type = ExprType::Unknown;

        obj.items.iter().for_each(|item| match item {
            ObjectItem::KV(kv) => {
                match &kv.key {
                    Key::Identifier(_) | Key::StringLiteral(_) => {
                        if expected_key_type.is_some_and(|t| !t.includes(&ExprType::String)) {
                            self.ctx.report_error(
                                ErrorData::TypeMismatch {
                                    expected_type: expected_key_type.unwrap().to_owned(),
                                    received_type: ExprType::String,
                                },
                                kv.key.node().clone(),
                                ErrorSeverity::Critical,
                            );
                        }
                        key_type.extend(&ExprType::String);
                    }
                    Key::ComputedProperty(key) => {
                        let expr_t = self.visit_expression(&key.expression, expected_key_type);
                        key_type.extend(&expr_t);
                    }
                };
                self.visit_expression(&kv.value, expected_value_type);
            }
            ObjectItem::Identifier(id) => {
                let id_t = self.visit_identifier(id, expected_type);
                key_type.extend(&ExprType::String);
                value_type.extend(&id_t);
            }
            ObjectItem::Method(_method) => todo!(),
        });

        ExprType::Object(Box::new(ObjectType {
            key_type,
            value_type,
        }))
    }

    fn visit_array_expression(
        &mut self,
        arr: &ArrayExpression,
        expected_type: Option<&ExprType>,
    ) -> ExprType {
        let expected_item_type = match expected_type {
            Some(t) => match t {
                ExprType::Array(arr) => Some(&**arr),
                _ => {
                    self.ctx.report_error(
                        ErrorData::TypeMismatch {
                            expected_type: expected_type.unwrap().to_owned(),
                            received_type: ExprType::Array(Box::new(todo!())),
                        },
                        arr.node.clone(),
                        ErrorSeverity::Critical,
                    );
                    None
                }
            },
            _ => None,
        };
        let mut item_type = ExprType::Unknown;

        arr.items.iter().for_each(|it| {
            let expr_t = self.visit_expression(it, expected_item_type);
            item_type.extend(&expr_t);
        });

        ExprType::Array(Box::new(item_type))
    }
}
