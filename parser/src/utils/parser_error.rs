use ariadne::{ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use lexer::Token;

// A lightweight error that is used to construct the actual ParserError
#[derive(Debug, PartialEq)]
pub struct ParserErrorInfo {
    pub kind: ErrorKind,
    #[cfg(debug_assertions)]
    pub id: String,
}

/// The actual error returned by the parser
#[derive(Debug, PartialEq)]
pub struct ParserError {
    pub kind: ErrorKind,
    pub token: Token,
    pub id: String,
}
impl ParserError {
    pub fn print(&self, source: &str) {
        let mut colors = ColorGenerator::new();

        let a = colors.next();

        let msg = match self.kind {
            ErrorKind::Todo => "TODO: This has not yet been implemented",
            ErrorKind::InternalError => "Internal error",
            ErrorKind::InvalidToken => "Invalid token",
        };

        Report::build(
            ReportKind::Error,
            (&self.id, self.token.start..self.token.end),
        )
        .with_code(3) // TODO: Errors should have a unique code
        .with_message(msg)
        .with_label(
            Label::new((&self.id, self.token.start..self.token.end))
                .with_message(msg.fg(a))
                .with_color(a),
        )
        .finish()
        .eprint((&self.id, Source::from(&source)))
        .unwrap();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorKind {
    Todo,
    InternalError,
    InvalidToken,
}
