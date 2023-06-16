use std::borrow::Borrow;
use std::str::FromStr;
use regex::Regex;
use either::*;
use substring::Substring;
use log::{debug, error};

use crate::defs::keyword;
use crate::defs::delimiter;
use crate::defs::raw_delimiter;
use crate::utils::file_position::FilePosition;
use crate::scanner::token::Token;

pub struct Scanner {
    pub error: bool,
    pub tokens: Vec<Token>
}

fn is_valid_character(c: &char, inside_quotes: &bool) -> bool {
    *inside_quotes || c.is_alphanumeric() || c.is_whitespace() || *c == '#' || *c == '_' || *c == '\'' || *c == '\"' || is_raw_delim(c, inside_quotes.borrow())
}

fn is_raw_delim(c: &char, inside_quotes: &bool) -> bool {
    match raw_delimiter::RawDelimiter::from_str(&c.to_string()) {
        Ok(_) if !inside_quotes => true,
        _ => false
    }
}

fn is_delim(d: &String) -> bool {
    match delimiter::Delimiter::from_str(d) {
        Ok(_) => true,
        _ => false
    }
}

fn is_keyword(token: &String) -> bool {
    match keyword::Keyword::from_str(&token) {
        Ok(_) => true,
        _ => false
    }
}

fn is_value(token: &String) -> bool {
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

fn is_ident(token: &String) -> bool {
    let ident_regex = Regex::new(r"^[a-zA-z_][a-zA-z\d_]*$").unwrap();
    ident_regex.is_match(token.borrow())
}

fn is_whitespace(c: char, inside_quotes: bool) -> bool {
    match c {
        '\n' | '\r' | '\t' => true,
        ' ' => if inside_quotes { false } else { true },
        _ => false
    }
}

fn update_file_pos(c: char, file_pos: &mut FilePosition) {
    match c {
        '\n' => {
            file_pos.column = 0;
            file_pos.line += 1;
            file_pos.line_text.clear()
        },
        '\r' => (),
        '\t' => file_pos.column += "\t".len(),
        _ => file_pos.column += 1
    }
}

fn incremented_fp(file_pos: &FilePosition) -> FilePosition {
    let mut file_pos2 = file_pos.clone();
    file_pos2.column += 1;
    file_pos2
}

fn column_adjusted_fp(file_pos: &FilePosition, token: &String) -> FilePosition {
    let mut temp_file_pos = file_pos.clone();
    temp_file_pos.column -= token.len();
    temp_file_pos
}

fn token_adjusted_fp(file_pos: &FilePosition, token: &String) -> FilePosition {
    let mut temp_file_pos = file_pos.clone();
    temp_file_pos.column -= token.len() + 1;
    temp_file_pos
}

impl Scanner {
    pub fn init() -> Scanner {
        Scanner{ error: false, tokens: vec![] }
    }

    fn push_delim_token(&mut self, token: &String, file_pos: &FilePosition) {
        if token.is_empty() {
            return;
        }

        let temp_file_pos = column_adjusted_fp(&file_pos, &token);
        self.tokens.push(
            Token::Delimiter { 
                delim: delimiter::Delimiter::from_str(token).unwrap(),
                fp: temp_file_pos
            }
        )
    }

    fn push_non_delim_token(&mut self, token: &mut String, file_pos: &FilePosition) {
        if token.is_empty() {
            return;
        }

        let temp_file_pos = token_adjusted_fp(&file_pos, &token);
        if is_keyword(token) {
            self.tokens.push(
                Token::Keyword {
                    keyword: keyword::Keyword::from_str(token).unwrap(),
                    fp: temp_file_pos
                }
            )
        } else if is_value(token) {
            self.tokens.push(
                Token::Value {
                    value: token.clone(),
                    fp: temp_file_pos
                }
            )
        } else if is_ident(token) {
            self.tokens.push(
                Token::Ident {
                    ident: token.clone(),
                    fp: temp_file_pos
                }
            )
        } else {
            self.error = true;
            error!("Unexpected: {}", file_pos.position());
        }
        token.clear()
    }

    fn add_delim_token(&mut self, delim: Either<char, &String>, file_pos: &FilePosition) {
        match delim {
            Left(c) => {
                if is_delim(&c.to_string()) {
                    self.push_delim_token(&c.to_string(), file_pos)
                }
            },
            Right(d) => {
                if is_delim(&d.to_string()) {
                    self.push_delim_token(d, file_pos)
                } else {
                    self.push_delim_token(&d.substring(0, 1).to_string(), file_pos);
                    self.push_delim_token(&d.substring(1, d.len()).to_string(), &incremented_fp(&file_pos));
                }
            }
        }
    }

    pub fn scan(&mut self, script: &String) {
        debug!("SCRIPT\n====================\n{:?}\n====================\n", script);

        let lines = script.lines();
        let text: Vec<char> = script.chars().collect();
        let mut token = String::from("");
        let mut inside_quotes = false;
        let mut in_comment = false;
        let file_pos = &mut FilePosition { line: 0, column: 0, line_text: String::from("") };
        let mut skip = false;
        
        let peek_raw_delim = |i: &usize, text: &Vec<char>, inside_quotes: &bool| -> bool { 
            *i < text.len() - 1 && is_raw_delim(&text[i + 1], inside_quotes.borrow()) 
        };

        for i in 0..text.len() {
            let c = text[i];
            file_pos.line_text = lines.clone().nth(file_pos.line).unwrap().to_string();

            if !is_valid_character(&c, &inside_quotes.borrow()) {
                self.error = true;
                error!("Invalid character \'{}\': {}", &c, file_pos.position());
                continue;
            }

            update_file_pos(c, file_pos);
            if c == '#' {
                in_comment = true;
                continue;
            } else if in_comment && c != '\n' {
                continue;
            } else if in_comment && c == '\n' {
                in_comment = false;
                continue;
            } else if is_whitespace(c, inside_quotes) {
                self.push_non_delim_token(&mut token, &file_pos);
                continue;
            } else if skip {
                skip = false;
                continue;
            }

            if is_raw_delim(&c, inside_quotes.borrow()) {
                self.push_non_delim_token(&mut token, &file_pos);
                let mut delim = String::from(text[i]);
                if peek_raw_delim(&i, &text, &inside_quotes.borrow()) {
                    skip = true;
                    delim.push(text[i + 1]);
                    self.add_delim_token(Right(&delim), &file_pos)
                } else {
                    self.add_delim_token(Left(c), &file_pos)
                }
            } else {
                if c == '\'' || c == '\"' {
                    inside_quotes = !inside_quotes;
                }
                
                if (c == ' ' && inside_quotes) || !c.is_whitespace() {
                    token.push(c)
                }
            }
        }
        
        self.push_non_delim_token(&mut token, &incremented_fp(&file_pos));
        debug!("TOKENS\n====================\n");
        for t in self.tokens.iter() {
            debug!("{:?}", t);
        }
        debug!("\n====================\n");
    }
}