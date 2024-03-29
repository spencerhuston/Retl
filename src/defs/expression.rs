use strum_macros::Display;

use crate::scanner::token::Token;
use crate::defs::retl_type::Type;
use crate::defs::operator::Operator;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Exp {
    pub exp: Expression,
    pub exp_type: Type,
    pub token: Token
}

#[derive(Display, Debug, Clone, Hash, Eq, PartialEq)]
pub enum Literal {
    IntLit{literal: i32},
    BoolLit{literal: bool},
    CharLit{literal: String},
    StringLit{literal: String},
    NullLit
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parameter {
    pub ident: String,
    pub param_type: Type,
    pub token: Token
}

#[derive(Display, Debug, Clone, Eq, PartialEq)]
pub enum Pattern {
    TypePattern{ident: String, case_type: Type, predicate: Option<Exp>},
    Literal{literal: Literal},
    MultiLiteral{literals: Vec<Literal>},
    Range{range: Expression},
    Any
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Case {
    pub pattern: Pattern,
    pub case_exp: Exp
}

#[derive(Display, Debug, Clone, Eq, PartialEq)]
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
    Iter{iter: Box<Exp>, iter_exp: Box<Exp>},
    ListDef{values: Vec<Exp>},
    TupleDef{values: Vec<Exp>},
    TupleAccess{ident: Box<Exp>, index: usize},
    DictDef{mapping: Vec<(Literal, Exp)>},
    SchemaDef{mapping: Vec<(String, Type)>},
    Empty
}