use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::str::FromStr;

use crate::defs::expression::{Exp, Expression};
use crate::defs::keyword::Keyword;
use crate::Type;
use crate::defs::retl_type::Type::*;
use crate::interpreter::value::{Value, Env, Val};
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

    pub fn interpret(&self, ident: Keyword, env: &Env) -> Value {
        let (args, rt): (Vec<Value>, Type) = self.get_meta(ident.clone(), env);

        match ident {
            Keyword::Readln => {
                let mut line = String::new();
                io::stdin().read_line(&mut line).expect("Expected input");
                Value{value: Val::StringValue{value: line}, val_type: rt}
            },
            Keyword::Println => {
                println!("{}", match &args[0].value { // TODO: FIX OUTPUT
                    Val::StringValue{value} => value.as_str(),
                    _ => ""
                });
                let _ = io::stdout().flush();
                null_val()
            },
            _ => Value{value: Val::Error, val_type: UnknownType}
        }
    }

    pub fn init() -> Builtin {
        let mut builtins = HashMap::new();
        builtins.insert("readln".to_string(), BuiltinMeta { params: vec![], return_type: StringType });
        builtins.insert("println".to_string(), BuiltinMeta { params: vec![("str".to_string(), StringType)], return_type: NullType });
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