use strum_macros::Display;

use crate::utils::file_position::FilePosition;
use crate::defs::{delimiter::Delimiter, keyword::Keyword};
use crate::defs::operator::Operator;

pub fn make_empty_token() -> Token {
    Token::Ident{ident: "".to_string(), fp: FilePosition{
        line: 0,
        column: 0,
        line_text: "".to_string()
    }}
}

#[derive(Display, Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Delimiter { delim: Delimiter, fp: FilePosition },
    Keyword { keyword: Keyword, fp: FilePosition },
    Value { value: String, fp: FilePosition },
    Ident { ident: String, fp: FilePosition }
}

impl Token {
    pub fn to_operator(&self) -> Option<Operator> {
        match self.clone() {
            Token::Delimiter{delim, fp: _} => {
                match delim.to_operator() {
                    s@Some(_) => s,
                    None => None
                }
            },
            Token::Keyword{keyword, fp: _} => {
                match keyword.to_operator() {
                    s@Some(_) => s,
                    None => None
                }
            }
            _ => None
        }
    }
}

pub fn get_fp_from_token(token: &Token) -> String {
    match token {
        Token::Delimiter{delim: _, fp} => fp.position().clone(),
        Token::Keyword{keyword: _, fp} => fp.position().clone(),
        Token::Value{value: _, fp} => fp.position().clone(),
        Token::Ident{ident: _, fp} => fp.position().clone()
    }
}