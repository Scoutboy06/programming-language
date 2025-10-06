use crate::{
    symbol::Symbol,
    types::{FunctionType, ResolvedType},
    CheckerContext,
};
use parser::{
    expressions::{
        ArrayExpression, BinaryExpression, Expression, Key, ObjectExpression, ObjectItem,
    },
    nodes::program::Program,
    statements::{FunctionDeclaration, ReturnStatement, Statement, VariableDeclaration},
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
            S::ReturnStatement(stmt) => self.visit_return_statement(stmt),
            _ => todo!("{:?}", &stmt),
        }
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) {
        for d in decl.declarations.iter() {
            let ast_type = d.type_annotation.as_ref().map(|ann| &ann.type_value);
            let resolved_type = ast_type.map(|t| ResolvedType::from_ast_type(t, &mut self.ctx));

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
                    .map(|ann| ResolvedType::from_ast_type(&ann.type_value, &mut self.ctx)),
                declared_at: param.node.clone(),
            })
            .collect();

        let display_ret_type = decl.return_type.as_ref().map(|t| t.type_value.to_owned());
        let unfolded_ret_type = display_ret_type
            .as_ref()
            .map(|t| ResolvedType::from_ast_type(&t, &mut self.ctx));

        let resolved_type = ResolvedType::Function(Box::new(FunctionType {
            args: args.to_owned(),
            display_ret_type,
            unfolded_ret_type,
        }));

        self.ctx.add_symbol(
            decl.id.name.to_owned(),
            Some(resolved_type),
            decl.node.clone(),
        );

        for arg in args.iter() {
            self.ctx.add_symbol(
                arg.id.to_owned(),
                arg.resolved_type.to_owned(),
                arg.declared_at,
            );
        }

        for stmt in decl.body.statements.iter() {
            self.visit_statement(stmt);
        }
    }

    fn visit_expression(&self, expr: &Expression) {
        use Expression as E;
        match expr {
            E::Literal(_) => {}
            E::Identifier(_) => {}
            E::ObjectExpression(obj) => self.visit_object_expression(obj),
            E::ArrayExpression(arr) => self.visit_array_expression(arr),
            E::BinaryExpression(bin_expr) => self.visit_binary_expression(bin_expr),
            _ => todo!("{:?}", &expr),
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

    fn visit_binary_expression(&self, bin_expr: &BinaryExpression) {
        self.visit_expression(&bin_expr.left);
        self.visit_expression(&bin_expr.right);
    }

    fn visit_return_statement(&self, stmt: &ReturnStatement) {
        self.visit_expression(&stmt.value);
    }
}
