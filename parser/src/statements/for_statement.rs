use parser_derive::Stmt;

use super::Statement;
use crate::{expressions::Expression, impl_from, nodes::Node, statements::VariableDeclaration};

#[derive(Debug, Clone, PartialEq, Stmt)]
pub enum ForStatement {
    ForClassic(ForClassic),
    ForIn(ForIn),
    ForOf(ForOf),
}

impl ForStatement {
    pub fn node(&self) -> &Node {
        match self {
            Self::ForClassic(s) => &s.node,
            Self::ForIn(s) => &s.node,
            Self::ForOf(s) => &s.node,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForClassic {
    pub node: Node,
    pub init: Option<Statement>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Statement,
}
impl_from!(ForStatement, ForClassic);

impl Into<Statement> for ForClassic {
    fn into(self) -> Statement {
        Statement::ForStatement(Box::new(ForStatement::ForClassic(self)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForOf {
    pub node: Node,
    pub left: ForLeft,
    pub right: Expression,
    pub body: Statement,
}
impl_from!(ForStatement, ForIn);

impl Into<Statement> for ForOf {
    fn into(self) -> Statement {
        Statement::ForStatement(Box::new(ForStatement::ForOf(self)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForIn {
    pub node: Node,
    pub left: ForLeft,
    pub right: Expression,
    pub body: Statement,
}
impl_from!(ForStatement, ForOf);

impl Into<Statement> for ForIn {
    fn into(self) -> Statement {
        Statement::ForStatement(Box::new(ForStatement::ForIn(self)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForLeft {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}
impl_from!(ForLeft, VariableDeclaration);
impl_from!(ForLeft, Expression);

#[derive(Debug, Clone, PartialEq)]
pub enum ForHead {
    Empty,
    VarDecl(VariableDeclaration),
    Expr(Expression),
}
