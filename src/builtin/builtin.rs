use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::str::FromStr;

use crate::defs::expression::{Exp, Expression};
use crate::defs::keyword::Keyword;
use crate::{Interpreter, Type};
use crate::defs::retl_type::Type::*;
use crate::defs::retl_type::type_conforms;
use crate::interpreter::value::{Value, Env, Val};
use crate::interpreter::interpreter::error;
use crate::scanner::token::make_empty_token;

fn null_val() -> Value {
    Value{value: Val::NullValue, val_type: NullType}
}

fn func_value(ident: &str, params: Vec<(String, Type)>, return_type: Type) -> Value {
    let func_type = FuncType{
        param_types: params.iter().map(|p: &(String, Type)| { p.1.clone() }).collect(),
        return_type: Box::new(return_type)
    };
    Value{
        value: Val::FuncValue{
            builtin_ident: match Keyword::from_str(&ident) {
                Ok(keyword_ident) => Some(keyword_ident),
                _ => None
            },
            parameters: params.iter()
                .map(|p: &(String, Type)| { (p.0.to_owned(), p.1.clone()) })
                .collect(),
            body: Exp{
                exp: Expression::Empty,
                exp_type: func_type.clone(),
                token: make_empty_token()
            },
            env: Env::new()
        },
        val_type: func_type
    }
}

#[derive(Clone)]
struct BuiltinMeta {
    params: Vec<(String, Type)>,
    return_type: Type
}

#[derive(Clone)]
pub struct Builtin {
    builtins: HashMap<String, BuiltinMeta>
}

fn value_to_string(val: &Val) -> Option<String> { // TODO - Collection types
    match val {
        Val::IntValue{value} => Some(value.to_string()),
        Val::BoolValue{value} => Some(if *value { "true".to_string() } else { "false".to_string() }),
        Val::CharValue{value} => Some(value.to_string()),
        Val::StringValue{value} => Some(value.clone()),
        _ => None
    }
}

impl Builtin {
    fn get_meta(&self, ident: Keyword, env: &Env) -> (Vec<Value>, Type) {
        match self.builtins.get(&*ident.to_string()) {
            Some(bm) => {
                (bm.params.clone().iter().map(|p: &(String, Type)| {
                    match env.get(p.0.as_str()) {
                        Some(arg) => arg.clone(),
                        _ => null_val()
                    }
                }).collect(),
                 bm.return_type.clone())
            },
            _ => (vec![], UnknownType)
        }
    }

    pub fn interpret(&self, ident: Keyword, env: &Env, exp: &Exp, interpreter: Interpreter) -> Value {
        let (args, rt): (Vec<Value>, Type) = self.get_meta(ident.clone(), env);

        match ident {
            Keyword::Readln => {
                let mut line = String::new();
                io::stdin().read_line(&mut line).expect("Expected input");
                Value{value: Val::StringValue{value: line}, val_type: rt}
            },
            Keyword::Println => {
                let str = value_to_string(&args[0].value);
                match str {
                    Some(str_val) => {
                        println!("{}", str_val);
                        let _ = io::stdout().flush();
                        null_val()
                    },
                    _ => error("Invalid argument type for \"println\"", exp)
                }
            },
            Keyword::Print => {
                let str = value_to_string(&args[0].value);
                match str {
                    Some(str_val) => {
                        print!("{}", str_val);
                        let _ = io::stdout().flush();
                        null_val()
                    },
                    _ => error("Invalid argument type for \"print\"", exp)
                }
            },
            Keyword::Map => self.map(args, exp, interpreter),
            Keyword::Filter => self.filter(args, exp, interpreter),
            Keyword::Type => {
                Value{value: Val::StringValue{value: args[0].val_type.as_string()}, val_type: StringType}
            },
            _ => Value{value: Val::Error, val_type: UnknownType}
        }
    }

