use std::collections::HashMap;

use parser::{expressions::types::TypeValue, nodes::Node};
use string_cache::DefaultAtom as Atom;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub id: Atom,
    pub type_value: Option<TypeValue>,
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

    pub fn add(&mut self, id: Atom, type_value: Option<TypeValue>, declared_at: Node) {
        debug_assert!(self.scopes.len() > 0);
        let a = self.scopes.last_mut().unwrap();
        a.insert(
            id.clone(),
            Symbol {
                id,
                type_value,
                declared_at,
            },
        );
    }
}
