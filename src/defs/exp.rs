use strum_macros::Display;
use std::collections::HashMap;

use crate::scanner::token::Token;
use crate::defs::retl_type::Type;
use crate::defs::operator::Operator;

#[derive(Debug)]
pub struct ExpMeta {
    exp_type: Type,
    token: Token
}

#[derive(Display, Debug)]
pub enum Literal {
    IntLit{literal: i32},
    BoolLit{literal: bool},
    CharLit{literal: char},
    StringLit{literal: String},
    NullLit
}

#[derive(Debug)]
pub struct Parameter {
    ident: String, 
    param_type: Option<Type>,
    token: Token
}

#[derive(Display, Debug)]
pub enum Pattern {
    TypePattern{ident: String, case_type: Type, predicate: Option<Exp>},
    Literal{literal: Literal},
    MultiLiteral{literals: Vec<Literal>},
    Range{range: Exp},
    Any
}

#[derive(Debug)]
pub struct Case {
    pattern: Pattern,
    case_exp: Exp,
    meta: ExpMeta
}

#[derive(Display, Debug)]
pub enum Exp {
    Lit{lit: Literal, meta: ExpMeta},
    Let{ident: String, let_type: Option<Type>, let_exp: Box<Exp>, after_exp: Option<Box<Exp>>, meta: ExpMeta},
    Alias{ident: String, alias: Type, after_alias: Option<Box<Exp>>, meta: ExpMeta},
    Lambda{params: Vec<Parameter>, return_type: Type, body: Box<Exp>, meta: ExpMeta},
    Application{ident: Box<Exp>, args: Vec<Exp>, meta: ExpMeta},
    Match{match_exp: Box<Exp>, cases: Vec<Case>, meta: ExpMeta},
    Primitive{operator: Operator, left: Box<Exp>, right: Box<Exp>, meta: ExpMeta},
    Reference{ident: String},
    Branch{condition: Box<Exp>, if_branch: Box<Exp>, else_branch: Box<Exp>},
    ListDef{values: Vec<Exp>},
    TupleDef{values: Vec<Exp>},
    TupleAccess{ident: String, index: Literal},
    DictDef{mapping: HashMap<Exp, Exp>},
    SchemaDef{mapping: HashMap<String, Type>}
}