use std::collections::HashMap;
use crate::defs::expression::Exp;

type Env = HashMap<String, Value>;

#[derive(Display, Debug, Clone, Eq, PartialEq)]
pub enum Value {
    IntValue{value: i32},
    BoolValue{value: bool},
    CharValue{value: char},
    StringValue{value: String},
    NullValue,
    ListValue{values: Vec<Value>},
    TupleValue{values: Vec<Value>},
    DictValue, // TODO: Figure out correct structure
    SchemaValue, // TODO: Figure out correct structure
    FuncValue{
        builtin: bool,
        parameters: Vec<String>,
        body: Exp,
        env: Env
    }
}