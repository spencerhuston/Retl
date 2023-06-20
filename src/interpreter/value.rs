use std::collections::HashMap;
use strum_macros::Display;

use crate::defs::expression::Exp;
use crate::defs::keyword::Keyword;
use crate::defs::retl_type::Type;

pub type Env = HashMap<String, Value>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Value {
    pub value: Val,
    pub val_type: Type
}

#[derive(Display, Debug, Clone, Eq, PartialEq)]
pub enum Val {
    IntValue{value: i32},
    BoolValue{value: bool},
    CharValue{value: String},
    StringValue{value: String},
    NullValue,
    ListValue{values: Vec<Value>},
    TupleValue{values: Vec<Value>},
    DictValue{values: Vec<(Value, Value)>},
    SchemaValue{values: Vec<(String, Type)>},
    TableValue{schema: Box<Value>, rows: Vec<Value>}, // Schema should be SchemaValue, rows should be List of Tuple
    FuncValue{
        builtin_ident: Option<Keyword>,
        parameters: Vec<(String, Type)>,
        body: Exp,
        env: Env
    },
    Error
}