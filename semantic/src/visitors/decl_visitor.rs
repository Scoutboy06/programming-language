use crate::{
    symbol::Symbol,
    types::{FunctionType, ResolvedType},
    CheckerContext,
};
use parser::{
    expressions::{ArrayExpression, Expression, Key, ObjectExpression, ObjectItem},
    nodes::program::Program,
    statements::{FunctionDeclaration, Statement, VariableDeclaration},
};

pub struct DeclVisitor<'a> {
    ctx: &'a mut CheckerContext,
}

impl<'a> DeclVisitor<'a> {
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
            S::FunctionDeclaration(decl) => self.visit_function_declaration(decl),
            _ => todo!(),
        }
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) {
        for d in decl.declarations.iter() {
            let ast_type = d.type_annotation.as_ref().map(|ann| &ann.type_value);
            let resolved_type = ast_type.map(|t| ResolvedType::from_type_value(t, &mut self.ctx));

            d.init.as_ref().inspect(|init| {
                self.visit_expression(init);
            });

            self.ctx
                .add_symbol(d.id.name.clone(), resolved_type, d.node.clone());
        }
    }

    fn visit_function_declaration(&mut self, decl: &FunctionDeclaration) {
        let args: Vec<Symbol> = decl
            .params
            .iter()
            .map(|param| Symbol {
                id: param.identifier.name.clone(),
                resolved_type: param
                    .type_annotation
                    .as_ref()
                    .map(|ann| ResolvedType::from_type_value(&ann.type_value, &mut self.ctx)),
                declared_at: param.node.clone(),
            })
            .collect();

        let display_ret_type = decl.return_type.as_ref().map(|t| t.type_value.to_owned());
        let unfolded_ret_type = display_ret_type
            .as_ref()
            .map(|t| ResolvedType::from_type_value(&t, &mut self.ctx));

        let resolved_type = ResolvedType::Function(Box::new(FunctionType {
            args,
            display_ret_type,
            unfolded_ret_type,
        }));

        self.ctx.add_symbol(
            decl.id.name.to_owned(),
            Some(resolved_type),
            decl.node.clone(),
        );

        decl.body.statements.iter().for_each(|stmt| {
            self.visit_statement(stmt);
        });
    }

    fn visit_expression(&self, expr: &Expression) {
        use Expression as E;
        match expr {
            E::Literal(_) => {}
            E::Identifier(_) => {}
            E::ObjectExpression(obj) => self.visit_object_expression(obj),
            E::ArrayExpression(arr) => self.visit_array_expression(arr),
            _ => todo!(),
        }
    }

    fn visit_object_expression(&self, obj: &ObjectExpression) {
        for item in obj.items.iter() {
            match item {
                ObjectItem::KV(kv) => match &kv.key {
                    Key::Identifier(_) | Key::StringLiteral(_) => {}
                    Key::ComputedProperty(prop) => self.visit_expression(&prop.expression),
                },
                ObjectItem::Identifier(_) => {}
                ObjectItem::Method(_method) => todo!(),
            };
        }
    }

    fn visit_array_expression(&self, arr: &ArrayExpression) {
        arr.items
            .iter()
            .for_each(|expr| self.visit_expression(expr));
    }
}
