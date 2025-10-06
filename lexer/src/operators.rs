#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,               // +
    Minus,              // -
    Mult,               // *
    Div,                // /
    Power,              // **
    Mod,                // %
    BitwiseAnd,         // &
    BitwiseOr,          // |
    BitwiseXor,         // ^
    BitwiseNot,         // ~
    BitwiseLeftShift,   // <<
    BitwiseRightShift,  // >>
    ZeroFillRightShift, // >>>

    // Update operators
    Increment, // ++
    Decrement, // --

    // Comparison operators
    Equals,              // ==
    NotEquals,           // !=
    StrictEquals,        // ===
    StrictNotEquals,     // !==
    GreaterThan,         // >
    GreaterOrEqualsThan, // >=
    LessThan,            // <
    LessOrEqualsThan,    // <=

    // Logical operators
    LogicalAnd, // &&
    LogicalOr,  // ||
    LogicalNot, // !

    // Assignment operators
    Assignment,       // =
    PlusEquals,       // +=
    MinusEquals,      // -=
    TimesEquals,      // *=
    DivEquals,        // /=
    PowerEquals,      // **=
    ModEquals,        // %=
    BitwiseAndEquals, // &=
    BitwiseOrEquals,  // |=
    BitwiseXorEquals, // ^=
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Mult => "*",
            Self::Div => "/",
            Self::Power => "/",
            Self::Mod => "%",
            Self::BitwiseAnd => "&",
            Self::BitwiseOr => "|",
            Self::BitwiseXor => "^",
            Self::BitwiseNot => "~",
            Self::BitwiseLeftShift => "<<",
            Self::BitwiseRightShift => ">>",
            Self::ZeroFillRightShift => ">>>",

            // Update operators
            Self::Increment => "++",
            Self::Decrement => "--",

            // Comparison operators
            Self::Equals => "==",
            Self::NotEquals => "!=",
            Self::StrictEquals => "===",
            Self::StrictNotEquals => "!==",
            Self::GreaterThan => ">",
            Self::GreaterOrEqualsThan => ">=",
            Self::LessThan => "<",
            Self::LessOrEqualsThan => "<=",

            // Logical operators
            Self::LogicalAnd => "&&",
            Self::LogicalOr => "||",
            Self::LogicalNot => "!",

            // Assignment operators
            Self::Assignment => "=",
            Self::PlusEquals => "+=",
            Self::MinusEquals => "-=",
            Self::TimesEquals => "*=",
            Self::DivEquals => "/=",
            Self::PowerEquals => "**=",
            Self::ModEquals => "=",
            Self::BitwiseAndEquals => "&=",
            Self::BitwiseOrEquals => "|=",
            Self::BitwiseXorEquals => "^=",
        };

        write!(f, "{}", s)
    }
}
