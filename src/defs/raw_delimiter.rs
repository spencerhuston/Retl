use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
pub enum RawDelimiter {
    Colon(String),
    #[strum(serialize = ":")]
    Equal(String),
    #[strum(serialize = "=")]
    Backslash(String),
    #[strum(serialize = "\\")]
    Semicolon(String),
    #[strum(serialize = ".")]
    Period(String),
    #[strum(serialize = ",")]
    Comma(String),
    #[strum(serialize = "+")]
    Plus(String),
    #[strum(serialize = "-")]
    Hyphen(String),
    #[strum(serialize = "*")]
    Star(String),
    #[strum(serialize = "/")]
    ForwardSlash(String),
    #[strum(serialize = "%")]
    Percent(String),
    #[strum(serialize = "|")]
    Pipe(String),
    #[strum(serialize = "!")]
    Exclamation(String),
    #[strum(serialize = "<")]
    ArrowLeft(String),
    #[strum(serialize = ">")]
    ArrowRight(String),
    #[strum(serialize = "[")]
    BracketLeft(String),
    #[strum(serialize = "]")]
    BracketRight(String),
    #[strum(serialize = "{")]
    BraceLeft(String),
    #[strum(serialize = "}")]
    BraceRight(String),
    #[strum(serialize = "(")]
    ParenLeft(String),
    #[strum(serialize = ")")]
    ParenRight(String),
    #[strum(serialize = "@")]
    At(String),
    #[strum(serialize = "_")]
    Underscore
}