use std::collections::HashMap;

use parser::{expressions::types::TypeValue, nodes::Node};
use string_cache::DefaultAtom as Atom;

use crate::types::ExprType;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub id: Atom,
    pub unfolded_type: Option<ExprType>,
    pub display_type: Option<TypeValue>,
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

    pub fn add(
        &mut self,
        id: Atom,
        unfolded_type: Option<ExprType>,
        display_type: Option<TypeValue>,
        declared_at: Node,
    ) {
        debug_assert!(self.scopes.len() > 0);
        let a = self.scopes.last_mut().unwrap();
        a.insert(
            id.clone(),
            Symbol {
                id,
                unfolded_type,
                display_type,
                declared_at,
            },
        );
    }
}
