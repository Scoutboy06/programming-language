mod errors;
mod symbol;
mod visitors;

use parser::nodes::{program::Program, Node};
use string_cache::DefaultAtom as Atom;
use symbol::{Symbol, SymbolTable};
use visitors::{decl_visitor::DeclVisitor, Visitor};

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
    pub errors: Vec<CompilationError>,
    pub symbols: SymbolTable,
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

    pub fn get_symbol(&self, id: Atom) -> Option<&Symbol> {
        for scope in self.symbols.scopes.iter().rev() {
            if let Some(symbol) = scope.get(&id) {
                return Some(symbol);
            }
        }
        None
    }
}

pub fn analyze(ast: &Program) -> Vec<CompilationError> {
    let mut ctx = CheckerContext::new();

    let decl_visitor = DeclVisitor::new();
    decl_visitor.visit_program(ast, &mut ctx);

    // let mut body_visitor = BodyVisitor::new();
    // body_visitor.visit_program(ast, &mut ctx);

    ctx.errors
}
