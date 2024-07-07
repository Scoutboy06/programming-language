#[derive(Debug)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: String,
}

impl BinaryOperation {
    pub fn evaluate(&self) -> BinaryOperationResult {
        let left = match self.left.as_ref() {
            Expression::BinaryOperation(op) => op.evaluate(),
            Expression::Float(float) => BinaryOperationResult::Float(*float),
            Expression::Int(int) => BinaryOperationResult::Int(*int),
        };

        let right = match self.right.as_ref() {
            Expression::BinaryOperation(op) => op.evaluate(),
            Expression::Float(float) => BinaryOperationResult::Float(*float),
            Expression::Int(int) => BinaryOperationResult::Int(*int),
        };

        let result = match self.operator.as_str() {
            "+" => left.add(&right),
            "-" => left.sub(&right),
            "*" => left.mult(&right),
            "/" => left.div(&right),
            "**" => todo!(),
            _ => unreachable!(),
        };

        result
    }
}

#[derive(Debug)]
pub enum BinaryOperationResult {
    Int(i64),
    Float(f64),
    // variable expressions, etc.
}

impl BinaryOperationResult {
    fn as_f64(&self) -> f64 {
        match *self {
            BinaryOperationResult::Int(int) => int as f64,
            BinaryOperationResult::Float(float) => float,
        }
    }

    fn add(&self, other: &BinaryOperationResult) -> BinaryOperationResult {
        match (self, other) {
            (BinaryOperationResult::Int(int_left), BinaryOperationResult::Int(int_right)) => {
                BinaryOperationResult::Int(*int_left + *int_right)
            }
            _ => {
                let left = self.as_f64();
                let right = other.as_f64();
                BinaryOperationResult::Float(left + right)
            }
        }
    }

    fn sub(&self, other: &BinaryOperationResult) -> BinaryOperationResult {
        match (self, other) {
            (BinaryOperationResult::Int(int_left), BinaryOperationResult::Int(int_right)) => {
                BinaryOperationResult::Int(*int_left - *int_right)
            }
            _ => {
                let left = self.as_f64();
                let right = other.as_f64();
                BinaryOperationResult::Float(left - right)
            }
        }
    }

    fn mult(&self, other: &BinaryOperationResult) -> BinaryOperationResult {
        match (self, other) {
            (BinaryOperationResult::Int(int_left), BinaryOperationResult::Int(int_right)) => {
                BinaryOperationResult::Int(*int_left * *int_right)
            }
            _ => {
                let left = self.as_f64();
                let right = other.as_f64();
                BinaryOperationResult::Float(left * right)
            }
        }
    }

    fn div(&self, other: &BinaryOperationResult) -> BinaryOperationResult {
        match (self, other) {
            (BinaryOperationResult::Int(int_left), BinaryOperationResult::Int(int_right)) => {
                BinaryOperationResult::Int(*int_left / *int_right)
            }
            _ => {
                let left = self.as_f64();
                let right = other.as_f64();
                BinaryOperationResult::Float(left / right)
            }
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Int(i64),
    Float(f64),
    BinaryOperation(Box<BinaryOperation>),
}

#[derive(Debug)]
pub enum ASTNode {
    Expression(Expression),
}
