use strum_macros::Display;
use crate::{Type, Value};
use crate::defs::retl_type::concat_tuple_types;
use crate::interpreter::interpreter::make_error_value;
use crate::interpreter::value::Val;

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

    pub fn interpret(&self, left: &Value, right: &Value) -> Value {
        match *self {
            Operator::Plus => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::IntValue{value: v1 + v2}, val_type: Type::IntType}
                },
                (Val::CharValue{value: v1}, Val::CharValue{value: v2}) => {
                    Value{value: Val::CharValue{value: v1 + &*v2 }, val_type: Type::CharType}
                },
                (Val::StringValue{value: v1}, Val::StringValue{value: v2}) => {
                    Value{value: Val::StringValue{value: v1 + &*v2 }, val_type: Type::StringType}
                },
                (Val::CharValue{value: v1}, Val::StringValue{value: v2}) => {
                    Value{value: Val::StringValue{value: v1 + &*v2 }, val_type: Type::StringType}
                },
                (Val::StringValue{value: v1}, Val::CharValue{value: v2}) => {
                    Value{value: Val::StringValue{value: v1 + &*v2 }, val_type: Type::StringType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    make_error_value()
                }
            },
            Operator::Minus => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::IntValue{value: v1 - v2}, val_type: Type::IntType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    make_error_value()
                }
            },
            Operator::Multiply => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::IntValue{value: v1 * v2}, val_type: Type::IntType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    make_error_value()
                }
            },
            Operator::Divide => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::IntValue{value: v1 / v2}, val_type: Type::IntType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    make_error_value()
                }
            },
            Operator::Modulus => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::IntValue{value: v1 % v2}, val_type: Type::IntType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    make_error_value()
                }
            },
            Operator::GreaterThan => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 > v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    make_error_value()
                }
            },
            Operator::LessThan => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 < v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    make_error_value()
                }
            },
            Operator::GreaterThanEqualTo => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 >= v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    make_error_value()
                }
            },
            Operator::LessThanEqualTo => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 <= v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    make_error_value()
                }
            },
            // Operator::Equal => match (left.value.clone(), right.value.clone()) {
            //
            // },
            Operator::And => match (left.value.clone(), right.value.clone()) {
                (Val::BoolValue{value: v1}, Val::BoolValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 && v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    make_error_value()
                }
            },
            Operator::Or => match (left.value.clone(), right.value.clone()) {
                (Val::BoolValue{value: v1}, Val::BoolValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 || v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    make_error_value()
                }
            },
            Operator::CollectionConcat => match (left.value.clone(), right.value.clone()) {
                (Val::ListValue{values: v1}, Val::ListValue{values: v2}) => {
                    v1.clone().append(&mut v2.clone());
                    Value{
                        value: Val::ListValue{values: v1},
                        val_type: left.val_type.clone()
                    }
                },
                (Val::TupleValue{values: tt1}, Val::TupleValue{values: tt2}) => {
                    tt1.clone().append(&mut tt2.clone());
                    Value{
                        value: Val::TupleValue{values: tt1},
                        val_type: Type::TupleType{tuple_types: concat_tuple_types(&left, &right)}
                    }
                },
                // (Val::DictValue{values: v1}, Val::DictValue{values: v2}) => { TODO
                //     v1.clone().append(&mut v2.clone());
                //     Value{
                //         value: Val::ListValue{values: v1},
                //         val_type: left.val_type.clone()
                //     }
                // },
                _ => {
                    // TODO: Throw error here, invalid operand
                    make_error_value()
                }
            },
            _ => {
                // TODO: Throw error here, invalid operand
                make_error_value()
            }
        }
    }
}