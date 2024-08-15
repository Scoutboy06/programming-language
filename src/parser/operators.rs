#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithmeticOperator {
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
    Increment,          // ++
    Decrement,          // --
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentOperator {
    Equals,                    // =
    PlusEquals,                // +=
    MinusEquals,               // -=
    TimesEquals,               // *=
    DivEquals,                 // /=
    PowerEquals,               // **=
    ModEquals,                 // %=
    BitwiseAndEquals,          // &=
    BitwiseOrEquals,           // |=
    BitwiseXorEquals,          // ^=
    BitwiseNotEquals,          // ~=
    BitwiseLeftShift,          // <<=
    BitwiseSignedRightShift,   // >>=
    BitwiseZeroFillRightShift, // >>>=
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonOperator {
    Equals,              // ==
    NotEquals,           // !=
    StrictEquals,        // ===
    StrictNotEquals,     // !==
    GreaterThan,         // >
    GreaterOrEqualsThan, // >=
    LessThan,            // <
    LessOrEqualsThan,    // <=
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalOperator {
    And, // &&
    Or,  // ||
    Not, // !
}
