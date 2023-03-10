use log::{debug, error};
use core::any::TypeId;

use crate::scanner::token::Token;
use crate::defs::keyword::Keyword;
use crate::defs::delimiter::Delimiter;
use crate::defs::expression::Exp;
use crate::defs::expression::ExpMeta;
use crate::defs::retl_type::Type;

pub struct Parser {
    pub (crate) root_exp: Exp,
    tokens: Vec<Token>,
    index: usize,
    dummy_count: i32,
    anon_count: i32
}

fn get_token_as_string(token: Token) -> String {
    match token {
        Token::Delimiter{delim, fp} => delim.to_string(),
        Token::Keyword{keyword, fp} => keyword.to_string(),
        Token::Value{value, fp} => value,
        Token::Ident{ident, fp} => ident,
        _ => String::from("")
    }
}

impl Parser {
    fn curr(&self) -> Token {
        self.tokens[self.index]
    }

    fn advance(&self) {
        self.index += 1
    }
    
    fn match_t<T>(&self, index: usize, token: T) -> bool {
        match self.tokens[index] {
            token => true,
            _ => false
        }
    }

    fn peek<T>(&self, token: T) -> bool {
        self.index < self.tokens.len() - 1 && self.match_t::<T>(self.index + 1, token)
    }

    fn match_required_delimiter(&self, delim: Delimiter) -> bool {
        let matched = self.match_t::<Delimiter>(self.index, delim);

        if !matched {
            error!("Expected {}, got {}", delim.to_string(), get_token_as_string(self.tokens[self.index]))
        }

        self.advance();
        matched
    }

    fn match_required_keyword(&self, keyword: Keyword) -> bool {
        let matched = self.match_t::<Keyword>(self.index, keyword);

        if !matched {
            error!("Expected {}, got {}", keyword.to_string(), get_token_as_string(self.tokens[self.index]))
        }

        self.advance();
        matched
    }

    fn match_optional<T>(&self, token: T) -> bool {
        let matched =  self.match_t::<T>(self.index, token);

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
                error!("Expected identifier, got {}", self.tokens[self.index]);
                String::from("")
            }
        }
    }

    fn dummy(&self) -> String {
        let mut dummy_string = String::from("dummy$");
        dummy_string += &self.dummy_count.to_string();
        self.dummy_count += 1;
        dummy_string
    }

    fn anon(&self) -> String {
        let mut anon_string = String::from("anon$");
        anon_string += &self.anon_count.to_string();
        self.anon_count += 1;
        anon_string
    }

    pub fn parse(&self, tokens: Vec<Token>) {
        self.tokens = tokens;
        self.root_exp = self.parse_expression();
    }

    fn parse_expression(&self) -> Exp {
        match self.curr() {
            Token::Keyword{keyword: k, fp: _} if self.match_optional(k) => self.parse_let(),
            _ => Exp::Empty // TODO
        }
    }

    fn parse_let(&self) -> Exp {
        let token = self.curr();
        self.match_required_keyword(Keyword::Let);
        let ident = self.match_ident::<Token>();
        
        let mut let_type = Type::UnknownType;
        if self.match_optional(Delimiter::DenoteType) {
            let_type = self.parse_type()
        }

        self.match_required_delimiter(Delimiter::Assignment);
        let let_exp = self.parse_simple_expression();

        let mut after_let_exp: Option<Exp> = None;
        if self.match_optional(Delimiter::StatementEnd) {
            after_let_exp = Some(self.parse_expression())
        }

        let meta = ExpMeta{token: token, exp_type: Type::UnknownType}; // TODO: Fix?
        Exp::Let{
            ident: ident, 
            let_type: let_type, 
            let_exp: Box::new(let_exp), 
            after_let_exp: Box::new(after_let_exp), 
            meta: meta}
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