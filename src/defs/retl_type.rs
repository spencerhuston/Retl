use strum_macros::Display;

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
    UnknownType
}

fn well_formed(t: &Type) -> Type {
    match t {
        Type::ListType{list_type} => well_formed(&**list_type),
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
            Type::FuncType{param_types: pts, return_type: Box::new(well_formed(&**return_type))}
        },
        Type::UnknownType => {
            // TODO: Throw error here, unresolved type
            println!("Error, unresolved type!");
            Type::UnknownType
        },
        _ => t.clone()
    }
}

// TODO: NEED FILE POSITION FOR ERROR
pub fn type_conforms(t1: &Type, t2: &Type) -> Type {
    match (t1, t2) {
        (_, _) if t1 == t2 => well_formed(t1),
        (Type::ListType{list_type: l1}, Type::ListType{list_type: l2}) => {
            Type::ListType{list_type: Box::new(type_conforms(&**l1, &**l2))}
        },
        (Type::TupleType{tuple_types: l1}, Type::TupleType{tuple_types: l2})
        if !l1.is_empty() && !l2.is_empty() && l1.len() == l2.len() => {
            // TODO
            Type::UnknownType
        },
        (Type::DictType{key_type: k1, value_type: v1},
            Type::DictType{key_type: k2, value_type: v2}) => {
            Type::DictType{
                key_type: Box::new(type_conforms(&**k1, &**k2)),
                value_type: Box::new(type_conforms(&**v1, &**v2))
            }
        },
        (Type::FuncType{param_types: p1, return_type: r1},
            Type::FuncType{param_types: p2, return_type: r2})
        if !p1.is_empty() && !p2.is_empty() && p1.len() == p2.len() => {
            // TODO
            Type::UnknownType
        },
        (_, Type::UnknownType) => well_formed(t1),
        (Type::UnknownType, _) => well_formed(t2),
        _ => {
            // TODO: Throw error here, type mismatch
            println!("Error, type mismatch!");
            Type::UnknownType
        }
    }
}