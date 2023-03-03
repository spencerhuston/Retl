use std::borrow::Borrow;
use std::str::FromStr;
use regex::Regex;
use either::*;
use substring::Substring;

use crate::defs::keyword;
use crate::defs::delimiter;
use crate::defs::raw_delimiter;
use crate::utils::file_position::FilePosition;
use crate::scanner::token::Token;

pub struct Scanner {
    pub(crate) tokens: Vec<Token>
}

fn is_valid_character(c: &char) -> bool {
    c.is_alphanumeric() || c.is_whitespace() || *c != '_' || *c != '\'' || *c != '\"' || !is_raw_delim(c)
}

fn is_raw_delim(c: &char) -> bool {
    match raw_delimiter::RawDelimiter::from_str(&c.to_string()) {
        Ok(_) => true,
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

fn is_whitespace_update_file_pos(c: char, file_pos: &mut FilePosition, inside_quotes: &mut bool) -> bool {
    match c {
        '\n' => {
            file_pos.column = 0;
            file_pos.line += 1;
            file_pos.line_text.clear();
            true
        },
        '\r' => true,
        ' ' => {
            file_pos.column += 1;
            !*inside_quotes
        },
        '\t' => {
            file_pos.column += "\t".len();
            true
        },
        _ => {
            file_pos.column += 1;
            false
        }
    }
}

fn push_char(token_text: &mut String, c: char, file_pos: &mut FilePosition) {
    file_pos.column += 1;
    token_text.push(c);
}

impl Scanner {
    fn push_delim_token(&mut self, token: &String, file_pos: &FilePosition) {
        self.tokens.push(
            Token::Delimiter { 
                delim: delimiter::Delimiter::from_str(token).unwrap(),
                fp: file_pos.clone()
            }
        )
    }

    fn push_non_delim_token(&mut self, token: &mut String, file_pos: &FilePosition) {
        if is_keyword(token) {
            self.tokens.push(
                Token::Keyword { 
                    keyword: keyword::Keyword::from_str(token).unwrap(),
                    fp: file_pos.clone()
                }
            )
        } else if is_value(token) {
            self.tokens.push(
                Token::Value {
                    value: token.clone(),
                    fp: file_pos.clone()
                }
            )
        } else if is_ident(token) {
            self.tokens.push(
                Token::Ident { 
                    ident: token.clone(),
                    fp: file_pos.clone()
                }
            )
        } else {
            () // THROW ERROR HERE, INVALID TOKEN
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
                if is_delim(&d.borrow()) {
                    self.push_delim_token(d, file_pos)
                } else {
                    self.push_delim_token(&d.substring(0, 1).to_string(), file_pos);
                    self.push_delim_token(&d.substring(1, d.len()).to_string(), file_pos);
                }
            }
        }
    }

    pub fn scan(&mut self, script: &String) {
        println!("====================");
        println!("{}", script);
        println!("====================");

        let mut lines = script.lines();
        let text: Vec<char> = script.chars().collect();
        let mut token = String::from("");
        let mut inside_quotes = false;
        let file_pos = &mut FilePosition { line: 0, column: 0, line_text: String::from("") };
        let mut skip = false;
        
        let peek_raw_delim = |i: &usize, text: &Vec<char>| -> bool { 
            *i < text.len() - 1 && is_raw_delim(&text[i + 1]) 
        };

        for i in 0..text.len() {
            let c = text[i];
            file_pos.line_text = lines.clone().nth(file_pos.line).unwrap().to_string();

            if !is_valid_character(&c) {
                // THROW ERROR HERE, INVALID CHARACTER
                continue;
            } else if is_whitespace_update_file_pos(c, file_pos, &mut inside_quotes) {
                self.push_non_delim_token(&mut token, &file_pos);
                continue;
            } else if skip {
                skip = false;
                continue;
            }

            if is_raw_delim(&c) {
                self.push_non_delim_token(&mut token, &file_pos);
                let mut delim = String::from(text[i]);
                if peek_raw_delim(&i, &text) {
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
                push_char(&mut token, c, file_pos)
            }
        }
        
        if !token.is_empty() {
            self.push_non_delim_token(&mut token, &file_pos);
        }

        for t in self.tokens.iter() {
            println!("{:?}\n", t)
        }
    }
}