use strum_macros::Display;
use std::collections::HashMap;

use crate::scanner::token::Token;
use crate::defs::retl_type::Type;
use crate::defs::operator::Operator;

#[derive(Debug)]
pub struct Exp {
    pub exp: Expression,
    pub exp_type: Type,
    pub token: Token
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
    case_exp: Exp
}

#[derive(Display, Debug)]
pub enum Expression {
    Lit{lit: Literal},
    Let{ident: String, let_type: Type, let_exp: Box<Exp>, after_let_exp: Box<Option<Exp>>},
    Alias{ident: String, alias: Type, after_alias_exp: Box<Option<Exp>>},
    Lambda{params: Vec<Parameter>, return_type: Type, body: Box<Exp>},
    Application{ident: Box<Exp>, args: Vec<Exp>},
    Match{match_exp: Box<Exp>, cases: Vec<Case>},
    Primitive{operator: Operator, left: Box<Exp>, right: Box<Exp>},
    Reference{ident: String},
    Branch{condition: Box<Exp>, if_branch: Box<Exp>, else_branch: Box<Option<Exp>>},
    ListDef{values: Vec<Exp>},
    TupleDef{values: Vec<Exp>},
    TupleAccess{ident: String, index: Literal},
    DictDef{mapping: HashMap<Exp, Exp>},
    SchemaDef{mapping: HashMap<String, Type>},
    Empty
}