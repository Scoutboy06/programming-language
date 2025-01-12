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
