use lexer::TokenKind;

// es5
// enum UnaryOperator {
//     "-" | "+" | "!" | "~" | "typeof" | "void" | "delete"
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    LogicalNot, // !
    BitwiseNot, // ~
    Plus,       // +
    Minus,      // -
    Typeof,     // typeof
    Void,       // void
    Delete,     // delete
}

impl UnaryOperator {
    /// Returns the precedence of the unary operator.
    /// In JavaScript, unary operators have precedence 14.
    pub fn precedence(&self) -> u8 {
        14
    }
}

// es5
// enum UpdateOperator {
//     "++" | "--"
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateOperator {
    Increment, // ++
    Decrement, // --
}

impl UpdateOperator {
    /// Returns the precedence of the update operator.
    /// In JavaScript, update operators (++/--) have precedence 15 (highest).
    pub fn precedence(&self) -> u8 {
        15
    }
}

// es5
// enum BinaryOperator {
//     "==" | "!=" | "===" | "!=="
//          | "<" | "<=" | ">" | ">="
//          | "<<" | ">>" | ">>>"
//          | "+" | "-" | "*" | "/" | "%"
//          | "|" | "^" | "&" | "in"
//          | "instanceof"
// }
//
// es2016
// extend enum BinaryOperator {
//     "**"
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Equals,          // ==
    NotEquals,       // !=
    StrictEquals,    // ===
    StrictNotEquals, // !==

    LessThan,        // <
    LessOrEquals,    // <=
    GreaterThan,     // >
    GreaterOrEquals, // >=

    LeftShift,          // <<
    RightShift,         // >>
    ZeroFillRightShift, // >>>

    Plus,  // +
    Minus, // -
    Mult,  // *
    Div,   // /
    Power, // **
    Mod,   // %

    BitwiseOr,  // |
    BitwiseXor, // ^
    BitwiseAnd, // &

    In,         // in
    Instanceof, // instanceof
}

impl BinaryOperator {
    /// Returns the precedence of the binary operator.
    /// Precedence values follow JavaScript operator precedence rules.
    pub fn precedence(&self) -> u8 {
        match self {
            // Exponentiation
            BinaryOperator::Power => 12,
            // Multiplicative
            BinaryOperator::Mult | BinaryOperator::Div | BinaryOperator::Mod => 11,
            // Additive
            BinaryOperator::Plus | BinaryOperator::Minus => 10,
            // Shift
            BinaryOperator::LeftShift
            | BinaryOperator::RightShift
            | BinaryOperator::ZeroFillRightShift => 9,
            // Relational
            BinaryOperator::LessThan
            | BinaryOperator::LessOrEquals
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterOrEquals
            | BinaryOperator::In
            | BinaryOperator::Instanceof => 8,
            // Equality
            BinaryOperator::Equals
            | BinaryOperator::NotEquals
            | BinaryOperator::StrictEquals
            | BinaryOperator::StrictNotEquals => 7,
            // Bitwise AND
            BinaryOperator::BitwiseAnd => 6,
            // Bitwise XOR
            BinaryOperator::BitwiseXor => 5,
            // Bitwise OR
            BinaryOperator::BitwiseOr => 4,
        }
    }
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            BinaryOperator::Equals => "==",
            BinaryOperator::NotEquals => "!=",
            BinaryOperator::StrictEquals => "===",
            BinaryOperator::StrictNotEquals => "!==",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessOrEquals => "<=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterOrEquals => ">=",
            BinaryOperator::LeftShift => "<<",
            BinaryOperator::RightShift => ">>",
            BinaryOperator::ZeroFillRightShift => ">>>",
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Mult => "*",
            BinaryOperator::Div => "/",
            BinaryOperator::Power => "**",
            BinaryOperator::Mod => "%",
            BinaryOperator::BitwiseOr => "|",
            BinaryOperator::BitwiseXor => "^",
            BinaryOperator::BitwiseAnd => "&",
            BinaryOperator::In => "in",
            BinaryOperator::Instanceof => "instanceof",
        };
        write!(f, "{}", op_str)
    }
}

