use crate::parser::ErrorKind;
use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};
use lexer::Token;

#[derive(Debug, PartialEq)]
pub struct ParserError<'a> {
    pub kind: ErrorKind,
    pub token: Token,
    pub source: &'a str,
}

impl<'a> ParserError<'a> {
    pub fn print(&self) {
        let mut colors = ColorGenerator::new();

        let a = colors.next();
        let b = colors.next();
        let out = Color::Fixed(81);

        let msg = match self.kind {
            ErrorKind::InternalError => "Internal error",
            ErrorKind::InvalidToken => "Invalid token",
        };

        Report::build(ReportKind::Error, ("source", 12..12))
            .with_code(3) // TODO: Errors should have a unique code
            .with_message(msg)
            .with_label(
                Label::new(("source", self.token.start..self.token.end))
                    .with_message(msg.fg(a))
                    .with_color(a),
            )
            .finish()
            .print(("source", Source::from(self.source)))
            .unwrap();
    }
}

