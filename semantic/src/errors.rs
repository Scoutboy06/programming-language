use lexer::Operator;
use parser::nodes::Node;
use string_cache::DefaultAtom as Atom;

use crate::types::ResolvedType;

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticError {
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
        expected_type: ResolvedType,
        received_type: ResolvedType,
    },
    InvalidNumberOfArguments {
        received: u8,
        expected: u8,
    },
    UnallowedBinaryOperationTypes {
        left_type: ResolvedType,
        right_type: ResolvedType,
        operator: Operator,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorSeverity {
    Critical,
    Warning,
}

impl std::fmt::Display for ErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownVariable { id } => {
                write!(f, "Unknown variable: {}", id.to_string())
            }
            Self::UseBeforeInit { id } => {
                write!(f, "Variable used before init: {}", id.to_string())
            }
            Self::TypeMismatch {
                expected_type,
                received_type,
            } => write!(
                f,
                "Type mismatch\nExpected: {}\nGot: {}",
                expected_type, received_type,
            ),
            Self::InvalidNumberOfArguments { received, expected } => {
                write!(
                    f,
                    "Invalid number of arguments: Got ({}), Expected ({})",
                    received, expected
                )
            }
            Self::UnallowedBinaryOperationTypes {
                left_type,
                right_type,
                operator,
            } => {
                write!(
                    f,
                    "Disallowed types for operator\nLeft type: {}\nRight type: {}\nOperator: {}",
                    left_type, right_type, operator
                )
            }
        }
    }
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity_msg: &'static str = match self.severity {
            ErrorSeverity::Critical => "Critical error",
            ErrorSeverity::Warning => "Warning",
        };

        write!(
            f,
            "{} on location {}-{}:\n{}",
            severity_msg, self.node.start, self.node.end, self.data
        )
    }
}
