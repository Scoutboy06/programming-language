#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    LogicalNot, // !
    BitwiseNot, // ~
    Plus,       // +x (unary plus)
    Minus,      // -x (unary minus)
    Typeof,     // typeof
    Void,       // void
    Delete,     // delete
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateOperator {
    Increment, // ++
    Decrement, // --
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOperator {
    Assign,      // =
    PlusEquals,  // +=
    MinusEquals, // -=
    TimesEquals, // *=
    DivEquals,   // /=
    ModEquals,   // %=
    PowerEquals, // **=

    LeftShiftEquals,          // <<=
    RightShiftEquals,         // <<=
    ZeroFillRightShiftEquals, // >>>=

    BitwiseOrEquals,  // |=
    BitwiseXorEquals, // ^=
    BitwiseAndEquals, // &=
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    Or,      // ||
    And,     // &&,
    Nullish, // ??
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
    /// Arity: how many operands the operator consumes (1 or 2 typically).
    pub fn arity(&self) -> u8 {
        match self {
            Operator::Binary(_) => 2,
            Operator::Assignment(_) => 2,
            Operator::Logical(_) => 2,
            Operator::Unary(_) => 1,
            Operator::Update(_) => 1,
        }
    }

    pub fn is_binary_op(&self) -> bool {
        matches!(self, Operator::Binary(_))
    }
    pub fn as_binary_op(&self) -> Option<BinaryOperator> {
        match self {
            Self::Binary(op) => Some(*op),
            _ => None,
        }
    }

    pub fn is_assignment_op(&self) -> bool {
        matches!(self, Operator::Assignment(_))
    }
    pub fn as_assignment_op(&self) -> Option<AssignmentOperator> {
        match self {
            Self::Assignment(op) => Some(*op),
            _ => None,
        }
    }

    pub fn is_logical_op(&self) -> bool {
        matches!(self, Operator::Logical(_))
    }
    pub fn as_logical_op(&self) -> Option<LogicalOperator> {
        match self {
            Self::Logical(op) => Some(*op),
            _ => None,
        }
    }

    pub fn is_unary_op(&self) -> bool {
        matches!(self, Operator::Unary(_))
    }
    pub fn as_unary_op(&self) -> Option<UnaryOperator> {
        match self {
            Self::Unary(op) => Some(*op),
            _ => None,
        }
    }

    pub fn is_update_op(&self) -> bool {
        matches!(self, Operator::Update(_))
    }
    pub fn as_update_op(&self) -> Option<UpdateOperator> {
        match self {
            Self::Update(op) => Some(*op),
            _ => None,
        }
    }

    pub fn precedence(&self) -> u8 {
        use AssignmentOperator as As;
        use BinaryOperator as Bi;
        use LogicalOperator as Lo;
        use UnaryOperator as Un;
        use UpdateOperator as Up;
        match self {
            Self::Logical(l) => match l {
                Lo::Or | Lo::Nullish => 3,
                Lo::And => 4,
            },
            Self::Unary(u) => match u {
                Un::Plus
                | Un::Minus
                | Un::Typeof
                | Un::Void
                | Un::Delete
                | Un::LogicalNot
                | Un::BitwiseNot => 14,
            },
            Self::Update(u) => match u {
                Up::Increment | Up::Decrement => 15,
            },
            Self::Binary(b) => match b {
                Bi::Power => 13,
                Bi::Mult | Bi::Div | Bi::Mod => 12,
                Bi::Plus | Bi::Minus => 11,
                Bi::LeftShift | Bi::RightShift | Bi::ZeroFillRightShift => 10,
                Bi::In | Bi::Instanceof => 9,
                Bi::LessThan | Bi::LessOrEquals | Bi::GreaterThan | Bi::GreaterOrEquals => 9,
                Bi::Equals | Bi::StrictEquals | Bi::NotEquals | Bi::StrictNotEquals => 8,
                Bi::BitwiseAnd => 7,
                Bi::BitwiseXor => 6,
                Bi::BitwiseOr => 5,
            },
            Self::Assignment(a) => match a {
                As::Assign
                | As::PlusEquals
                | As::MinusEquals
                | As::TimesEquals
                | As::PowerEquals
                | As::DivEquals
                | As::ModEquals
                | As::LeftShiftEquals
                | As::RightShiftEquals
                | As::ZeroFillRightShiftEquals
                | As::BitwiseAndEquals
                | As::BitwiseOrEquals
                | As::BitwiseXorEquals => 2,
            },
        }
    }
}

// Optional Display impl: useful for debug / pretty printing
impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Binary(b) => write!(f, "{:?}", b),
            Operator::Unary(u) => write!(f, "{:?}", u),
            Operator::Update(u) => write!(f, "{:?}", u),
            Operator::Assignment(a) => write!(f, "{:?}", a),
            Operator::Logical(l) => write!(f, "{:?}", l),
        }
    }
}

impl Into<Operator> for UnaryOperator {
    fn into(self) -> Operator {
        Operator::Unary(self)
    }
}
impl Into<Operator> for UpdateOperator {
    fn into(self) -> Operator {
        Operator::Update(self)
    }
}
impl Into<Operator> for BinaryOperator {
    fn into(self) -> Operator {
        Operator::Binary(self)
    }
}
impl Into<Operator> for AssignmentOperator {
    fn into(self) -> Operator {
        Operator::Assignment(self)
    }
}
impl Into<Operator> for LogicalOperator {
    fn into(self) -> Operator {
        Operator::Logical(self)
    }
}
