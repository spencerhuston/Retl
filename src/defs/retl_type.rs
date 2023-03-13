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