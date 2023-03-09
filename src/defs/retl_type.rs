use strum_macros::Display;

#[derive(Display, Debug)]
pub enum Type {
    IntType,
    BoolType,
    CharType,
    StringType,
    NullType,
    ListType{listType: Box<Type>},
    TupleType{tupleTypes: Vec<Type>},
    DictType{keyType: Box<Type>, valueType: Box<Type>},
    SchemaType,
    FuncType{paramTypes: Vec<Type>, returnType: Box<Type>}
}