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
    pub fn is_arithmetic_op(&self) -> bool {
        match *self {
            Operator::Plus(_) | 
            Operator::Minus(_) | 
            Operator::Multiply(_) | 
            Operator::Divide(_) | 
            Operator::Modulus(_) => true,
            _ => false
        }
    }

    pub fn is_boolean_op(&self) -> bool {
        match *self {
            Operator::GreaterThan(_) | 
            Operator::LessThan(_) | 
            Operator::GreaterThanEqualTo(_) | 
            Operator::LessThanEqualTo(_) | 
            Operator::Equal(_) | 
            Operator::Not(_) | 
            Operator::And(_) | 
            Operator::Or(_) => true,
            _ => false
        }
    }

    pub fn is_unary_op(&self) -> bool {
        match *self {
            Operator::Minus(_) | Operator::Not(_) => true,
            _ => false
        }
    }
}