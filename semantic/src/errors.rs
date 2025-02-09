use parser::nodes::Node;
use string_cache::DefaultAtom as Atom;

use crate::types::ExprType;

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationError {
    pub data: ErrorData,
    pub node: Node,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorData {
    UnknownVariable {
        id: Atom,
    },
    UseBeforeInit {
        id: Atom,
    },
    TypeMismatch {
        expected_type: ExprType,
        received_type: ExprType,
    },
    InvalidNumberOfArguments {
        received: u8,
        expected: u8,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorSeverity {
    Critical,
    Warning,
}