// es5
// enum AssignmentOperator {
//     "=" | "+=" | "-=" | "*=" | "/=" | "%="
//         | "<<=" | ">>=" | ">>>="
//         | "|=" | "^=" | "&="
// }
//
// es2016
// extend enum AssignmentOperator {
//     "**="
// }
//
// es2021
// extend enum AssignmentOperator {
//     "||=" | "&&=" | "??="
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOperator {
    Assign,  // =
    PlusEq,  // +=
    MinusEq, // -=
    TimesEq, // *=
    DivEq,   // /=
    ModEq,   // %=
    PowerEq, // **=

    LeftShiftEq,          // <<=
    RightShiftEq,         // <<=
    ZeroFillRightShiftEq, // >>>=

    BitwiseOrEq,  // |=
    BitwiseXorEq, // ^=
    BitwiseAndEq, // &=

    OrEq,      // ||=
    AndEq,     // &&=
    NullishEq, // ??=
}

impl AssignmentOperator {
    /// Returns the precedence of the assignment operator.
    /// Assignment operators have precedence 2 in JavaScript.
    pub fn precedence(&self) -> u8 {
        2
    }
}

// es5
// enum LogicalOperator {
//     "||" | "&&"
// }
//
// es2020
// extend enum LogicalOperator {
//     "??"
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    Or,      // ||
    And,     // &&,
    Nullish, // ??
}

impl LogicalOperator {
    /// Returns the precedence of the logical operator.
    /// || = 3, && = 4, ?? = 3 (same as ||)
    pub fn precedence(&self) -> u8 {
        match self {
            LogicalOperator::Or => 3,
            LogicalOperator::Nullish => 3,
            LogicalOperator::And => 4,
        }
    }
}

pub trait TokenKindExt {
    fn as_unary_op(&self) -> Option<UnaryOperator>;
    fn is_unary_op(&self) -> bool {
        self.as_unary_op().is_some()
    }
    fn as_update_op(&self) -> Option<UpdateOperator>;
    fn is_update_op(&self) -> bool {
        self.as_update_op().is_some()
    }
    fn as_binary_op(&self) -> Option<BinaryOperator>;
    fn is_binary_op(&self) -> bool {
        self.as_binary_op().is_some()
    }
    fn as_assignment_op(&self) -> Option<AssignmentOperator>;
    fn is_assignment_op(&self) -> bool {
        self.as_assignment_op().is_some()
    }
    fn as_logical_op(&self) -> Option<LogicalOperator>;
    fn is_logical_op(&self) -> bool {
        self.as_logical_op().is_some()
    }
}

impl TokenKindExt for TokenKind {
    fn as_unary_op(&self) -> Option<UnaryOperator> {
        match self {
            TokenKind::Bang => Some(UnaryOperator::LogicalNot),
            TokenKind::Tilde => Some(UnaryOperator::BitwiseNot),
            TokenKind::Plus => Some(UnaryOperator::Plus),
            TokenKind::Minus => Some(UnaryOperator::Minus),
            TokenKind::Typeof => Some(UnaryOperator::Typeof),
            TokenKind::Void => Some(UnaryOperator::Void),
            TokenKind::Delete => Some(UnaryOperator::Delete),
            _ => None,
        }
    }

    fn as_update_op(&self) -> Option<UpdateOperator> {
        match self {
            TokenKind::PlusPlus => Some(UpdateOperator::Increment),
            TokenKind::MinusMinus => Some(UpdateOperator::Decrement),
            _ => None,
        }
    }

