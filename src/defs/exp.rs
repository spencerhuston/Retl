use strum_macros::Display;
use std::collections::HashMap;

use crate::scanner::token::Token;
use crate::defs::retl_type::Type;
use crate::defs::operator::Operator;

#[derive(Debug)]
pub struct ExpMeta {
    expType: Type,
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
    paramType: Option<Type>,
    token: Token
}

#[derive(Display, Debug)]
pub enum Pattern {
    TypePattern{ident: String, caseType: Type, predicate: Option<Exp>},
    Literal{literal: Literal},
    MultiLiteral{literals: Vec<Literal>},
    Range{range: Exp},
    Any
}

#[derive(Debug)]
pub struct Case {
    pattern: Pattern,
    caseExp: Exp,
    meta: ExpMeta
}

#[derive(Display, Debug)]
pub enum Exp {
    Lit{lit: Literal, meta: ExpMeta},
    Let{ident: String, letType: Option<Type>, letExp: Box<Exp>, afterExp: Option<Box<Exp>>, meta: ExpMeta},
    Alias{ident: String, alias: Type, afterAlias: Option<Box<Exp>>, meta: ExpMeta},
    Lambda{params: Vec<Parameter>, returnType: Type, body: Box<Exp>, meta: ExpMeta},
    Application{ident: Box<Exp>, args: Vec<Exp>, meta: ExpMeta},
    Match{matchExp: Box<Exp>, cases: Vec<Case>, meta: ExpMeta},
    Primitive{operator: Operator, left: Box<Exp>, right: Box<Exp>, meta: ExpMeta},
    Reference{ident: String},
    Branch{condition: Box<Exp>, ifBranch: Box<Exp>, elseBranch: Box<Exp>},
    ListDef{values: Vec<Exp>},
    TupleDef{values: Vec<Exp>},
    TupleAccess{ident: String, index: Literal},
    DictDef{mapping: HashMap<Exp, Exp>},
    SchemaDef{mapping: HashMap<String, Type>}
}