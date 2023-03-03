use std::borrow::Borrow;
use std::str::FromStr;
use std::fs::File;
use regex::Regex;

use crate::defs::keyword;
use crate::defs::delimiter;
use crate::defs::raw_delimiter;
use crate::utils::file_position::FilePosition;
use crate::scanner::token::{Token, TokenInfo};

pub struct Scanner {
    pub(crate) tokens: Vec<Token>
}

fn is_raw_delim(c: &char) -> bool {
    if *c == '\"' || *c == '\'' {
        false
    } else {
        match raw_delimiter::RawDelimiter::from_str(&c.to_string()) {
            _RawDelimiter => true,
            _ => false
        }
    }
}

fn is_keyword(token: String) -> bool {
    match keyword::Keyword::from_str(&token) {
        _Keyword => true,
        _ => false
    }
}

fn is_value(token: String) -> bool {
    if token == "true" || token == "false" {
        true
    } else if token.trim().parse::<i64>().is_ok() {
        true
    } else if token.starts_with('\"') && token.ends_with('\"') {
        true
    } else if token.starts_with('\'') && token.ends_with('\'') {
        true
    } else {
        false
    }
}

fn is_ident(token: String) -> bool {
    let ident_regex = Regex::new(r"^[a-zA-z_][a-zA-z\d_]*$").unwrap();
    ident_regex.is_match(token.borrow())
}

fn update_file_pos(c: char, file_pos: &mut FilePosition) {
    match c {
        '\n' => {
            file_pos.column = 0;
            file_pos.line += 1;
            file_pos.line_text.clear()
        },
        ' ' => file_pos.column += 1,
        '\t' => file_pos.column += "\t".len() as i32,
        _ => ()
    }
}

fn push_char(token_text: &mut String, c: char, file_pos: &mut FilePosition) {
    file_pos.column += 1;
    token_text.push(c);
}

impl Scanner {
    fn add_tokens(&mut self, token_text: String, file_pos: FilePosition) -> Option<()>{
        println!("{}", token_text);
        let make_token_info = |token_text: &String, file_pos: &FilePosition| {
            TokenInfo { token_text: token_text.clone(), fp: file_pos.clone() }
        };

        let mut token = String::from("");
        let text: Vec<char> = token_text.chars().collect();
        for chars in text.windows(2) {
            if !token.is_empty() {
                let text_str: String = token.clone();
                if is_keyword(text_str.clone()) {

                } else if is_value(text_str.clone()) {

                } else if is_ident(text_str.clone()) {

                } else {
                    () // THROW ERROR HERE, UNKNOWN TOKEN
                }
            }

            // add delim tokens here
        }
        None
    }

    pub fn scan(&mut self, script: &String) {
        println!("{}", script);
        let mut token_text: String = String::from("");
        let mut inside_quotes = false;
        let mut file_pos = FilePosition { line: 0, column: 0, line_text: String::from("") };

        for c in script.chars() {
            if c == '\"' || c == '\'' {
                inside_quotes = !inside_quotes;
                push_char(&mut token_text, c, &mut file_pos);
            } else if c.is_whitespace() && inside_quotes {
                push_char(&mut token_text, c, &mut file_pos);
            } else if !c.is_whitespace() {
                push_char(&mut token_text, c, &mut file_pos);
            } else if !token_text.is_empty() {
                update_file_pos(c, &mut file_pos);
                self.add_tokens(token_text.clone(), file_pos.clone());
                token_text.clear();
            }
        }

        if !token_text.is_empty() {
            self.add_tokens(token_text.clone(), file_pos.clone());
        }
    }
}