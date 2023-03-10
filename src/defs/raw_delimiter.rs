use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
pub enum RawDelimiter {
    #[strum(serialize = ":")]
    Colon,
    #[strum(serialize = "=")]
    Equal,
    #[strum(serialize = "\\")]
    Backslash,
    #[strum(serialize = ";")]
    Semicolon,
    #[strum(serialize = ".")]
    Period,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Hyphen,
    #[strum(serialize = "*")]
    Star,
    #[strum(serialize = "/")]
    ForwardSlash,
    #[strum(serialize = "%")]
    Percent,
    #[strum(serialize = "|")]
    Pipe,
    #[strum(serialize = "!")]
    Exclamation,
    #[strum(serialize = "<")]
    ArrowLeft,
    #[strum(serialize = ">")]
    ArrowRight,
    #[strum(serialize = "[")]
    BracketLeft,
    #[strum(serialize = "]")]
    BracketRight,
    #[strum(serialize = "{")]
    BraceLeft,
    #[strum(serialize = "}")]
    BraceRight,
    #[strum(serialize = "(")]
    ParenLeft,
    #[strum(serialize = ")")]
    ParenRight,
    #[strum(serialize = "@")]
    At,
    #[strum(serialize = "$")]
    DollarSign,
}