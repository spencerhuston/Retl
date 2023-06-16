use log::{error, trace};
use strum_macros::Display;
use crate::scanner::token::{Token, get_fp_from_token};

#[derive(Display, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    IntType,
    BoolType,
    CharType,
    StringType,
    NullType,
    UnionType{union_types: Vec<Type>},
    ListType{list_type: Box<Type>},
    TupleType{tuple_types: Vec<Type>},
    DictType{key_type: Box<Type>, value_type: Box<Type>},
    SchemaType,
    FuncType{param_types: Vec<Type>, return_type: Box<Type>},
    UnknownType,
    Any
}

fn well_formed(t: &Type) -> Type {
    match t {
        Type::UnionType{union_types} => {
            let mut uts: Vec<Type> = vec![];
            for ut in union_types.iter() {
                uts.push(well_formed(&ut))
            }
            Type::UnionType{union_types: uts}
        },
        Type::ListType{list_type} => Type::ListType{
            list_type: Box::new(well_formed(&**list_type))
        },
        Type::TupleType{tuple_types} => {
            let mut tts: Vec<Type> = vec![];
            for tt in tuple_types.iter() {
                tts.push(well_formed(&tt))
            }
            Type::TupleType{tuple_types: tts}
        },
        Type::DictType{key_type, value_type} => {
            Type::DictType {
                key_type: Box::new(well_formed(&**key_type)),
                value_type: Box::new(well_formed(&**value_type))
            }
        },
        Type::FuncType{param_types, return_type} => {
            let mut pts: Vec<Type> = vec![];
            for pt in param_types.iter() {
                pts.push(well_formed(&pt))
            }
            Type::FuncType{
                param_types: pts,
                return_type: Box::new(well_formed(&**return_type))
            }
        },
        Type::UnknownType => Type::UnknownType,
        _ => t.clone()
    }
}

fn _type_conforms(t1: &Type, t2: &Type, token: &Token) -> Type {
    trace!("t1: {:?}, t2: {:?}, token: {:?}", t1, t2, token);
    match (t1, t2) {
        (_, _) if t1 == t2 => well_formed(t1),
        (Type::Any, t) => well_formed(t),
        (t, Type::Any) => well_formed(t),
        (Type::UnionType{union_types: uts1}, Type::UnionType{union_types: uts2})
        if !uts1.is_empty() && !uts2.is_empty() && uts1.len() == uts2.len() => {
            let mut uts: Vec<Type> = vec![];
            for (ut1, ut2) in uts1.iter().zip(uts2) {
                uts.push(_type_conforms(ut1, ut2, token))
            }
            Type::UnionType{union_types: uts}
        },
        (Type::UnionType{union_types: uts}, _) => {
            match uts.iter().find(|ut| -> bool {
                match _type_conforms(ut, t2, token) {
                    Type::UnknownType => false,
                    t@_ => !has_unknown_types(&t)
                }
            }) {
                Some(t) => t.clone(),
                _ => Type::UnknownType
            }
        },
        (_, Type::UnionType{union_types: uts}) => {
            match uts.iter().find(|ut| -> bool {
                match _type_conforms(ut, t1, token) {
                    Type::UnknownType => false,
                    t@_ => !has_unknown_types(&t)
                }
            }) {
                Some(t) => t.clone(),
                _ => Type::UnknownType
            }
        },
        (Type::ListType{list_type: l1}, Type::ListType{list_type: l2}) => {
            Type::ListType{list_type: Box::new(_type_conforms(&**l1, &**l2, token))}
        },
        (Type::TupleType{tuple_types: tts1}, Type::TupleType{tuple_types: tts2})
        if !tts1.is_empty() && !tts2.is_empty() && tts1.len() == tts2.len() => {
            let mut tts: Vec<Type> = vec![];
            for (tt1, tt2) in tts1.iter().zip(tts2) {
                tts.push(_type_conforms(tt1, tt2, token))
            }
            Type::TupleType{tuple_types: tts}
        },
        (Type::DictType{key_type: k1, value_type: v1},
            Type::DictType{key_type: k2, value_type: v2}) => {
            Type::DictType{
                key_type: Box::new(_type_conforms(&**k1, &**k2, token)),
                value_type: Box::new(_type_conforms(&**v1, &**v2, token))
            }
        },
        (Type::FuncType{param_types: pts1, return_type: r1},
            Type::FuncType{param_types: pts2, return_type: r2})
        if !pts1.is_empty() && !pts2.is_empty() && pts1.len() == pts2.len() => {
            let mut pts: Vec<Type> = vec![];
            for (pt1, pt2) in pts1.iter().zip(pts2) {
                pts.push(_type_conforms(pt1, pt2, token))
            }
            Type::FuncType{
                param_types: pts,
                return_type: Box::new(_type_conforms(r1, r2, token))
            }
        },
        (_, Type::UnknownType) => well_formed(t1),
        (Type::UnknownType, _) => well_formed(t2),
        _ => Type::UnknownType
    }
}

fn type_is_unknown(t: &Type) -> bool {
    match t {
        Type::UnknownType => true,
        _ => false
    }
}

fn has_unknown_types(t: &Type) -> bool {
    match t {
        Type::UnionType{union_types} => {
            match union_types.iter().find(|ut| { type_is_unknown(ut) }) {
                Some(_) => true,
                _ => false
            }
        },
        Type::ListType{list_type} => {
            type_is_unknown(list_type)
        },
        Type::TupleType{tuple_types} => {
            match tuple_types.iter().find(|tt| { type_is_unknown(tt) }) {
                Some(_) => true,
                _ => false
            }
        },
        Type::DictType{key_type, value_type} => {
            type_is_unknown(key_type) || type_is_unknown(value_type)
        },
        Type::FuncType{param_types, return_type} => {
            match param_types.iter().find(|pt| { type_is_unknown(pt) }) {
                Some(_) => true,
                _ => type_is_unknown(return_type)
            }
        },
        Type::UnknownType => true,
        _ => false
    }
}

pub fn type_conforms(t1: &Type, t2: &Type, token: &Token) -> Type {
    let resolved_type = _type_conforms(t1, t2, token);
    if has_unknown_types(&resolved_type) {
        error!("Type mismatch, {:?} vs. {:?}: {}",
            t1.as_string(),
            t2.as_string(),
            get_fp_from_token(&token));
        Type::UnknownType
    } else {
        resolved_type
    }
}

pub fn type_conforms_no_error(t1: &Type, t2: &Type, token: &Token) -> Type {
    _type_conforms(t1, t2, token)
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
            Type::UnionType{union_types} => {
                "union[".to_owned() + &*type_list_as_string(&union_types) + "]"
            },
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