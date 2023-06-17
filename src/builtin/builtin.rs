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
    pub fn init() -> Builtin {
        let mut builtins = HashMap::new();
        builtins.insert("readln".to_string(), BuiltinMeta { params: vec![], return_type: StringType });
        builtins.insert("readCSV".to_string(), BuiltinMeta {
            params: vec![
                ("path".to_string(), StringType),
                ("schema".to_string(), SchemaType)
            ],
            return_type: ListType{list_type: Box::new(Any)}
        });
        builtins.insert("writeCSV".to_string(), BuiltinMeta {
            params: vec![
                ("path".to_string(), StringType),
                ("table".to_string(), ListType{list_type: Box::new(Any)}),
                ("schema".to_string(), SchemaType)
            ],
            return_type: NullType
        });
        builtins.insert("println".to_string(), BuiltinMeta { params: vec![("str".to_string(), Any)], return_type: NullType });
        builtins.insert("print".to_string(), BuiltinMeta { params: vec![("str".to_string(), Any)], return_type: NullType });
        builtins.insert("map".to_string(), BuiltinMeta { params: vec![
            ("l".to_string(), ListType{list_type: Box::new(Any)}),
            ("f".to_string(), FuncType{param_types: vec![Any], return_type: Box::new(Any)})
        ], return_type: Any });
        builtins.insert("filter".to_string(), BuiltinMeta { params: vec![
            ("l".to_string(), ListType{list_type: Box::new(Any)}),
            ("f".to_string(), FuncType{param_types: vec![Any], return_type: Box::new(Any)})
        ], return_type: Any });
        builtins.insert("foldl".to_string(), BuiltinMeta { params: vec![
            ("acc".to_string(), Any),
            ("l".to_string(), ListType{list_type: Box::new(Any)}),
            ("f".to_string(), FuncType{param_types: vec![Any, Any], return_type: Box::new(Any)})
        ], return_type: Any });
        builtins.insert("foldr".to_string(), BuiltinMeta { params: vec![
            ("acc".to_string(), Any),
            ("l".to_string(), ListType{list_type: Box::new(Any)}),
            ("f".to_string(), FuncType{param_types: vec![Any, Any], return_type: Box::new(Any)})
        ], return_type: Any });
        builtins.insert("substr".to_string(), BuiltinMeta { params: vec![
            ("str".to_string(), StringType),
            ("s".to_string(), IntType),
            ("e".to_string(), IntType)
        ], return_type: StringType});
        builtins.insert("zip".to_string(), BuiltinMeta { params: vec![
            ("l1".to_string(), ListType{list_type: Box::new(Any)}),
            ("l2".to_string(), ListType{list_type: Box::new(Any)})
        ], return_type: ListType{list_type: Box::new(TupleType{tuple_types: vec![Any, Any]})} });
        builtins.insert("len".to_string(), BuiltinMeta { params: vec![("c".to_string(), Any)], return_type: IntType });
        builtins.insert("type".to_string(), BuiltinMeta { params: vec![("v".to_string(), Any)], return_type: StringType });
        builtins.insert("intToString".to_string(), BuiltinMeta { params: vec![("i".to_string(), IntType)], return_type: StringType });
        builtins.insert("stringToInt".to_string(), BuiltinMeta { params: vec![("s".to_string(), StringType)], return_type: IntType });
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
            Keyword::ReadCSV => self.read_csv(args, exp),
            Keyword::WriteCSV => self.write_csv(args, exp),
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
            Keyword::Foldl => self.fold(args, exp, interpreter, true),
            Keyword::Foldr => self.fold(args, exp, interpreter, false),
            Keyword::Substr => self.substr(args, exp),
            Keyword::Zip => self.zip(args, exp),
            Keyword::Type => {
                Value{value: Val::StringValue{value: args[0].val_type.as_string()}, val_type: StringType}
            },
            Keyword::Len => {
                let size = match &args[0].value {
                    Val::StringValue{value} => Some(value.len()),
                    Val::ListValue{values} => {
                        type_conforms(&args[0].val_type, &ListType{list_type: Box::new(Any)}, &exp.token);
                        Some(values.len())
                    },
                    Val::TupleValue{values} => {
                        type_conforms(&args[0].val_type, &TupleType{tuple_types: vec![Any; values.len()]}, &exp.token);
                        Some(values.len())
                    },
                    Val::DictValue{values} => {
                        type_conforms(&args[0].val_type, &DictType{key_type: Box::new(Any), value_type: Box::new(Any)}, &exp.token);
                        Some(values.len())
                    },
                    _ => None
                };
                match size {
                    Some(size) => Value{value: Val::IntValue{value: size as i32}, val_type: IntType},
                    _ => {
                        error("Invalid argument type for \"len\"", exp);
                        Value{value: Val::IntValue{value: -1}, val_type: IntType}
                    }
                }
            },
            Keyword::IntToString => self.int_to_string(args),
            Keyword::StringToInt => self.string_to_int(args),
            _ => Value{value: Val::Error, val_type: UnknownType}
        }
    }

    fn row_entry_to_value(&self, column_type: &Type, element: &str, exp: &Exp) -> Value {
        match column_type {
            IntType => Value{
                value: Val::IntValue{value: element.parse::<i32>().unwrap()},
                val_type: IntType
            },
            BoolType => Value{
                value: Val::BoolValue{value:
                    if element == "true" { true }
                    else if element == "false" { false }
                    else {
                        error("Invalid value for boolean column type", exp);
                        false
                    }
                },
                val_type: BoolType
            },
            CharType => Value{
                value: Val::CharValue{value: element.to_string()},
                val_type: CharType
            },
            StringType => Value{
                value: Val::StringValue{value: element.to_string()},
                val_type: StringType
            },
            _ => error("Cannot convert value to column type", exp)
        }
    }

    fn read_csv(&self, args: Vec<Value>, exp: &Exp) -> Value {
        match &args[0].value {
            Val::StringValue{value} => {
                let path = value;
                match &args[1].value {
                    Val::SchemaValue{values} => {
                        let schema = values;
                        let csv_reader = csv::ReaderBuilder::new()
                            .has_headers(true)
                            .from_path(path);
                        let mut row_values: Vec<Vec<Value>> = vec![];

                        for (row_index, record) in csv_reader.expect("Could not read CSV").records().enumerate() {
                            row_values.push(vec![]);
                            match record {
                                Ok(row) => {
                                    for (column_index, entry) in row.iter().enumerate() {
                                        match row_values.get(row_index) {
                                            Some(_) => {
                                                let column_type = match schema.get(column_index) {
                                                    Some(schema_column) => schema_column.1.clone(),
                                                    _ => UnknownType
                                                };
                                                row_values[row_index].push(self.row_entry_to_value(&column_type, entry, exp))
                                            },
                                            _ => {
                                                error("Could not read row from CSV", exp);
                                                return null_val()
                                            }
                                        };
                                    }
                                },
                                _ => {
                                    error("Could not read CSV row", exp);
                                }
                            }
                        };

                        let row_type = TupleType{tuple_types: schema.iter()
                            .map(|col| { col.1.clone() })
                            .collect()};
                        let list_tuple_value = row_values.iter().map(|lv| {
                            Value{
                                value: Val::TupleValue{values: lv.clone()},
                                val_type: row_type.clone()
                            }
                        }).collect();

                        Value{
                            value: Val::ListValue{values: list_tuple_value},
                            val_type: ListType{list_type: Box::new(row_type)}
                        }
                    },
                    _ => error("Invalid argument type for \"schema\" in \"readCSV\"", exp)
                }
            },
            _ => error("Invalid argument type for \"path\" in \"readCSV\"", exp)
        }
    }

    fn value_to_row_entry(&self, val: &Value) -> String {
        match val.value.clone() {
            Val::IntValue{value} => value.to_string(),
            Val::BoolValue{value} => if value { "true".to_string() } else { "false".to_string() },
            Val::CharValue{value} => value,
            Val::StringValue{value} => value,
            _ => "".to_string()
        }
    }

    fn write_csv(&self, args: Vec<Value>, exp: &Exp) -> Value {
        match &args[0].value {
            Val::StringValue{value} => {
                let path = value;
                match &args[1].value {
                    Val::ListValue{values} => {
                        let table = values;
                        match &args[2].value {
                            Val::SchemaValue{..} => {
                                let mut csv_writer = match csv::Writer::from_path(path) {
                                    Ok(writer) => writer,
                                    _ => {
                                        error("Could not open CSV with given path", exp);
                                        return null_val()
                                    }
                                };
                                for row in table {
                                    match row.value.clone() {
                                        Val::TupleValue{values} => {
                                            let converted_row_values: Vec<String> = values.iter().map(|v| {
                                                self.value_to_row_entry(v)
                                            }).collect();
                                            csv_writer.write_record(&converted_row_values).expect("Could not write row to CSV");
                                        },
                                        _ => {}
                                    }
                                }

                                null_val()
                            },
                            _ => error("Invalid argument type for \"schema\" in \"writeCSV\"", exp)
                        }
                    },
                    _ => error("Invalid argument type for \"table\" in \"writeCSV\"", exp)
                }
            },
            _ => error("Invalid argument type for \"path\" in \"writeCSV\"", exp)
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

    fn fold(&self, args: Vec<Value>, exp: &Exp, interpreter: Interpreter, left: bool) -> Value {
        let mut acc = args[0].clone();
        let list = &args[1];
        let func_value = &args[2];
        match (list.val_type.clone(), func_value.val_type.clone()) {
            (ListType{list_type}, FuncType{param_types, return_type}) => {
                match *list_type {
                    CharType => { // char -> string edge-case
                        type_conforms(&acc.val_type, &StringType, &exp.token);
                        type_conforms(&param_types[0], &StringType, &exp.token);
                        type_conforms(&list_type, &param_types[1], &exp.token);
                        type_conforms(&return_type, &StringType, &exp.token)
                    },
                    _ => {
                        type_conforms(&acc.val_type, &list_type, &exp.token);
                        type_conforms(&acc.val_type, &param_types[0], &exp.token);
                        type_conforms(&list_type, &param_types[1], &exp.token);
                        type_conforms(&list_type, &return_type, &exp.token)
                    }
                };

                match list.value.clone() {
                    Val::ListValue{values} => {
                        let mut fold_func = |v: &Value| {
                            acc = match func_value.value.clone() {
                                Val::FuncValue{builtin_ident, parameters, body, env} => {
                                    let mut temp_body_env = env.clone();
                                    temp_body_env.insert(parameters[0].0.clone(), acc.clone());
                                    temp_body_env.insert(parameters[1].0.clone(), v.clone());
                                    match builtin_ident {
                                        Some(bi) => self.interpret(bi.clone(), &mut temp_body_env, &body, interpreter.clone()),
                                        _ => interpreter.clone().interpret(&body, &mut temp_body_env, &return_type)
                                    }
                                },
                                _ => error("Invalid function type for \"foldl\"", exp)
                            }
                        };
                        if left {
                            values.iter().for_each(|v: &Value| fold_func(v))
                        } else {
                            values.iter().rev().for_each(|v: &Value| fold_func(v))
                        }
                        acc
                    },
                    _ => error("Invalid list type for \"foldl\"", exp)
                }
            },
            _ => error("Invalid list or function types for \"foldl\"", exp)
        }
    }

    fn substr(&self, args: Vec<Value>, exp: &Exp) -> Value {
        let str = match &args[0].value {
            Val::StringValue{value} => value.clone(),
            _ => "".to_string()
        };
        let str_len = str.len();
        let start_index = match &args[1].value {
            Val::IntValue{value} => value.clone(),
            _ => -1
        };
        let end_index = match &args[2].value {
            Val::IntValue{value} => value.clone(),
            _ => -1
        };

        if start_index < 0 {
            error("Invalid start index for \"substr\", less than 0", exp);
        } else if start_index > end_index {
            error("Start index is greater than end index for \"substr\"", exp);
        } else if end_index > str_len as i32 {
            error("Invalid end index for \"substr\", greater than list size", exp);
        }

        let sub = str[start_index as usize..end_index as usize].to_string();
        Value{
            value: Val::StringValue{value: sub},
            val_type: StringType
        }
    }

    fn zip(&self, args: Vec<Value>, exp: &Exp) -> Value {
        let (list1, list1_type) = match &args[0] {
            Value{
                value: Val::ListValue{values},
                val_type: ListType{list_type}
            } => (Some(values), *list_type.clone()),
            _ => (None, UnknownType)
        };
        let (list2, list2_type) = match &args[1] {
            Value{
                value: Val::ListValue{values},
                val_type: ListType{list_type}
            } => (Some(values), *list_type.clone()),
            _ => (None, UnknownType)
        };
        let error_list = Value{
            value: Val::ListValue{values: vec![]},
            val_type: ListType{list_type: Box::new(NullType)}
        };

        match (list1, list2) {
            (Some(l1), Some(l2)) => {
                if l1.len() == l2.len() {
                    let tuple_type = TupleType{tuple_types: vec![list1_type.clone(), list2_type.clone()]};
                    let mut zipped_values: Vec<Value> = vec![];
                    l1.iter().zip(l2.iter()).for_each(|v: (&Value, &Value)| {
                        zipped_values.push(Value{
                            value: Val::TupleValue{values: vec![v.0.clone(), v.1.clone()]},
                            val_type: tuple_type.clone()
                        })
                    });
                    Value{
                        value: Val::ListValue{values: zipped_values},
                        val_type: ListType{list_type: Box::new(tuple_type)}
                    }
                } else {
                    error("Different sized lists provided for \"zip\"", exp);
                    error_list
                }
            },
            _ => error_list
        }
    }

    fn int_to_string(&self, args: Vec<Value>) -> Value {
        Value{
            value: Val::StringValue{value: match &args[0].value {
                Val::IntValue{value} => value.to_string(),
                _ => "".to_string()
            }},
            val_type: StringType
        }
    }

    fn string_to_int(&self, args: Vec<Value>) -> Value {
        Value{
            value: Val::IntValue{value: match &args[0].value {
                Val::StringValue{value} => value.parse::<i32>().unwrap(),
                _ => -1
            }},
            val_type: IntType
        }
    }
}