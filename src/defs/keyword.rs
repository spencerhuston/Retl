use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
pub enum Keyword {
    #[strum(serialize = "let")]
    Let(String),
    #[strum(serialize = "int")]
    Int(String),
    #[strum(serialize = "bool")]
    Bool(String),
    #[strum(serialize = "char")]
    Char(String),
    #[strum(serialize = "string")]
    String(String),
    #[strum(serialize = "list")]
    List(String),
    #[strum(serialize = "dict")]
    Dict(String),
    #[strum(serialize = "tuple")]
    Tuple(String),
    #[strum(serialize = "schema")]
    Schema(String),
    #[strum(serialize = "true")]
    True(String),
    #[strum(serialize = "false")]
    False(String),
    #[strum(serialize = "and")]
    And(String),
    #[strum(serialize = "or")]
    Or(String),
    #[strum(serialize = "parallel")]
    Parallel(String),
    #[strum(serialize = "if")]
    If(String),
    #[strum(serialize = "else")]
    Else(String),
    #[strum(serialize = "match")]
    Match(String),
    #[strum(serialize = "case")]
    Case(String),
    #[strum(serialize = "readln")]
    Readln(String),
    #[strum(serialize = "readCSV")]
    ReadCSV(String),
    #[strum(serialize = "writeCSV")]
    WriteCSV(String),
    #[strum(serialize = "print")]
    Print(String),
    #[strum(serialize = "println")]
    Println(String),
    #[strum(serialize = "map")]
    Map(String),
    #[strum(serialize = "filter")]
    Filter(String),
    #[strum(serialize = "zip")]
    Zip(String),
    #[strum(serialize = "foldl")]
    Foldl(String),
    #[strum(serialize = "foldr")]
    Foldr(String),
    #[strum(serialize = "foreach")]
    Foreach(String),
    #[strum(serialize = "range")]
    Range(String)
}