    fn as_binary_op(&self) -> Option<BinaryOperator> {
        match self {
            TokenKind::EqEq => Some(BinaryOperator::Equals),
            TokenKind::BangEq => Some(BinaryOperator::NotEquals),
            TokenKind::EqEqEq => Some(BinaryOperator::StrictEquals),
            TokenKind::BangEqEq => Some(BinaryOperator::StrictNotEquals),
            TokenKind::Lt => Some(BinaryOperator::LessThan),
            TokenKind::LtEq => Some(BinaryOperator::LessOrEquals),
            TokenKind::Gt => Some(BinaryOperator::GreaterThan),
            TokenKind::GtEq => Some(BinaryOperator::GreaterOrEquals),
            TokenKind::LtLt => Some(BinaryOperator::LeftShift),
            TokenKind::GtGt => Some(BinaryOperator::RightShift),
            TokenKind::GtGtGt => Some(BinaryOperator::ZeroFillRightShift),
            TokenKind::Plus => Some(BinaryOperator::Plus),
            TokenKind::Minus => Some(BinaryOperator::Minus),
            TokenKind::Star => Some(BinaryOperator::Mult),
            TokenKind::Slash => Some(BinaryOperator::Div),
            TokenKind::Percent => Some(BinaryOperator::Mod),
            TokenKind::StarStar => Some(BinaryOperator::Power),
            TokenKind::Pipe => Some(BinaryOperator::BitwiseOr),
            TokenKind::Caret => Some(BinaryOperator::BitwiseXor),
            TokenKind::Amp => Some(BinaryOperator::BitwiseAnd),
            TokenKind::In => Some(BinaryOperator::In),
            TokenKind::Instanceof => Some(BinaryOperator::Instanceof),
            _ => None,
        }
    }

    fn as_assignment_op(&self) -> Option<AssignmentOperator> {
        match self {
            TokenKind::Eq => Some(AssignmentOperator::Assign),
            TokenKind::PlusEq => Some(AssignmentOperator::PlusEq),
            TokenKind::MinusEq => Some(AssignmentOperator::MinusEq),
            TokenKind::StarEq => Some(AssignmentOperator::TimesEq),
            TokenKind::SlashEq => Some(AssignmentOperator::DivEq),
            TokenKind::PercentEq => Some(AssignmentOperator::ModEq),
            TokenKind::StarStarEq => Some(AssignmentOperator::PowerEq),
            TokenKind::LtLtEq => Some(AssignmentOperator::LeftShiftEq),
            TokenKind::GtGtEq => Some(AssignmentOperator::RightShiftEq),
            TokenKind::GtGtGtEq => Some(AssignmentOperator::ZeroFillRightShiftEq),
            TokenKind::PipeEq => Some(AssignmentOperator::BitwiseOrEq),
            TokenKind::CaretEq => Some(AssignmentOperator::BitwiseXorEq),
            TokenKind::AmpEq => Some(AssignmentOperator::BitwiseAndEq),
            TokenKind::PipePipeEq => Some(AssignmentOperator::OrEq),
            TokenKind::AmpAmpEq => Some(AssignmentOperator::AndEq),
            TokenKind::QuestionQuestionEq => Some(AssignmentOperator::NullishEq),
            _ => None,
        }
    }

    fn as_logical_op(&self) -> Option<LogicalOperator> {
        match self {
            TokenKind::PipePipe => Some(LogicalOperator::Or),
            TokenKind::AmpAmp => Some(LogicalOperator::And),
            TokenKind::QuestionQuestion => Some(LogicalOperator::Nullish),
            _ => None,
        }
    }
}

/// Top-level Operator wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Unary(UnaryOperator),
    Update(UpdateOperator),
    Binary(BinaryOperator),
    Assignment(AssignmentOperator),
    Logical(LogicalOperator),
}

impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Unary(op) => op.precedence(),
            Operator::Update(op) => op.precedence(),
            Operator::Binary(op) => op.precedence(),
            Operator::Assignment(op) => op.precedence(),
            Operator::Logical(op) => op.precedence(),
        }
    }
}
