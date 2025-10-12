#[macro_export]
macro_rules! throw_error {
    ($kind:ident) => {{
        use crate::utils::parser_error::{ErrorKind, ParserErrorInfo};
        let err = ParserErrorInfo {
            kind: ErrorKind::$kind,
            #[cfg(debug_assertions)]
            id: concat!(file!(), ":", line!()).to_owned(),
        };
        return Err(err);
    }};
}
