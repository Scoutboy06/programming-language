pub mod errors;
pub mod symbol;
pub mod types;
pub mod visitors;

use errors::{ErrorData, ErrorSeverity, SemanticError};
use parser::nodes::{program::Program, Node};
use string_cache::DefaultAtom as Atom;
use symbol::{Symbol, SymbolTable};
use types::ResolvedType;
use visitors::{body_visitor::BodyVisitor, decl_visitor::DeclVisitor};

pub struct CheckerContext {
    errors: Vec<SemanticError>,
    symbols: SymbolTable,
}

impl CheckerContext {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            symbols: SymbolTable::new(),
        }
    }

    pub fn report_error(&mut self, data: ErrorData, node: Node, severity: ErrorSeverity) {
        self.errors.push(SemanticError {
            data,
            node,
            severity,
        });
    }

    pub fn get_symbol(&self, id: Atom) -> Option<&Symbol> {
        for scope in self.symbols.scopes.iter().rev() {
            if let Some(symbol) = scope.get(&id) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn get_symbol_mut(&mut self, id: Atom) -> Option<&mut Symbol> {
        for scope in self.symbols.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.get_mut(&id) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn add_symbol(&mut self, id: Atom, resolved_type: Option<ResolvedType>, declared_at: Node) {
        self.symbols.add(id, resolved_type, declared_at);
    }
}

pub fn analyze(ast: &Program) -> Vec<SemanticError> {
    let mut ctx = CheckerContext::new();

    DeclVisitor::visit_program(ast, &mut ctx);
    BodyVisitor::visit_program(ast, &mut ctx);

    ctx.errors
}