    fn map(&self, args: Vec<Value>, exp: &Exp, interpreter: Interpreter) -> Value {
        let collection = &args[0];
        let collection_iterator_type = match args[1].val_type.clone() {
            FuncType{return_type, ..} => *return_type,
            _ => NullType
        };
        match &args[1].value.clone() {
            Val::FuncValue{builtin_ident, parameters, body, env} => {
                match collection.value.clone() {
                    Val::ListValue{values} => {
                        let mapped_values: Vec<Value> = values.iter().map(|v: &Value| {
                            let mut temp_body_env = env.clone();
                            type_conforms(&parameters[0].1, &v.val_type, &exp.token);
                            temp_body_env.insert(parameters[0].0.clone(), v.clone());
                            match builtin_ident {
                                Some(bi) => self.interpret(bi.clone(), &mut temp_body_env, body, interpreter.clone()),
                                _ => interpreter.clone().interpret(body, &mut temp_body_env, &collection_iterator_type.clone())
                            }
                        }).collect();
                        Value{value: Val::ListValue{values: mapped_values}, val_type: ListType{list_type: Box::new(collection_iterator_type)}}
                    },
                    _ => error("Invalid collection type for \"map\"", exp)
                }
            },
            _ => error("Invalid function type for \"map\"", exp)
        }
    }

    fn filter(&self, args: Vec<Value>, exp: &Exp, interpreter: Interpreter) -> Value {
        let collection = &args[0];
        let collection_iterator_type = match args[1].val_type.clone() {
            FuncType{param_types, return_type} => {
                type_conforms(&*return_type, &BoolType, &exp.token);
                param_types[0].clone()
            },
            _ => NullType
        };
        match &args[1].value.clone() {
            Val::FuncValue{builtin_ident, parameters, body, env} => {
                match collection.value.clone() {
                    Val::ListValue{values} => {
                        let mut filtered_values: Vec<Value> = vec![];
                        values.iter().for_each(|v: &Value| {
                            let mut temp_body_env = env.clone();
                            type_conforms(&parameters[0].1, &v.val_type, &exp.token);
                            temp_body_env.insert(parameters[0].0.clone(), v.clone());
                            let result = match builtin_ident {
                                Some(bi) => self.interpret(bi.clone(), &mut temp_body_env, body, interpreter.clone()),
                                _ => interpreter.clone().interpret(body, &mut temp_body_env, &BoolType)
                            };
                            match result.value {
                                Val::BoolValue{value} => if value { filtered_values.push(v.clone()) },
                                _ => {}
                            }
                        });
                        Value{value: Val::ListValue{values: filtered_values}, val_type: ListType{list_type: Box::new(collection_iterator_type)}}
                    },
                    _ => error("Invalid collection type for \"filter\"", exp)
                }
            },
            _ => error("Invalid function type for \"filter\"", exp)
        }
    }

    pub fn init() -> Builtin {
        let mut builtins = HashMap::new();
        builtins.insert("readln".to_string(), BuiltinMeta { params: vec![], return_type: StringType });
        builtins.insert("println".to_string(), BuiltinMeta { params: vec![("str".to_string(), Any)], return_type: NullType });
        builtins.insert("print".to_string(), BuiltinMeta { params: vec![("str".to_string(), Any)], return_type: NullType });
        builtins.insert("map".to_string(), BuiltinMeta { params: vec![
            ("c".to_string(), Any),
            ("f".to_string(), FuncType{param_types: vec![Any], return_type: Box::new(Any)})
        ], return_type: Any });
        builtins.insert("filter".to_string(), BuiltinMeta { params: vec![
            ("c".to_string(), Any),
            ("f".to_string(), FuncType{param_types: vec![Any], return_type: Box::new(Any)})
        ], return_type: Any });
        builtins.insert("type".to_string(), BuiltinMeta { params: vec![("val".to_string(), UnknownType)], return_type: StringType });
        Builtin{builtins}
    }

    pub fn load_builtins(&self, env: &Env) -> Env {
        let mut builtin_env = env.clone();
        self.builtins.keys().for_each(|k: &String| {
            let b = self.builtins.get(k).unwrap();
            builtin_env.insert(k.clone(), func_value(k, b.params.clone(), b.return_type.clone()));
        });
        builtin_env
    }
}