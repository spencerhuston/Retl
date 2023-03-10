use strum_macros::{EnumString, Display};
use crate::defs::operator::Operator;

#[derive(Display, Debug, PartialEq, Eq, EnumString, Clone, Hash)]
pub enum Delimiter {
    #[strum(serialize = ":")]
    DenoteType,
    #[strum(serialize = "->")]
    ReturnType,
    #[strum(serialize = "=")]
    Assignment,
    #[strum(serialize = "\\")]
    NewlineSlash,
    #[strum(serialize = ";")]
    StatementEnd,
    #[strum(serialize = "..")]
    Range,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "*")]
    Multiply,
    #[strum(serialize = "/")]
    Divide,
    #[strum(serialize = "%")]
    Modulus,
    #[strum(serialize = ">")]
    GreaterThan,
    #[strum(serialize = "<")]
    LessThan,
    #[strum(serialize = ">=")]
    GreaterThanEqualTo,
    #[strum(serialize = "<=")]
    LessThanEqualTo,
    #[strum(serialize = "==")]
    Equal,
    #[strum(serialize = "!")]
    Not,
    #[strum(serialize = "++")]
    ListConcat,
    #[strum(serialize = "|")]
    LambdaSig,
    #[strum(serialize = "(")]
    ParenLeft,
    #[strum(serialize = ")")]
    ParenRight,
    #[strum(serialize = "[")]
    BracketLeft,
    #[strum(serialize = "]")]
    BracketRight,
    #[strum(serialize = "{")]
    BraceLeft,
    #[strum(serialize = "}")]
    BraceRight,
    #[strum(serialize = ".")]
    TupleAccess,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = "_")]
    CatchallCase,
    #[strum(serialize = "=>")]
    CaseExp,
    #[strum(serialize = "|>")]
    Bird,
    #[strum(serialize = "$")]
    SchemaStart
}

impl Delimiter {
    pub fn to_operator(&self) -> Option<Operator> {
        match *self {
            Delimiter::Plus => Some(Operator::Plus),
            Delimiter::Minus => Some(Operator::Minus),
            Delimiter::Multiply => Some(Operator::Multiply),
            Delimiter::Divide => Some(Operator::Divide),
            Delimiter::Modulus => Some(Operator::Modulus),
            Delimiter::GreaterThan => Some(Operator::GreaterThan),
            Delimiter::LessThan => Some(Operator::LessThan),
            Delimiter::GreaterThanEqualTo => Some(Operator::GreaterThanEqualTo),
            Delimiter::LessThanEqualTo => Some(Operator::LessThanEqualTo),
            Delimiter::Equal => Some(Operator::Equal),
            Delimiter::ListConcat => Some(Operator::CollectionConcat),
            _ => None
        }
    }
}