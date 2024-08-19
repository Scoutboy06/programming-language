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
    And, // &&
    Or,  // ||
    Not, // !
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
