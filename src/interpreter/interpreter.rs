use log::{debug, trace, error};

use crate::defs::expression::{Exp, Expression, Literal};
use crate::defs::retl_type::type_conforms;
use crate::defs::retl_type::Type;
use crate::interpreter::value::{Value, Env, Val};
use crate::scanner::token::{get_fp_from_token, make_empty_token, Token};

pub struct Interpreter {
    pub error: bool,
    root_exp: Exp
}

pub fn make_error_value() -> Value { Value{value: Val::Error, val_type: Type::UnknownType} }

impl Interpreter {
    pub fn init() -> Interpreter {
        Interpreter{
            error: false,
            root_exp: Exp{
                exp: Expression::Empty,
                exp_type: Type::NullType,
                token: make_empty_token()
            }
        }
    }

    pub fn interpret(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret: {:?}", exp);
        match &exp.exp {
            Expression::Lit{..} => self.interpret_literal(&exp, expected_type),
            Expression::Let{..} => self.interpret_let(&exp, env, expected_type),
            Expression::Primitive{..} => self.interpret_primitive(&exp, env, expected_type),
            Expression::Reference{..} => self.interpret_reference(&exp, env, expected_type),
            Expression::Branch{..} => self.interpret_branch(&exp, env, expected_type),
            Expression::ListDef{..} => self.interpret_list_def(&exp, env, expected_type),
            Expression::TupleDef{..} => self.interpret_tuple_def(&exp, env, expected_type),
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
            }
        }
    }

    fn interpret_literal(&mut self, exp: &Exp, expected_type: &Type) -> Value {
        trace!("interpret_literal: {:?}", exp);
        type_conforms(&exp.exp_type, expected_type, &exp.token);
        match &exp.exp {
            Expression::Lit{lit} => {
                match lit {
                    Literal::IntLit{literal} =>
                        Value{value: Val::IntValue{value: literal.clone()}, val_type: Type::IntType},
                    Literal::BoolLit{literal} =>
                        Value{value: Val::BoolValue{value: literal.clone()}, val_type: Type::BoolType},
                    Literal::CharLit{literal} =>
                        Value{value: Val::CharValue{value: literal.clone()}, val_type: Type::CharType},
                    Literal::StringLit{literal} =>
                        Value{value: Val::StringValue{value: literal.clone()}, val_type: Type::StringType},
                    Literal::NullLit =>
                        Value{value: Val::NullValue, val_type: Type::NullType}
                }
            },
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
            }
        }
    }

    fn interpret_let(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_let: {:?}", exp);
        match &exp.exp {
            Expression::Let{ident, let_type, let_exp, after_let_exp} => {
                let resolved_exp = self.interpret(let_exp, env, let_type);
                env.insert(ident.clone(), resolved_exp);
                match &**after_let_exp {
                    Some(after) => self.interpret(after, env, expected_type),
                    _ => Value{value: Val::NullValue, val_type: Type::NullType}
                }
            },
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
            }
        }
    }

    // fn interpret_alias(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_lambda(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_match(&mut self) -> Value {
    //
    // }

    fn interpret_primitive(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_primitive: {:?}", exp);
        match &exp.exp {
            Expression::Primitive{operator, left, right} => {
                let op_type = exp.exp_type.clone();
                let left_value = self.interpret(left, env, &op_type);
                let right_value = self.interpret(right, env, &op_type);
                let result = operator.interpret(&left_value, &right_value);
                type_conforms(&result.val_type, expected_type, &exp.token);
                result
            },
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
            }
        }
    }

    fn interpret_reference(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_reference: {:?}", exp);
        match &exp.exp {
            Expression::Reference{ident} => {
                let ref_value = env.get(ident).clone();
                match ref_value {
                    Some(r) => {
                        type_conforms(&r.val_type, expected_type, &exp.token);
                        r.clone()
                    },
                    _ => {
                        // TODO: Throw error here, reference does not exist
                        make_error_value()
                    }
                }
            },
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
            }
        }
    }

    fn interpret_branch(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_branch: {:?}", exp);
        match &exp.exp {
            Expression::Branch{
                condition,
                if_branch,
                else_branch
            } => {
                match &**else_branch {
                    Some(else_exp) => type_conforms(&if_branch.exp_type, &else_exp.exp_type, &exp.token),
                    _ => type_conforms(&if_branch.exp_type, &Type::NullType, &exp.token)
                };

                match self.interpret(&**condition, env, &Type::BoolType).value {
                    Val::BoolValue{value} => {
                        if value {
                            self.interpret(&if_branch, env, expected_type)
                        } else {
                            match &**else_branch {
                                Some(else_exp) => self.interpret(&else_exp, env, expected_type),
                                _ => Value{value: Val::NullValue, val_type: Type::NullType }
                            }
                        }
                    },
                    _ => {
                        // TODO: Throw error here, not a valid condition
                        make_error_value()
                    }
                }
            },
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
            }
        }
    }

    fn interpret_list_def(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_list_def: {:?}", exp);
        match &exp.exp {
            Expression::ListDef{values} => {
                let mut list_values: Vec<Value> = vec![];
                let expected_list_type = type_conforms(&exp.exp_type, expected_type, &exp.token);
                let list_type = match expected_list_type.clone() {
                    Type::ListType{list_type} => *list_type,
                    _ => Type::UnknownType
                };
                for v in values {
                    list_values.push(self.interpret(v, env, &list_type))
                }
                Value{value: Val::ListValue{values: list_values}, val_type: expected_list_type}
            },
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
            }
        }
    }

    fn interpret_tuple_def(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_tuple_def: {:?}", exp);
        match &exp.exp {
            Expression::TupleDef{values} => {
                let mut tuple_values: Vec<Value> = vec![];
                let expected_tuple_type = type_conforms(&exp.exp_type, expected_type, &exp.token);
                let mut expected_tuple_types = match expected_tuple_type {
                    Type::TupleType{tuple_types} => tuple_types,
                    _ => vec![Type::UnknownType; values.len()]
                };
                for (v, t) in values.iter().zip(expected_tuple_types) {
                    tuple_values.push(self.interpret(v, env, &t))
                }
                let mut tuple_types: Vec<Type> = vec![];
                for t in tuple_values.clone() {
                    tuple_types.push(t.val_type)
                }
                Value{
                    value: Val::TupleValue{values: tuple_values},
                    val_type: Type::TupleType{tuple_types}
                }
            },
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
            }
        }
    }

    // fn interpret_tuple_access(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_dict_def(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_schema_def(&mut self) -> Value {
    //
    // }
}