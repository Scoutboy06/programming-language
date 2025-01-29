use std::collections::HashMap;

use parser::{
    expressions::{types::TypeValue, Expression},
    nodes::{program::Program, Node},
};

pub struct Symbol {
    pub name: String,
    pub type_value: TypeValue,
    pub declared_at: Node,
}

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}

pub struct TypeChecker {
    symbols: SymbolTable,
    errors: Vec<CompilationError>,
}

impl TypeChecker {
    pub fn check(&mut self, module: &Program) -> Vec<CompilationError> {
        todo!()
    }

    fn check_expression(&mut self, expression: &Expression) -> TypeValue {
        match expression {
            _ => todo!(),
        }
    }
}

pub struct CompilationError {
    pub message: String,
    pub node: Node,
    pub severity: ErrorSeverity,
}

pub enum ErrorSeverity {
    Critical,
    Warning,
}
