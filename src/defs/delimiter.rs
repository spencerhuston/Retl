pub enum Delimiter {
    DenoteType,
    ReturnType,
    Assignment,
    NewlineSlash,
    StatementEnd,
    Range,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    And,
    Or,
    GreaterThan,
    LessThan,
    GreaterThanEqualTo,
    LessThanEqualTo,
    Equal,
    NotEqual,
    Not,
    ListConcat,
    CollectionEqual,
    ParenLeft,
    ParenRight,
    BracketLeft,
    BracketRight,
    BraceLeft,
    BraceRight,
    TupleAccess,
    Comma,
    NamedPattern,
    MultiPattern,
    CatchallCase,
    CaseExp,
    Bird
}

impl Delimiter {
    fn val(&self) -> &'static str {
        match self {
            Delimiter::DenoteType => ":",
            Delimiter::ReturnType => "->",
            Delimiter::Assignment => "=",
            Delimiter::NewlineSlash => "\\",
            Delimiter::StatementEnd => ";",
            Delimiter::Range => "..",
            Delimiter::Plus => "+",
            Delimiter::Minus => "-",
            Delimiter::Multiply => "*",
            Delimiter::Divide => "/",
            Delimiter::Modulus => "%",
            Delimiter::And => "&&",
            Delimiter::Or => "||",
            Delimiter::GreaterThan => ">",
            Delimiter::LessThan => "<",
            Delimiter::GreaterThanEqualTo => ">=",
            Delimiter::LessThanEqualTo => "<=",
            Delimiter::Equal => "==",
            Delimiter::NotEqual => "!=",
            Delimiter::Not => "!",
            Delimiter::ListConcat => "++",
            Delimiter::CollectionEqual => "===",
            Delimiter::BracketLeft => "[",
            Delimiter::BracketRight => "]",
            Delimiter::BraceLeft => "{",
            Delimiter::BraceRight => "}",
            Delimiter::ParenLeft => "(",
            Delimiter::ParenRight => ")",
            Delimiter::TupleAccess => ".",
            Delimiter::Comma => ",",
            Delimiter::NamedPattern => "@",
            Delimiter::MultiPattern => "|",
            Delimiter::CatchallCase => "_",
            Delimiter::CaseExp => "=>",
            Delimiter::Bird => "|>"
        }
    }
}