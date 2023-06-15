use strum_macros::Display;
use crate::{Exp, Type, Value};
use crate::defs::retl_type::{concat_tuple_types, type_conforms};
use crate::interpreter::interpreter::error;
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
            Operator::And | Operator::Or => 0,
            Operator::Plus | Operator::Minus | Operator::CollectionConcat => 2,
            Operator::Multiply | Operator::Divide | Operator::Modulus => 3,
            _ => 1
        }
    }

    pub fn is_binary_op(&self, min: i32) -> bool {
        (self.is_boolean_op() || self.is_arithmetic_op() || self.is_collection_op()) &&
            self.get_precedence() >= min
    }

    pub fn get_type(&self, left: &mut Exp, right: &mut Exp) -> Type {
        match *self {
            Operator::Plus => type_conforms(&left.exp_type, &right.exp_type, &left.token),
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
            Operator::Or => Type::BoolType,
            Operator::CollectionConcat => type_conforms(&left.exp_type, &right.exp_type, &left.token)
        }
    }

    pub fn interpret(&self, left: &Value, right: &Value) -> Value {
        match *self {
            Operator::Plus => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::IntValue{value: v1 + v2}, val_type: Type::IntType}
                },
                (Val::CharValue{value: v1}, Val::CharValue{value: v2}) => {
                    Value{value: Val::StringValue{value: v1 + &*v2 }, val_type: Type::StringType}
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
                    error()
                }
            },
            Operator::Minus => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::IntValue{value: v1 - v2}, val_type: Type::IntType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    error()
                }
            },
            Operator::Multiply => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::IntValue{value: v1 * v2}, val_type: Type::IntType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    error()
                }
            },
            Operator::Divide => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::IntValue{value: v1 / v2}, val_type: Type::IntType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    error()
                }
            },
            Operator::Modulus => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::IntValue{value: v1 % v2}, val_type: Type::IntType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    error()
                }
            },
            Operator::GreaterThan => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 > v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    error()
                }
            },
            Operator::LessThan => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 < v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    error()
                }
            },
            Operator::GreaterThanEqualTo => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 >= v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    error()
                }
            },
            Operator::LessThanEqualTo => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 <= v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    error()
                }
            },
            Operator::Equal => match (left.value.clone(), right.value.clone()) {
                (Val::IntValue{value: v1}, Val::IntValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 == v2}, val_type: Type::BoolType}
                },
                (Val::BoolValue{value: v1}, Val::BoolValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 == v2}, val_type: Type::BoolType}
                },
                (Val::CharValue{value: v1}, Val::CharValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 == v2}, val_type: Type::BoolType}
                },
                (Val::StringValue{value: v1}, Val::StringValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 == v2}, val_type: Type::BoolType}
                },
                (Val::ListValue{values: v1}, Val::ListValue{values: v2}) => {
                    Value{
                        value: Val::BoolValue{value: v1.iter().zip(v2.clone()).all(|(l1, l2)| {
                            match self.interpret(l1, &l2).value {
                                Val::BoolValue{value} => value,
                                _ => false
                            }
                        })},
                        val_type: Type::BoolType
                    }
                },
                (Val::TupleValue{values: v1}, Val::TupleValue{values: v2}) => {
                    Value{
                        value: Val::BoolValue{value: v1.iter().zip(v2.clone()).all(|(t1, t2)| {
                            match self.interpret(t1, &t2).value {
                                Val::BoolValue{value} => value,
                                _ => false
                            }
                        })},
                        val_type: Type::BoolType
                    }
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    error()
                }
            },
            Operator::And => match (left.value.clone(), right.value.clone()) {
                (Val::BoolValue{value: v1}, Val::BoolValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 && v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    error()
                }
            },
            Operator::Or => match (left.value.clone(), right.value.clone()) {
                (Val::BoolValue{value: v1}, Val::BoolValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 || v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    error()
                }
            },
            Operator::Not => match (left.value.clone(), right.value.clone()) {
                (Val::BoolValue{value: v1}, Val::BoolValue{value: v2}) => {
                    Value{value: Val::BoolValue{value: v1 == v2}, val_type: Type::BoolType}
                },
                _ => {
                    // TODO: Throw error here, invalid types for operand
                    error()
                }
            },
            Operator::CollectionConcat => match (left.value.clone(), right.value.clone()) {
                (Val::ListValue{values: v1}, Val::ListValue{values: v2}) => {
                    let mut concat_list = v1.clone();
                    concat_list.append(&mut v2.clone());
                    Value{
                        value: Val::ListValue{values: concat_list},
                        val_type: left.val_type.clone()
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
                    error()
                }
            }
        }
    }
}