use log::{error, trace};
use strum_macros::Display;
use crate::scanner::token::{Token, get_fp_from_token};
use crate::Value;

#[derive(Display, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    IntType,
    BoolType,
    CharType,
    StringType,
    NullType,
    ListType{list_type: Box<Type>},
    TupleType{tuple_types: Vec<Type>},
    DictType{key_type: Box<Type>, value_type: Box<Type>},
    SchemaType,
    FuncType{param_types: Vec<Type>, return_type: Box<Type>},
    UnknownType,
    Any
}

fn well_formed(t: &Type, token: &Token) -> Type {
    match t {
        Type::ListType{list_type} => Type::ListType{
            list_type: Box::new(well_formed(&**list_type, token))
        },
        Type::TupleType{tuple_types} => {
            let mut tts: Vec<Type> = vec![];
            for tt in tuple_types.iter() {
                tts.push(well_formed(&tt, token))
            }
            Type::TupleType{tuple_types: tts}
        },
        Type::DictType{key_type, value_type} => {
            Type::DictType {
                key_type: Box::new(well_formed(&**key_type, token)),
                value_type: Box::new(well_formed(&**value_type, token))
            }
        },
        Type::FuncType{param_types, return_type} => {
            let mut pts: Vec<Type> = vec![];
            for pt in param_types.iter() {
                pts.push(well_formed(&pt, token))
            }
            Type::FuncType{
                param_types: pts,
                return_type: Box::new(well_formed(&**return_type, token))
            }
        },
        Type::UnknownType => Type::UnknownType,
        _ => t.clone()
    }
}

fn _type_conforms(t1: &Type, t2: &Type, token: &Token) -> Type {
    trace!("t1: {:?}, t2: {:?}, token: {:?}", t1, t2, token);
    match (t1, t2) {
        (_, _) if t1 == t2 => well_formed(t1, token),
        (Type::ListType{list_type: l1}, Type::ListType{list_type: l2}) => {
            Type::ListType{list_type: Box::new(type_conforms(&**l1, &**l2, token))}
        },
        (Type::TupleType{tuple_types: tts1}, Type::TupleType{tuple_types: tts2})
        if !tts1.is_empty() && !tts2.is_empty() && tts1.len() == tts2.len() => {
            let mut tts: Vec<Type> = vec![];
            for (tt1, tt2) in tts1.iter().zip(tts2) {
                tts.push(type_conforms(tt1, tt2, token))
            }
            Type::TupleType{tuple_types: tts}
        },
        (Type::DictType{key_type: k1, value_type: v1},
            Type::DictType{key_type: k2, value_type: v2}) => {
            Type::DictType{
                key_type: Box::new(type_conforms(&**k1, &**k2, token)),
                value_type: Box::new(type_conforms(&**v1, &**v2, token))
            }
        },
        (Type::FuncType{param_types: pts1, return_type: r1},
            Type::FuncType{param_types: pts2, return_type: r2})
        if !pts1.is_empty() && !pts2.is_empty() && pts1.len() == pts2.len() => {
            let mut pts: Vec<Type> = vec![];
            for (pt1, pt2) in pts1.iter().zip(pts2) {
                pts.push(type_conforms(pt1, pt2, token))
            }
            Type::FuncType{
                param_types: pts,
                return_type: Box::new(type_conforms(r1, r2, token))
            }
        },
        (_, Type::UnknownType) => well_formed(t1, token),
        (Type::UnknownType, _) => well_formed(t2, token),
        (Type::Any, t) => well_formed(t, token),
        (t, Type::Any) => well_formed(t, token),
        _ => Type::UnknownType
    }
}

pub fn type_conforms(t1: &Type, t2: &Type, token: &Token) -> Type {
    match _type_conforms(t1, t2, token) {
        ut@Type::UnknownType => {
            error!("Type mismatch, {:?} vs. {:?}: {}",
                t1.as_string(),
                t2.as_string(),
                get_fp_from_token(&token));
            ut
        },
        t@_ => t
    }
}

pub fn type_conforms_no_error(t1: &Type, t2: &Type, token: &Token) -> Type {
    match _type_conforms(t1, t2, token) {
        ut@Type::UnknownType => ut,
        t@_ => t
    }
}

fn type_list_as_string(ts: &Vec<Type>) -> String {
    let mut type_str = String::from("");
    for i in 0..ts.len() - 1 {
        type_str.push_str(&*(ts[i].as_string() + ","))
    }
    type_str.push_str(&*ts.last().unwrap().as_string());
    type_str
}

impl Type {
    pub fn as_string(&self) -> String {
        match self {
            Type::IntType => String::from("int"),
            Type::BoolType => String::from("bool"),
            Type::CharType => String::from("char"),
            Type::StringType => String::from("string"),
            Type::NullType => String::from("null"),
            Type::ListType{list_type } => {
                "list[".to_owned() + &list_type.clone().as_string() + "]"
            },
            Type::TupleType{tuple_types} => {
                "tuple[".to_owned() + &*type_list_as_string(&tuple_types) + "]"
            },
            Type::DictType{key_type, value_type} => {
                "dict[".to_owned() + &key_type.clone().as_string() + ": " + &value_type.clone().as_string() + "]"
            },
            Type::SchemaType => String::from("schema"),
            Type::FuncType{param_types, return_type} => {
                "lambda[".to_owned() + &*type_list_as_string(param_types) +
                    "->" + &return_type.clone().as_string() + "]"
            },
            Type::UnknownType => String::from("unknown"),
            Type::Any => String::from("any")
        }
    }
}