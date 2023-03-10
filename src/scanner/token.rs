use strum_macros::Display;

use crate::utils::file_position::FilePosition;
use crate::defs::{delimiter::Delimiter, keyword::Keyword};

#[derive(Display, Debug, Clone)]
pub enum Token {
    Delimiter { delim: Delimiter, fp: FilePosition },
    Keyword { keyword: Keyword, fp: FilePosition },
    Value { value: String, fp: FilePosition },
    Ident { ident: String, fp: FilePosition }
}