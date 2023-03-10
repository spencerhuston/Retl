use strum_macros::Display;

#[derive(Display, Debug, Eq, PartialEq, Clone, Hash)]
pub enum Operator {
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "*")]
    Multiply,
    #[strum(serialize = "/")]
    Divide,
    #[strum(serialize = "%")]
    Modulus,
    #[strum(serialize = ">")]
    GreaterThan,
    #[strum(serialize = "<")]
    LessThan,
    #[strum(serialize = ">=")]
    GreaterThanEqualTo,
    #[strum(serialize = "<=")]
    LessThanEqualTo,
    #[strum(serialize = "==")]
    Equal,
    #[strum(serialize = "not")]
    Not,
    #[strum(serialize = "and")]
    And,
    #[strum(serialize = "or")]
    Or,
    #[strum(serialize = "++")]
    CollectionConcat
}

impl Operator {
    pub fn is_arithmetic_op(&self) -> bool {
        match *self {
            Operator::Plus | 
            Operator::Minus | 
            Operator::Multiply | 
            Operator::Divide | 
            Operator::Modulus => true,
            _ => false
        }
    }

    pub fn is_boolean_op(&self) -> bool {
        match *self {
            Operator::GreaterThan | 
            Operator::LessThan | 
            Operator::GreaterThanEqualTo | 
            Operator::LessThanEqualTo | 
            Operator::Equal | 
            Operator::Not | 
            Operator::And | 
            Operator::Or => true,
            _ => false
        }
    }

    pub fn is_collection_op(&self) -> bool {
        match *self {
            Operator::CollectionConcat => true,
            _ => false
        }
    }

    pub fn get_precedence(&self) -> i32 {
        match *self {
            Operator::And | Operator::Or | Operator::CollectionConcat => 0,
            Operator::Plus | Operator::Minus => 2,
            Operator::Multiply | Operator::Divide | Operator::Modulus => 3,
            _ => 1
        }
    }

    pub fn is_binary_op(&self, min: i32) -> bool {
        (self.is_boolean_op() || self.is_arithmetic_op() || self.is_collection_op()) &&
            self.get_precedence() >= min
    }
}