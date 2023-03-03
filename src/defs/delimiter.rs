use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
pub enum Delimiter {
    #[strum(serialize = ":")]
    DenoteType(String),
    #[strum(serialize = "->")]
    ReturnType(String),
    #[strum(serialize = "=")]
    Assignment(String),
    #[strum(serialize = "\\")]
    NewlineSlash(String),
    #[strum(serialize = ";")]
    StatementEnd(String),
    #[strum(serialize = "..")]
    Range(String),
    #[strum(serialize = "+")]
    Plus(String),
    #[strum(serialize = "-")]
    Minus(String),
    #[strum(serialize = "*")]
    Multiply(String),
    #[strum(serialize = "/")]
    Divide(String),
    #[strum(serialize = "%")]
    Modulus(String),
    #[strum(serialize = ">")]
    GreaterThan(String),
    #[strum(serialize = "<")]
    LessThan(String),
    #[strum(serialize = ">=")]
    GreaterThanEqualTo(String),
    #[strum(serialize = "<=")]
    LessThanEqualTo(String),
    #[strum(serialize = "==")]
    Equal(String),
    #[strum(serialize = "!=")]
    NotEqual(String),
    #[strum(serialize = "!")]
    Not(String),
    #[strum(serialize = "++")]
    ListConcat(String),
    #[strum(serialize = "|")]
    LambdaSig(String),
    #[strum(serialize = "(")]
    ParenLeft(String),
    #[strum(serialize = ")")]
    ParenRight(String),
    #[strum(serialize = "[")]
    BracketLeft(String),
    #[strum(serialize = "]")]
    BracketRight(String),
    #[strum(serialize = "{")]
    BraceLeft(String),
    #[strum(serialize = "}")]
    BraceRight(String),
    #[strum(serialize = ".")]
    TupleAccess(String),
    #[strum(serialize = ",")]
    Comma(String),
    #[strum(serialize = "@")]
    NamedPattern(String),
    #[strum(serialize = "_")]
    CatchallCase(String),
    #[strum(serialize = "=>")]
    CaseExp(String),
    #[strum(serialize = "|>")]
    Bird
}