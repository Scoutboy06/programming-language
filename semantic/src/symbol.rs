use std::collections::HashMap;

use parser::nodes::Node;
use string_cache::DefaultAtom as Atom;

use crate::types::ResolvedType;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub id: Atom,
    pub resolved_type: Option<ResolvedType>,
    pub declared_at: Node,
}

pub struct SymbolTable {
    pub scopes: Vec<HashMap<Atom, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn add(&mut self, id: Atom, resolved_type: Option<ResolvedType>, declared_at: Node) {
        debug_assert!(self.scopes.len() > 0);
        let a = self.scopes.last_mut().unwrap();
        a.insert(
            id.clone(),
            Symbol {
                id,
                resolved_type,
                declared_at,
            },
        );
    }
}
