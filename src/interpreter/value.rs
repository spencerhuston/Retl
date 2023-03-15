use std::collections::HashMap;
use strum_macros::Display;

use crate::defs::expression::Exp;
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
    DictValue{values: (Box<Value>, Box<Value>)},
    SchemaValue{values: HashMap<String, Type>},
    FuncValue{
        builtin: bool,
        parameters: Vec<String>,
        body: Exp,
        env: Env
    },
    Error
}