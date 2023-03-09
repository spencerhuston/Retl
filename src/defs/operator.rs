use strum_macros::Display;

#[derive(Display, Debug)]
pub enum Operator {
    #[strum(serialize = "+")]
    Plus(String),
    #[strum(serialize = "-")]
    Minus(String),
    #[strum(serialize = "*")]
    Multiply(String),
    #[strum(serialize = "/")]
    Divide(String),
    #[strum(serialize = "%")]
    Modulus(String),
    #[strum(serialize = ">")]
    GreaterThan(String),
    #[strum(serialize = "<")]
    LessThan(String),
    #[strum(serialize = ">=")]
    GreaterThanEqualTo(String),
    #[strum(serialize = "<=")]
    LessThanEqualTo(String),
    #[strum(serialize = "==")]
    Equal(String),
    #[strum(serialize = "not")]
    Not(String),
    #[strum(serialize = "and")]
    And(String),
    #[strum(serialize = "or")]
    Or(String),
    #[strum(serialize = "++")]
    CollectionConcat(String)
}

impl Operator {
    pub fn isArithmeticOp(&self) -> bool {
        match self {
            Plus | Minus | Multiply | Divide | Modulus => true,
            _ => false
        }
    }

    pub fn isBooleanOp(&self) -> bool {
        match self {
            GreaterThan | LessThan | GreaterThanEqualTo | LessThanEqualTo | Equal | Not | And | Or => true,
            _ => false
        }
    }

    pub fn isUnaryOp(&self) -> bool {
        match self {
            Minus | Not => true,
            _ => false
        }
    }
}