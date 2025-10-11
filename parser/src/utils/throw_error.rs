#[macro_export]
macro_rules! throw_error {
    ($kind:ident) => {{
        use crate::utils::parser_error::{ErrorKind, ParserErrorInfo};
        let err = ParserErrorInfo {
            kind: ErrorKind::$kind,
            id: concat!(file!(), ":", line!()).to_owned(),
        };
        return Err(err);
    }};
}
