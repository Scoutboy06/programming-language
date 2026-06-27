use crate::{
    symbol::Symbol,
    types::{FunctionType, ResolvedType},
    CheckerContext,
};
use parser::ast_types::{
    declarations::{
        declaration::Declaration, function_declaration::FunctionDeclaration,
        variable_declaration::VariableDeclaration,
    },
    expressions::{ArrayExpression, BinaryExpression, Expression, ObjectExpression},
    patterns::pattern::Pattern,
    programs::{program::ProgramBodyItem, Program},
    statements::{FunctionBodyBody, ReturnStatement, Statement},
};

pub struct DeclVisitor<'a> {
    ctx: &'a mut CheckerContext,
}

impl<'a> DeclVisitor<'a> {
    pub fn visit_program(ast: &Program, ctx: &'a mut CheckerContext) {
        let mut visitor = Self { ctx };
        ast.body.iter().for_each(|body_item| match body_item {
            ProgramBodyItem::Statement(stmt) => visitor.visit_statement(stmt),
            ProgramBodyItem::ImportOrExportDeclaration(_) => todo!(),
        });
    }

    fn visit_statement(&mut self, stmt: &Statement) {
        use Statement as S;

        match stmt {
            S::BlockStatement(stmt) => stmt.body.iter().for_each(|s| self.visit_statement(s)),
            S::BreakStatement(stmt) => todo!("{:?}", &stmt),
            S::ContinueStatement(stmt) => todo!("{:?}", &stmt),
            S::DebuggerStatement(stmt) => todo!("{:?}", &stmt),
            S::Declaration(decl) => self.visit_declaration(decl),
            S::Directive(dir) => todo!("{:?}", &dir),
            S::DoWhileStatement(stmt) => todo!("{:?}", &stmt),
            S::EmptyStatement(_) => {}
            S::ExpressionStatement(expr) => todo!("{:?}", &expr),
            S::ForInStatement(stmt) => todo!("{:?}", &stmt),
            S::ForOfStatement(stmt) => todo!("{:?}", &stmt),
            S::ForStatement(stmt) => todo!("{:?}", &stmt),
            S::IfStatement(stmt) => todo!("{:?}", &stmt),
            S::LabeledStatement(stmt) => todo!("{:?}", &stmt),
            S::ReturnStatement(stmt) => self.visit_return_statement(stmt),
            S::SwitchStatement(stmt) => todo!("{:?}", &stmt),
            S::ThrowStatement(stmt) => todo!("{:?}", &stmt),
            S::TryStatement(stmt) => todo!("{:?}", &stmt),
            S::WhileStatement(stmt) => todo!("{:?}", &stmt),
            S::WithStatement(stmt) => todo!("{:?}", &stmt),
        }
    }

    fn visit_declaration(&mut self, decl: &Declaration) {
        match decl {
            Declaration::FunctionDeclaration(func_decl) => {
                self.visit_function_declaration(func_decl)
            }
            Declaration::VariableDeclaration(var_decl) => self.visit_variable_declaration(var_decl),
        }
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) {
        for d in decl.declarations.iter() {
            let ast_type = d.type_annotation.as_ref().map(|ann| &ann.type_value);
            let resolved_type = ast_type.map(|t| ResolvedType::from_ast_type(t, &mut self.ctx));

            d.init.as_ref().inspect(|init| {
                self.visit_expression(init);
            });

            let id = match &d.id {
                Pattern::Identifier(id) => id,
                _ => todo!(),
            }
            .to_owned();

            self.ctx.add_symbol(id.name, resolved_type, d.node.clone());
        }
    }

    fn visit_function_declaration(&mut self, decl: &FunctionDeclaration) {
        let args: Vec<Symbol> = todo!();
        //  decl
        //     .params
        //     .iter()
        //     .map(|param| Symbol {
        //         id: param.identifier.name.clone(),
        //         resolved_type: param
        //             .type_annotation
        //             .as_ref()
        //             .map(|ann| ResolvedType::from_ast_type(&ann.type_value, &mut self.ctx)),
        //         declared_at: param.node.clone(),
        //     })
        //     .collect();

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

        for body in decl.body.body.iter() {
            match body {
                FunctionBodyBody::Statement(stmt) => {
                    self.visit_statement(stmt);
                }
                _ => todo!(),
            }
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
        todo!()
        // for item in obj.items.iter() {
        //     match item {
        //         ObjectItem::KV(kv) => match &kv.key {
        //             Key::Identifier(_) | Key::StringLiteral(_) => {}
        //             Key::ComputedProperty(prop) => self.visit_expression(&prop.expression),
        //         },
        //         ObjectItem::Identifier(_) => {}
        //         ObjectItem::Method(_method) => todo!(),
        //     };
        // }
    }

    fn visit_array_expression(&self, arr: &ArrayExpression) {
        arr.elements.iter().for_each(|elem| {
            elem.as_ref().inspect(|e| self.visit_expression(e));
        });
    }

    fn visit_binary_expression(&self, bin_expr: &BinaryExpression) {
        self.visit_expression(&bin_expr.left);
        self.visit_expression(&bin_expr.right);
    }

    fn visit_return_statement(&self, stmt: &ReturnStatement) {
        stmt.argument
            .as_ref()
            .inspect(|expr| self.visit_expression(expr));
    }
}
