use log::{debug, error};
use std::borrow::Borrow;

use crate::scanner::token::Token;
use crate::defs::keyword::Keyword;
use crate::defs::delimiter::Delimiter;
use crate::defs::expression::Exp;
use crate::defs::expression::ExpMeta;
use crate::defs::retl_type::Type;

pub struct Parser {
    root_exp: Exp,
    tokens: Vec<Token>,
    index: usize,
    dummy_count: i32,
    anon_count: i32
}

fn get_token_as_string(token: Token) -> String {
    match token {
        Token::Delimiter{delim, fp: _} => delim.to_string(),
        Token::Keyword{keyword, fp: _} => keyword.to_string(),
        Token::Value{value, fp: _} => value,
        Token::Ident{ident, fp: _} => ident
    }
}

impl Parser {
    pub fn init() -> Parser {
        Parser{root_exp: Exp::Empty, tokens: vec![], index: 0, dummy_count: 0, anon_count: 0}
    }

    fn curr(&self) -> Token {
        self.tokens[self.index].clone()
    }

    fn advance(&mut self) {
        self.index += 1
    }

    // fn peek<T>(&self, token: T) -> bool {
    //     self.index < self.tokens.len() - 1 && self.match_t::<T>(self.index + 1, token)
    // }

    fn match_required_delimiter(&mut self, delim: Delimiter) -> bool {
        let matched = match self.tokens[self.index].borrow() {
            delim => true,
            _ => false
        };

        if !matched {
            error!("Expected {}, got {}", delim.to_string(), get_token_as_string(self.tokens[self.index].clone()))
        }

        self.advance();
        matched
    }

    fn match_required_keyword(&mut self, keyword: Keyword) -> bool {
        let matched = match self.tokens[self.index].borrow() {
            keyword => true,
            _ => false
        };

        if !matched {
            error!("Expected {}, got {}", keyword.to_string(), get_token_as_string(self.tokens[self.index].clone()))
        }

        self.advance();
        matched
    }

    fn match_optional_delimiter(&mut self, delim: Delimiter) -> bool {
        let matched = match self.tokens[self.index].borrow() {
            delim => true,
            _ => false
        };

        if matched {
            self.advance()
        }

        matched
    }

    fn match_optional_keyword(&mut self, keyword: Keyword) -> bool {
        let matched = match self.tokens[self.index].borrow() {
            keyword => true,
            _ => false
        };

        if matched {
            self.advance()
        }

        matched
    }

    fn match_ident<T>(&self) -> String {
        match self.curr() {
            Token::Ident{ident: i, fp: _} => {
                i
            },
            _ => {
                error!("Expected identifier, got {}", get_token_as_string(self.tokens[self.index].clone()));
                String::from("")
            }
        }
    }

    fn dummy(&mut self) -> String {
        let mut dummy_string = String::from("dummy$");
        dummy_string += &self.dummy_count.to_string();
        self.dummy_count += 1;
        dummy_string
    }

    fn anon(&mut self) -> String {
        let mut anon_string = String::from("anon$");
        anon_string += &self.anon_count.to_string();
        self.anon_count += 1;
        anon_string
    }

    pub fn parse(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
        self.root_exp = self.parse_expression();
    }

    fn parse_expression(&mut self) -> Exp {
        match self.curr() {
            Token::Keyword{keyword: k, fp: _} if self.match_optional_keyword(k.clone()) => self.parse_let(),
            _ => Exp::Empty // TODO
        }
    }

    fn parse_let(&mut self) -> Exp {
        let token = self.curr();
        self.match_required_keyword(Keyword::Let);
        let ident = self.match_ident::<Token>();
        
        let mut let_type = Type::UnknownType;
        if self.match_optional_delimiter(Delimiter::DenoteType) {
            let_type = self.parse_type()
        }

        self.match_required_delimiter(Delimiter::Assignment);
        let let_exp = self.parse_simple_expression();

        let mut after_let_exp: Option<Exp> = None;
        if self.match_optional_delimiter(Delimiter::StatementEnd) {
            after_let_exp = Some(self.parse_expression())
        }

        let meta = ExpMeta{token, exp_type: Type::UnknownType}; // TODO: Fix?
        Exp::Let{
            ident,
            let_type,
            let_exp: Box::new(let_exp), 
            after_let_exp: Box::new(after_let_exp), 
            meta
        }
    }

    fn parse_simple_expression(&self) -> Exp {
        Exp::Empty // TODO
    }

    // fn parseAlias(&self) {
        
    // }

    // fn parseUtight(&self) {
        
    // }

    // fn parseTight(&self) {
        
    // }

    // fn parseAtom(&self) {
        
    // }

    // fn parsePrimitive(&self) {
        
    // }
    
    // fn parseBranch(&self) {
        
    // }
    
    // fn parseCollectionDef(&self) {
        
    // }
    
    // fn parseTupleDef(&self) {

    // }

    // fn parseSchemaDef(&self) {

    // }
    
    // fn parseMatch(&self) {
        
    // }
    
    // fn parseLambda(&self) {
        
    // }

    // fn parseApplication(&self) {
        
    // }

    fn parse_type(&self) -> Type {
        Type::UnknownType
    }
}