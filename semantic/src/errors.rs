use parser::nodes::Node;

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
