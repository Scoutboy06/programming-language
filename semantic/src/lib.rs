mod errors;
mod types;
mod visitors;

use parser::{
    expressions::types::TypeValue,
    nodes::{program::Program, Node},
};
use std::collections::HashMap;
use string_cache::DefaultAtom as Atom;
use visitors::{decl_visitor::DeclVisitor, Visitor};

pub struct Symbol {
    pub id: Atom,
    pub type_value: Option<TypeValue>,
    pub declared_at: Node,
}

pub struct SymbolTable {
    scopes: Vec<HashMap<Atom, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn add(&mut self, id: Atom, type_value: Option<TypeValue>, declared_at: Node) {
        debug_assert!(self.scopes.len() > 0);
        let a = self.scopes.last_mut().unwrap();
        a.insert(
            id.clone(),
            Symbol {
                id: id.clone(),
                type_value,
                declared_at,
            },
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationError {
    pub message: String,
    pub node: Node,
    pub severity: ErrorSeverity,
}

impl CompilationError {
    pub fn new(message: String, node: Node, severity: ErrorSeverity) -> Self {
        Self {
            message,
            node,
            severity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorSeverity {
    Critical,
    Warning,
}

struct CheckerContext {
    errors: Vec<CompilationError>,
    symbols: SymbolTable,
}

impl CheckerContext {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            symbols: SymbolTable::new(),
        }
    }

    pub fn report_error(&mut self, error: CompilationError) {
        self.errors.push(error);
    }

    pub fn add_symbol(&mut self, id: Atom, type_value: Option<TypeValue>, declared_at: Node) {
        self.symbols.add(id, type_value, declared_at);
    }
}

pub fn analyze(ast: &Program) -> Vec<CompilationError> {
    let mut ctx = CheckerContext::new();

    let mut decl_visitor = DeclVisitor::new();
    decl_visitor.visit_program(ast, &mut ctx);

    // let mut body_visitor = BodyVisitor::new();
    // body_visitor.visit_program(ast, &mut ctx);

    dbg!(&ctx.errors);

    ctx.errors
}
