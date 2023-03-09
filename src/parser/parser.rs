use log::{error};

use crate::defs::keyword::Keyword;
use crate::defs::delimiter::Delimiter;

pub struct Parser {
    pub (crate) rootExp: Exp,
    tokens: Vec<Token>,
    index: i32,
    dummyCount: i32,
    anonCount: i32
}

impl Parser {
    fn curr(&self) -> Token {
        self.tokens[self.index]
    }

    fn advance(&self) {
        self.index += 1
    }
    
    fn matchT<T>(&self, index: i32, token: T) -> bool {
        match self.tokens[index] {
            token => true,
            _ => false
        }
    }

    fn peek<T>(&self, token: T) -> bool {
        index < tokens.len() - 1 && self.matchT<T>(self.index + 1, token)
    }

    fn match_required<T>(&self, token: T) {
        let matched = self.matchT<T>(self.index, token);

        if !matched {
            error!("Expected {}, got {}", tokens[index], token)
        }

        self.advance();
        matched
    }

    fn match_optional<T>(&self, token: T) {
        let matched =  self.matchT<T>(self.index, token);

        if matched {
            self.advance()
        }

        matched
    }

    fn dummy(&self) -> String {
        let dummyString = String::from("dummy$");
        dummyString += self.dummyCount.to_string;
        self.dummyCount += 1;
        dummyString
    }

    fn anon(&self) -> String {
        let anonString = String::from("anon$");
        anonString += self.anonCount.to_string;
        self.anonCount += 1;
        anonString
    }

    // pub fn parse(&self, tokens: Vec<Token>) {
        
    // }

    // fn parseExp(&self) {
        
    // }

    // fn parseSimpleExp(&self) {
        
    // }

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

    // fn parseType(&self) {
        
    // }
}