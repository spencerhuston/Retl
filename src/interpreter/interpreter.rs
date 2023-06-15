use std::borrow::Borrow;
use log::{error, trace};
use regex::internal::Input;
use crate::Builtin;

use crate::defs::expression::{Exp, Expression, Literal, Parameter, Pattern};
use crate::defs::retl_type::{type_conforms, type_conforms_no_error};
use crate::defs::retl_type::Type;
use crate::interpreter::value::{Value, Env, Val};
use crate::scanner::token::{get_fp_from_token, make_empty_token};

pub struct Interpreter {
    pub error: bool,
    root_exp: Exp,
    builtin: Builtin
}

pub fn error() -> Value {
    Value{value: Val::Error, val_type: Type::UnknownType}
}

impl Interpreter {
    pub fn init(builtin: &Builtin) -> Interpreter {
        Interpreter{
            error: false,
            root_exp: Exp{
                exp: Expression::Empty,
                exp_type: Type::NullType,
                token: make_empty_token()
            },
            builtin: builtin.clone()
        }
    }

    pub fn interpret(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret: {:?}", exp);
        match &exp.exp {
            Expression::Lit{..} => self.interpret_literal(&exp, expected_type),
            Expression::Let{..} => self.interpret_let(&exp, env, expected_type),
            Expression::Alias{..} => self.interpret_alias(&exp, env, expected_type),
            Expression::Lambda{..} => self.interpret_lambda(&exp, env, expected_type),
            Expression::Application{..} => self.interpret_application(&exp, env, expected_type),
            Expression::Match{..} => self.interpret_match(&exp, env, expected_type),
            Expression::Primitive{..} => self.interpret_primitive(&exp, env, expected_type),
            Expression::Reference{..} => self.interpret_reference(&exp, env, expected_type),
            Expression::Branch{..} => self.interpret_branch(&exp, env, expected_type),
            Expression::ListDef{..} => self.interpret_list_def(&exp, env, expected_type),
            Expression::TupleDef{..} => self.interpret_tuple_def(&exp, env, expected_type),
            Expression::TupleAccess{..} => self.interpret_tuple_access(&exp, env, expected_type),
            Expression::DictDef{..} => self.interpret_dict_def(&exp, env, expected_type),
            Expression::SchemaDef{..} => self.interpret_schema_def(&exp, env, expected_type),
            _ => {
                // TODO: Throw error here, invalid expression
                error()
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
            _ => error()
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
            _ => error()
        }
    }

    fn interpret_alias(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_alias: {:?}", exp);
        match &exp.exp {
            Expression::Alias{ident, alias, after_alias_exp} => {
                match &**after_alias_exp {
                    Some(after_exp) => self.interpret(after_exp, env, &expected_type),
                    _ => Value{value: Val::NullValue, val_type: Type::NullType}
                }
            },
            _ => error()
        }
    }

    fn interpret_lambda(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_lambda: {:?}", exp);
        match &exp.exp {
            Expression::Lambda{params, return_type, body} => {
                let func_type = Type::FuncType{
                    param_types: params.iter().map(|p: &Parameter| {p.param_type.clone()}).collect(),
                    return_type: Box::new(return_type.clone())
                };
                type_conforms(&func_type, &expected_type, &exp.token);
                Value{
                    value: Val::FuncValue{
                        builtin_ident: None,
                        parameters: params.iter()
                            .map(|p: &Parameter| {(p.ident.clone(), p.param_type.clone())})
                            .collect(),
                        body: *body.clone(),
                        env: env.clone()
                    },
                    val_type: func_type
                }
            },
            _ => error()
        }
    }

    fn interpret_application(&mut self, exp: &Exp, app_env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_application: {:?}", exp);
        match &exp.exp {
            Expression::Application{ident, args} => {
                let ident_value = self.interpret(ident, app_env, &Type::UnknownType);
                match ident_value.value {
                    Val::StringValue{value} => {
                        if args.len() != 1 {
                            // TODO: Throw error here, argument size must be 1
                            error()
                        } else {
                            type_conforms(&ident_value.val_type, expected_type, &exp.token);
                            let arg = self.interpret(args.get(0).unwrap(), app_env, &Type::IntType);
                            let string_val = value.clone();
                            match arg.value {
                                Val::IntValue{value} => {
                                    match string_val.get(value as usize..(value + 1) as usize) {
                                        Some(char) if char != "\"" => Value{
                                            value: Val::CharValue{value: char.to_string()},
                                            val_type: Type::CharType
                                        },
                                        _ => {
                                            // TODO: Throw error here, invalid index
                                            error()
                                        }
                                    }
                                }
                                _ => error()
                            }
                        }
                    },
                    Val::ListValue{values} => {
                        if args.len() != 1 {
                            // TODO: Throw error here, argument size must be 1
                            error()
                        } else {
                            match ident_value.val_type {
                                Type::ListType{list_type} => {
                                    type_conforms(&list_type, expected_type, &exp.token);
                                    let arg = self.interpret(args.get(0).unwrap(), app_env, &Type::IntType);
                                    match arg.value {
                                        Val::IntValue{value} => {
                                            match values.get(value as usize) {
                                                Some(value) => value.clone(),
                                                _ => {
                                                    // TODO: Throw error here, invalid index
                                                    error()
                                                }
                                            }
                                        }
                                        _ => error()
                                    }
                                }
                                _ => error()
                            }
                        }
                    },
                    Val::DictValue{values} => {
                        if args.len() != 1 {
                            // TODO: Throw error here, argument size must be 1
                            error()
                        } else {
                            match ident_value.val_type {
                                Type::DictType{key_type, value_type} => {
                                    type_conforms(&*value_type, expected_type, &exp.token);
                                    let arg = self.interpret(args.get(0).unwrap(), app_env, &*key_type);
                                    match values.iter().find(|v| {v.0 == arg}) {
                                        Some(value) => value.clone().1,
                                        _ => {
                                            // TODO: Throw error here, key does not exist
                                            error()
                                        }
                                    }
                                },
                                _ => error()
                            }
                        }
                    },
                    Val::FuncValue{builtin_ident, parameters, body, env} => {
                        if parameters.len() != args.len() {
                            // TODO: Throw error here, arguments do not match function parameters
                            error()
                        } else {
                            match ident_value.val_type {
                                Type::FuncType{return_type, ..} => {
                                    type_conforms(&*return_type, expected_type, &exp.token);
                                    let mut body_env = env.clone();
                                    parameters.iter().zip(args)
                                        .for_each(|pa| {
                                            let arg_value = self.interpret(&pa.1.clone(), app_env, &pa.0.1);
                                            body_env.insert(pa.0.0.clone(), arg_value);
                                        });
                                    match builtin_ident {
                                        Some(ident) => self.builtin.interpret(ident.clone(), &mut body_env),
                                        _ => self.interpret(&body, &mut body_env, &*return_type)
                                    }
                                },
                                _ => error()
                            }
                        }
                    },
                    _ => {
                        // TODO: Throw error here, application can not be done on exp type
                        error()
                    }
                }
            },
            _ => error()
        }
    }

    fn interpret_match(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_match: {:?}", exp);
        match &exp.exp {
            Expression::Match{match_exp, cases} => {
                let match_val = self.interpret(match_exp, env, &Type::UnknownType);
                match cases.iter().find(|case| {
                    match case.pattern.clone() {
                        Pattern::TypePattern{ident, case_type, predicate} => {
                            match type_conforms_no_error(&match_val.val_type.clone(), &case_type, &case.case_exp.token) {
                                Type::UnknownType => false,
                                _ => {
                                    env.insert(ident, match_val.clone());
                                    match predicate {
                                        Some(pred) => {
                                            match self.interpret(&pred, env, &Type::BoolType).value {
                                                Val::BoolValue{value} => value,
                                                _ => false
                                            }
                                        },
                                        _ => true
                                    }
                                }
                            }
                        },
                        Pattern::Literal{literal} => {
                            match (match_val.value.clone(), literal) {
                                (Val::IntValue{value}, Literal::IntLit{literal}) => value == literal,
                                (Val::BoolValue{value}, Literal::BoolLit{literal}) => value == literal,
                                (Val::CharValue{value}, Literal::CharLit{literal}) => value == literal,
                                (Val::StringValue{value}, Literal::StringLit{literal}) => value == literal,
                                (Val::NullValue, Literal::NullLit) => true,
                                _ => false
                            }
                        },
                        Pattern::MultiLiteral{literals} => {
                            literals.iter().any(|lit: &Literal| {
                                match (match_val.value.clone(), lit) {
                                    (Val::IntValue{value}, Literal::IntLit{literal}) => value == *literal,
                                    (Val::BoolValue{value}, Literal::BoolLit{literal}) => value == *literal,
                                    (Val::CharValue{value}, Literal::CharLit{literal}) => value == *literal,
                                    (Val::StringValue{value}, Literal::StringLit{literal}) => value == *literal,
                                    (Val::NullValue, Literal::NullLit) => true,
                                    _ => false
                                }
                            })
                        },
                        Pattern::Range{range} => {
                            match range {
                                Expression::ListDef{values} => {
                                    values.iter().any(|value: &Exp| {
                                        match (match_val.value.clone(), value.exp.clone()) {
                                            (Val::IntValue{value}, Expression::Lit{lit: Literal::IntLit{literal}}) => value == literal,
                                            _ => false
                                        }
                                    })
                                },
                                _ => false
                            }
                        },
                        Pattern::Any => true
                    }
                }) {
                    Some(value) => self.interpret(&value.case_exp, env, expected_type),
                    _ => {
                        // TODO: Throw error here, no matching cases
                        error()
                    }
                }
            },
            _ => error()
        }
    }

    fn interpret_primitive(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_primitive: {:?}", exp);
        match &exp.exp {
            Expression::Primitive{operator, left, right} => {
                let left_value = self.interpret(left, env, &Type::UnknownType);
                let right_value = self.interpret(right, env, &Type::UnknownType);
                let result = operator.interpret(&left_value, &right_value);
                type_conforms(&result.val_type, expected_type, &exp.token);
                result
            },
            _ => error()
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
                        error!("Reference \"{}\" does not exist: {}",
                                ident,
                                get_fp_from_token(&exp.token));
                        error()
                    }
                }
            },
            _ => error()
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
                        error()
                    }
                }
            },
            _ => error()
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
            _ => error()
        }
    }

    fn interpret_tuple_def(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_tuple_def: {:?}", exp);
        match &exp.exp {
            Expression::TupleDef{values} => {
                let expected_tuple_type = type_conforms(&exp.exp_type, expected_type, &exp.token);
                let expected_tuple_types = match expected_tuple_type {
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
            _ => error()
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
                        error()
                    }
                }
            },
            _ => error()
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
            _ => error()
        }
    }

    fn interpret_schema_def(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        trace!("interpret_schema_def: {:?}", exp);
        match &exp.exp {
            Expression::SchemaDef{mapping} => {
                Value{
                    value: Val::SchemaValue{values: mapping.clone()},
                    val_type: Type::SchemaType
                }
            },
            _ => error()
        }
    }
}