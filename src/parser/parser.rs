use log::{debug, error};
use std::borrow::Borrow;

use crate::scanner::token::{make_empty_token, Token};
use crate::defs::keyword::Keyword;
use crate::defs::delimiter::Delimiter;
use crate::defs::expression::{Exp, Expression};
use crate::defs::expression::Expression::Primitive;
use crate::defs::retl_type::Type;
use crate::defs::retl_type::Type::{NullType, UnknownType};

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

fn get_return_type(exp: &Option<Exp>) -> Type {
    match exp {
        Some(e) => e.exp_type.clone(),
        None => NullType
    }
}

impl Parser {
    pub fn init() -> Parser {
        Parser{root_exp: Exp{
            exp: Expression::Empty,
            exp_type: Type::NullType,
            token: make_empty_token()
        }, tokens: vec![], index: 0, dummy_count: 0, anon_count: 0}
    }

    fn make_empty_exp_todo(&mut self) -> Exp {
        Exp{
            exp: Expression::Empty,
            exp_type: NullType,
            token: self.curr()
        }
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
            Token::Delimiter{delim: d, fp: _, .. } if *d == delim => true,
            _ => false
        };

        if !matched {
            error!("Expected {:?}, got {}",
                delim,
                get_token_as_string(self.tokens[self.index].clone()))
        }

