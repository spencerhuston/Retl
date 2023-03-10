use strum_macros::{EnumString, Display};

#[derive(Display, Debug, PartialEq, EnumString, Clone)]
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