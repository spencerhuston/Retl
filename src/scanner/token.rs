use crate::utils::file_position::FilePosition;
use crate::defs::{delimiter::Delimiter, keyword::Keyword};

pub struct TokenInfo {
    pub(crate) token_text: String,
    pub(crate) fp: FilePosition
}

pub enum Token {
    Delimiter { t: TokenInfo, delim: Delimiter },
    Keyword { t: TokenInfo, keyword: Keyword },
    Value { t: TokenInfo, value: String },
    Ident { t: TokenInfo, ident: String }
}