use strum_macros::{EnumString, Display};
use crate::defs::operator::Operator;

#[derive(Display, Debug, PartialEq, Eq, EnumString, Clone)]
pub enum Keyword {
    #[strum(serialize = "let")]
    Let,
    #[strum(serialize = "alias")]
    Alias,
    #[strum(serialize = "int")]
    Int,
    #[strum(serialize = "bool")]
    Bool,
    #[strum(serialize = "char")]
    Char,
    #[strum(serialize = "string")]
    String,
    #[strum(serialize = "null")]
    Null,
    #[strum(serialize = "list")]
    List,
    #[strum(serialize = "dict")]
    Dict,
    #[strum(serialize = "tuple")]
    Tuple,
    #[strum(serialize = "schema")]
    Schema,
    #[strum(serialize = "true")]
    True,
    #[strum(serialize = "false")]
    False,
    #[strum(serialize = "and")]
    And,
    #[strum(serialize = "or")]
    Or,
    #[strum(serialize = "not")]
    Not,
    #[strum(serialize = "parallel")]
    Parallel,
    #[strum(serialize = "if")]
    If,
    #[strum(serialize = "else")]
    Else,
    #[strum(serialize = "match")]
    Match,
    #[strum(serialize = "case")]
    Case,
    #[strum(serialize = "readln")]
    Readln,
    #[strum(serialize = "readCSV")]
    ReadCSV,
    #[strum(serialize = "writeCSV")]
    WriteCSV,
    #[strum(serialize = "print")]
    Print,
    #[strum(serialize = "println")]
    Println,
    #[strum(serialize = "map")]
    Map,
    #[strum(serialize = "filter")]
    Filter,
    #[strum(serialize = "zip")]
    Zip,
    #[strum(serialize = "foldl")]
    Foldl,
    #[strum(serialize = "foldr")]
    Foldr,
    #[strum(serialize = "foreach")]
    Foreach,
    #[strum(serialize = "range")]
    Range
}

impl Keyword {
    pub fn to_operator(&self) -> Option<Operator> {
        match *self {
            Keyword::Not => Some(Operator::Not),
            Keyword::And => Some(Operator::And),
            Keyword::Or => Some(Operator::Or),
            _ => None
        }
    }

    pub fn is_builtin_function(&self) -> bool {
        match *self {
            Keyword::Readln |
            Keyword::ReadCSV |
            Keyword::WriteCSV |
            Keyword::Print |
            Keyword::Println |
            Keyword::Map |
            Keyword::Filter |
            Keyword::Zip |
            Keyword::Foldl |
            Keyword::Foldr |
            Keyword::Foreach |
            Keyword::Range
            => true,
            _ => false
        }
    }
}