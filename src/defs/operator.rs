use strum_macros::Display;
use crate::{Type, Value};

#[derive(Display, Debug, Eq, PartialEq, Clone)]
pub enum Operator {
    #[strum(serialize = "+")] // int, char/string
    Plus,
    #[strum(serialize = "-")] // int
    Minus,
    #[strum(serialize = "*")] // int
    Multiply,
    #[strum(serialize = "/")] // int
    Divide,
    #[strum(serialize = "%")] // int
    Modulus,
    #[strum(serialize = ">")] // int
    GreaterThan,
    #[strum(serialize = "<")] // int
    LessThan,
    #[strum(serialize = ">=")] // int
    GreaterThanEqualTo,
    #[strum(serialize = "<=")] // int
    LessThanEqualTo,
    #[strum(serialize = "==")] // int, bool, char, string, list, tuple, dict
    Equal,
    #[strum(serialize = "not")] // bool
    Not,
    #[strum(serialize = "and")] // bool
    And,
    #[strum(serialize = "or")] // bool
    Or,
    #[strum(serialize = "++")] // list, tuple, dict
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

    pub fn get_type(&self) -> Type {
        match *self {
            Operator::Plus => Type::UnknownType,
            Operator::Minus |
            Operator::Multiply |
            Operator::Divide |
            Operator::Modulus => Type::IntType,
            Operator::GreaterThan |
            Operator::LessThan |
            Operator::GreaterThanEqualTo |
            Operator::LessThanEqualTo => Type::IntType,
            Operator::Equal |
            Operator::Not |
            Operator::And |
            Operator::Or =>  Type::BoolType,
            Operator::CollectionConcat => Type::UnknownType
        }
    }

    pub fn types_allowed(&self, t1: &Type, t2: &Type) -> bool {
        match *self {
            Operator::Plus => match (t1, t2) {
                (Type::IntType, Type::IntType) => true,
                (Type::CharType, Type::CharType) => true,
                (Type::StringType, Type::StringType) => true,
                (Type::CharType, Type::StringType) => true,
                (Type::StringType, Type::CharType) => true,
                _ => false
            },
            Operator::Minus |
            Operator::Multiply |
            Operator::Divide |
            Operator::Modulus => match (t1, t2) {
                (Type::IntType, Type::IntType) => true,
                _ => false
            },
            Operator::GreaterThan |
            Operator::LessThan |
            Operator::GreaterThanEqualTo |
            Operator::LessThanEqualTo => match (t1, t2) {
                (Type::IntType, Type::IntType) => true,
                _ => false
            },
            Operator::Equal => match (t1, t2) {
                (Type::IntType, Type::IntType) => true,
                (Type::BoolType, Type::BoolType) => true,
                (Type::CharType, Type::CharType) => true,
                (Type::StringType, Type::StringType) => true,
                (Type::ListType{list_type: l1},
                    Type::ListType{list_type: l2}) => {
                    self.types_allowed(l1, l2)
                },
                (Type::TupleType{tuple_types: tts1},
                    Type::TupleType{tuple_types: tts2}) => {
                    tts1.iter()
                        .zip(tts2)
                        .all(|(tt1, tt2)| { self.types_allowed(tt1 ,tt2) })
                },
                (Type::DictType{key_type: k1, value_type: v1},
                    Type::DictType{key_type: k2, value_type: v2}) => {
                    self.types_allowed(k1, k2) && self.types_allowed(v1, v2)
                },
                _ => false
            },
            Operator::Not |
            Operator::And |
            Operator::Or => match (t1, t2) {
                (Type::BoolType, Type::BoolType) => true,
                _ => false
            },
            Operator::CollectionConcat => match (t1, t2) {
                (Type::ListType{..}, Type::ListType{..}) => true,
                (Type::TupleType{..}, Type::TupleType{..}) => true,
                _ => false
            }
        }
    }

    // pub fn interpret(left: &Value, right: &Value) -> Value {
    //     match *self {
    //         Operator::Plus => ,
    //         Operator::Minus => ,
    //         Operator::Multiply => ,
    //         Operator::Divide => ,
    //         Operator::Modulus => ,
    //         Operator::GreaterThan => ,
    //         Operator::LessThan => ,
    //         Operator::GreaterThanEqualTo => ,
    //         Operator::LessThanEqualTo => ,
    //         Operator::Equal => ,
    //         Operator::Not => ,
    //         Operator::And => ,
    //         Operator::Or => ,
    //         Operator::CollectionConcat => ,
    //     }
    // }
}