        self.advance();
        matched
    }

    fn match_required_keyword(&mut self, keyword: Keyword) -> bool {
        let matched = match self.tokens[self.index].borrow() {
            Token::Keyword{keyword: k, fp: _, .. } if *k == keyword => true,
            _ => false
        };

        if !matched {
            error!("Expected {:?}, got {}",
                keyword,
                get_token_as_string(self.tokens[self.index].clone()))
        }

        self.advance();
        matched
    }

    fn match_optional_delimiter(&mut self, delim: Delimiter) -> bool {
        let matched = match self.tokens[self.index].borrow() {
            Token::Delimiter{delim: d, fp: _, .. } if *d == delim => true,
            _ => false
        };

        if matched { self.advance() }

        matched
    }

    fn match_optional_keyword(&mut self, keyword: Keyword) -> bool {
        let matched = match self.tokens[self.index].borrow() {
            Token::Keyword{keyword: k, fp: _, .. } if *k == keyword => true,
            _ => false
        };

        if matched { self.advance() }

        matched
    }

    fn match_ident(&self) -> String {
        match self.curr() {
            Token::Ident{ident, fp: _} => ident,
            _ => {
                error!("Expected identifier, got {}",
                    get_token_as_string(self.tokens[self.index].clone()));
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

    fn is_binary_op(&self, min: i32) -> bool {
        match self.curr().to_operator() {
            Some(op) => op.is_binary_op(min),
            None => false
        }
    }

    pub fn parse(&mut self, tokens: &Vec<Token>) {
        self.tokens = tokens.clone();
        self.root_exp = self.parse_expression();
        println!("{:?}", self.root_exp)
    }

    fn parse_expression(&mut self) -> Exp {
        match self.curr() {
            Token::Keyword{..}
                if self.match_optional_keyword(Keyword::Let)
                    => self.parse_let(),
            _ => self.make_empty_exp_todo()
        }
    }

    fn parse_let(&mut self) -> Exp {
        let token = self.curr();
        let ident = self.match_ident();
        
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
        let exp_type = get_return_type(&after_let_exp);

        Exp{
            exp: Expression::Let{
                ident,
                let_type,
                let_exp: Box::new(let_exp),
                after_let_exp: Box::new(after_let_exp)
            },
            exp_type,
            token
        }
    }

    fn parse_simple_expression(&mut self) -> Exp {
        match self.curr() {
            Token::Keyword{..}
                if self.match_optional_keyword(Keyword::If)
                    => self.parse_branch(),
            Token::Delimiter{..}
                if self.match_optional_delimiter(Delimiter::BracketLeft)
                    => self.parse_collection_def(),
            Token::Delimiter{..}
                if self.match_optional_delimiter(Delimiter::ParenLeft)
                    => self.parse_tuple_def(),
            Token::Delimiter{..}
                if self.match_optional_delimiter(Delimiter::SchemaStart)
                    => self.parse_schema_def(),
            Token::Keyword{..}
                if self.match_optional_keyword(Keyword::Match)
                    => self.parse_match(),
            Token::Delimiter{..}
                if self.match_optional_delimiter(Delimiter::LambdaSig)
                    => self.parse_lambda(),
            Token::Keyword{..}
                if self.match_optional_keyword(Keyword::Alias)
                    => self.parse_alias(),
            _ => self.parse_utight_with_min(0)
        }
    }

    fn parse_alias(&mut self) -> Exp {
        let token = self.curr();
        let ident = self.match_ident();
        self.match_required_delimiter(Delimiter::Assignment);
        let alias = self.parse_type();

        let mut after_alias_exp: Option<Exp> = None;
        if self.match_optional_delimiter(Delimiter::StatementEnd) {
            after_alias_exp = Some(self.parse_expression())
        }
        let exp_type = get_return_type(&after_alias_exp);

        Exp{
            exp: Expression::Alias{
                ident,
                alias,
                after_alias_exp: Box::new(after_alias_exp)
            },
            exp_type,
            token
        }
    }

    fn parse_utight_with_min(&mut self, min: i32) -> Exp {
        let token = self.curr();
        let mut left = self.parse_utight();
        while self.is_binary_op(min) {
            let operator = self.curr().clone().to_operator().unwrap();
            let temp_min = operator.get_precedence() + 1;
            self.advance();
            let right = self.parse_utight_with_min(temp_min);
            left = Exp{
                exp: Primitive{
                    operator,
                    left: Box::new(left),
                    right: Box::new(right)},
                exp_type: UnknownType,
                token: token.clone()
            }
        }
        left
    }

    fn parse_utight(&mut self) -> Exp {
        self.make_empty_exp_todo()
    }

    fn parse_tight(&mut self) -> Exp {
        self.make_empty_exp_todo()
    }

    fn parse_atom(&mut self) -> Exp {
        self.make_empty_exp_todo()
    }

    fn parse_primitive(&mut self) -> Exp {
        self.make_empty_exp_todo()
    }
    
    fn parse_branch(&mut self) -> Exp {
        let token = self.curr();
        self.match_required_delimiter(Delimiter::ParenLeft);
        let condition = self.parse_simple_expression();
        self.match_required_delimiter(Delimiter::ParenRight);
        self.match_required_delimiter(Delimiter::BraceLeft);
        let if_branch = self.parse_simple_expression();

        let mut else_branch = None;
        if self.match_optional_keyword(Keyword::Else) {
            else_branch = Some(self.parse_simple_expression());
        }
        let exp_type = if_branch.exp_type.clone();
        
        Exp{
            exp: Expression::Branch{
                condition: Box::new(condition),
                if_branch: Box::new(if_branch),
                else_branch: Box::new(else_branch)
            },
            exp_type,
            token
        }
    }
    
    fn parse_collection_def(&mut self) -> Exp {
        self.make_empty_exp_todo()
    }
    
    fn parse_tuple_def(&mut self) -> Exp {
        self.make_empty_exp_todo()
    }

    fn parse_schema_def(&mut self) -> Exp {
        self.make_empty_exp_todo()
    }
    
    fn parse_match(&mut self) -> Exp {
        self.make_empty_exp_todo()
    }
    
    fn parse_lambda(&mut self) -> Exp {
        self.make_empty_exp_todo()
    }

    // fn parse_application(&mut &self) -> Exp {
        
    // }

    fn parse_type(&mut self) -> Type {
        Type::UnknownType
    }
}