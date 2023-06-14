use std::collections::HashMap;
use log::{debug, trace, error};

use crate::defs::expression::{Exp, Expression, Literal, Parameter};
use crate::defs::retl_type::type_conforms;
use crate::defs::retl_type::Type;
use crate::interpreter::value::{Value, Env, Val};
use crate::interpreter::value::Val::FuncValue;
use crate::scanner::token::{get_fp_from_token, make_empty_token, Token};

pub struct Interpreter {
    pub error: bool,
    root_exp: Exp
}

pub fn make_error_value() -> Value {
    Value{value: Val::Error, val_type: Type::UnknownType}
}

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
            Expression::Lambda{..} => self.interpret_lambda(&exp, env, expected_type),
            Expression::Application{..} => self.interpret_application(&exp, env, expected_type),
            Expression::Primitive{..} => self.interpret_primitive(&exp, env, expected_type),
            Expression::Reference{..} => self.interpret_reference(&exp, env, expected_type),
            Expression::Branch{..} => self.interpret_branch(&exp, env, expected_type),
            Expression::ListDef{..} => self.interpret_list_def(&exp, env, expected_type),
            Expression::TupleDef{..} => self.interpret_tuple_def(&exp, env, expected_type),
            Expression::TupleAccess{..} => self.interpret_tuple_access(&exp, env, expected_type),
            Expression::DictDef{..} => self.interpret_dict_def(&exp, env, expected_type),
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
            _ => {make_error_value()}
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
            _ => {make_error_value()}
        }
    }

    // fn interpret_alias(&mut self) -> Value {
    //
    // }

    fn interpret_lambda(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_lambda: {:?}", exp);
        match &exp.exp {
            Expression::Lambda{params, return_type, body} => {
                type_conforms(
                    &Type::FuncType{
                        param_types: params.iter().map(|p: &Parameter| {p.param_type.clone()}).collect(),
                        return_type: Box::new(return_type.clone())
                    },
                    &expected_type,
                    &exp.token
                );
                Value{
                    value: FuncValue{
                        builtin: false,
                        parameters: params.iter()
                            .map(|p: &Parameter| {(p.ident.clone(), p.param_type.clone())})
                            .collect(),
                        body: *body.clone(),
                        env: env.clone()
                    },
                    val_type: Type::IntType
                }
            },
            _ => {make_error_value()}
        }
    }

    fn interpret_application(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_application: {:?}", exp);
        match &exp.exp {
            Expression::Application{ident, args} => {
                let ident_value = self.interpret(ident, env, &Type::UnknownType); // TODO??
                match ident_value.value {
                    Val::ListValue{values} => {
                        if args.len() != 1 {
                            // TODO: Throw error here, argument size must be 1
                            make_error_value()
                        } else {
                            match ident_value.val_type {
                                Type::ListType{list_type} => {
                                    type_conforms(&list_type, expected_type, &exp.token);
                                    let arg = self.interpret(args.get(0).unwrap(), env, &Type::IntType);
                                    match arg.value {
                                        Val::IntValue{value} => {
                                            match values.get(value as usize) {
                                                Some(value) => value.clone(),
                                                _ => {
                                                    // TODO: Throw error here, invalid index
                                                    make_error_value()
                                                }
                                            }
                                        }
                                        _ => {make_error_value()}
                                    }
                                }
                                _ => {make_error_value()}
                            }
                        }
                    },
                    Val::DictValue{values} => {
                        if args.len() != 1 {
                            // TODO: Throw error here, argument size must be 1
                            make_error_value()
                        } else {
                            match ident_value.val_type {
                                Type::DictType{key_type, value_type} => {
                                    type_conforms(&*value_type, expected_type, &exp.token);
                                    let arg = self.interpret(args.get(0).unwrap(), env, &*key_type);
                                    match values.iter().find(|v| {v.0 == arg}) {
                                        Some(value) => value.clone().1,
                                        _ => {
                                            // TODO: Throw error here, key does not exist
                                            make_error_value()
                                        }
                                    }
                                },
                                _ => {make_error_value()}
                            }
                        }
                    },
                    // Val::FuncValue{builtin, parameters, body, env} => {
                    //     // check args size matches param size
                    //     // check arg types match param types
                    //     // check return type matches expected type
                    //     // if builtin, call builtin func
                    //     // interpret params and add to env
                    //     // interpret body and return result
                    // },
                    _ => {
                        // TODO: Throw error here, application can not be done on exp type
                        make_error_value()
                    }
                }
            },
            _ => {make_error_value()}
        }
    }

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
            _ => {make_error_value()}
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
            _ => {make_error_value()}
        }
    }

    fn interpret_branch(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_branch: {:?}", exp);
        match &exp.exp {
            Expression::Branch{condition, if_branch, else_branch } => {
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
            _ => {make_error_value()}
        }
    }

    fn interpret_list_def(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_list_def: {:?}", exp);
        match &exp.exp {
            Expression::ListDef{values} => {
                let expected_list_type = type_conforms(&exp.exp_type, expected_type, &exp.token);
                let list_type = match expected_list_type.clone() {
                    Type::ListType{list_type} => *list_type,
                    _ => Type::UnknownType
                };
                let list_values: Vec<Value> = values.iter().map(|e: &Exp| {
                    self.interpret(e, env, &list_type)
                }).collect();
                Value{value: Val::ListValue{values: list_values}, val_type: expected_list_type}
            },
            _ => {make_error_value()}
        }
    }

    fn interpret_tuple_def(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_tuple_def: {:?}", exp);
        match &exp.exp {
            Expression::TupleDef{values} => {
                let expected_tuple_type = type_conforms(&exp.exp_type, expected_type, &exp.token);
                let mut expected_tuple_types = match expected_tuple_type {
                    Type::TupleType{tuple_types} => tuple_types,
                    _ => vec![Type::UnknownType; values.len()]
                };
                let tuple_values: Vec<Value> = values.iter().zip(expected_tuple_types)
                    .map(|(e, t): (&Exp, Type)| { self.interpret(e, env, &t) })
                    .collect();
                let tuple_types: Vec<Type> = tuple_values.iter()
                    .map(|tv: &Value| { tv.val_type.clone() }).collect();
                Value{
                    value: Val::TupleValue{values: tuple_values},
                    val_type: Type::TupleType{tuple_types}
                }
            },
            _ => {make_error_value()}
        }
    }

    fn interpret_tuple_access(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_tuple_access: {:?}", exp);
        match &exp.exp {
            Expression::TupleAccess{ident, index} => {
                let tuple_value = self.interpret(&**ident, env, &exp.exp_type);
                match tuple_value.value {
                    Val::TupleValue{values} => {
                        let tuple_element = values[*index].clone();
                        type_conforms(&tuple_element.val_type, expected_type, &exp.token);
                        tuple_element
                    },
                    _ => {
                        // TODO: Throw error here, not a tuple
                        make_error_value()
                    }
                }
            },
            _ => {make_error_value()}
        }
    }

    fn interpret_dict_def(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_dict_def: {:?}", exp);
        match &exp.exp {
            Expression::DictDef{mapping} => {
                let expected_dict_type = type_conforms(&exp.exp_type, expected_type, &exp.token);
                let (key_type, value_type): (Type, Type) = match expected_dict_type.clone() {
                    Type::DictType{key_type, value_type} => (*key_type, *value_type),
                    _ => (Type::UnknownType, Type::UnknownType)
                };
                let mut dict_values: Vec<(Value, Value)> = vec![];
                let _ = mapping.keys()
                    .for_each(|key: &Literal| {
                        let hashmap_value = mapping[key].clone();
                        let key_value = Exp {
                            exp: Expression::Lit { lit: key.clone() },
                            exp_type: key_type.clone(),
                            token: exp.token.clone()
                        };
                        dict_values.push((self.interpret(&key_value, env, &key_type.clone()),
                                          self.interpret(&hashmap_value, env, &value_type.clone())));
                    });
                Value{
                    value: Val::DictValue{values: dict_values},
                    val_type: Type::DictType{key_type: Box::new(key_type), value_type: Box::new(value_type)}
                }
            },
            _ => {make_error_value()}
        }
    }

    // fn interpret_schema_def(&mut self) -> Value {
    //
    // }
}