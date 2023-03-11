use log::{debug, error};
use std::borrow::Borrow;
use std::collections::HashMap;

use crate::scanner::token::{make_empty_token, Token};
use crate::defs::keyword::Keyword;
use crate::defs::delimiter::Delimiter;
use crate::defs::expression::{Exp, Expression, Literal};
use crate::defs::expression::Literal::*;
use crate::defs::operator::Operator;
use crate::defs::retl_type::Type;
use crate::defs::retl_type::Type::*;

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

fn get_exp_literal(exp: Exp) -> Literal {
    match exp.exp {
        Expression::Lit{lit} if lit != NullLit => lit,
        _ => NullLit // TODO: Throw error here, dict key requires non-null literal type
    }
}

fn make_list_def_range(start: usize, end: usize, token: &Token) -> Vec<Exp> {
    let mut range: Vec<Exp> = vec![];
    let mut index = start;
    while index < end {
        range.push(
            Exp{
                exp: Expression::Lit{lit: IntLit{literal: index as i32}},
                exp_type: IntType,
                token: token.clone()
            }
        );
        index += 1;
    }
    range
}

fn get_int_literal_value(exp: Expression) -> usize {
    match exp {
        Expression::Lit{lit} => {
            match lit {
                IntLit{literal} => {
                    if literal < 0 {
                        // TODO: Throw error here, negative range bound
                        0
                    } else {
                        literal as usize
                    }
                },
                _ => 0 // TODO: Throw error here, not an integer literal
            }
        },
        _ => 0 // TODO: Throw error here, not an integer literal
    }
}

impl Parser {
    pub fn init() -> Parser {
        Parser{root_exp: Exp{
            exp: Expression::Empty,
            exp_type: NullType,
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
        println!("curr: {:?}", self.tokens[self.index].clone());
        self.tokens[self.index].clone()
    }

    fn advance(&mut self) {
        self.index += 1
    }

    fn match_required_delimiter(&mut self, delim: Delimiter) -> bool {
        let matched = match self.curr() {
            Token::Delimiter{delim: d, fp: _, .. } if d == delim => true,
            _ => false
        };

        if !matched {
            error!("Expected {:?}, got {}",
                delim,
                get_token_as_string(self.tokens[self.index].clone()))
        } else {
            println!("MATCHED DELIM: curr {:?}, delim {:?}", self.curr(), delim);
        }

        self.advance();
        matched
    }

    fn match_required_keyword(&mut self, keyword: Keyword) -> bool {
        let matched = match self.curr() {
            Token::Keyword{keyword: k, fp: _, .. } if k == keyword => true,
            _ => false
        };

        if !matched {
            error!("Expected {:?}, got {}",
                keyword,
                get_token_as_string(self.tokens[self.index].clone()))
        } else {
            println!("MATCHED KEYWORD: curr {:?}, delim {:?}", self.curr(), keyword);
        }

        self.advance();
        matched
    }

    fn match_optional_delimiter(&mut self, delim: Delimiter) -> bool {
        let matched = match self.curr() {
            Token::Delimiter{delim: d, fp: _, .. } if d == delim => true,
            _ => false
        };

        if matched {
            println!("MATCHED DELIM: curr {:?}, delim {:?}", self.curr(), delim);
            self.advance()
        }

        matched
    }

    fn match_optional_keyword(&mut self, keyword: Keyword) -> bool {
        let matched = match self.curr() {
            Token::Keyword{keyword: k, fp: _, .. } if k == keyword => true,
            _ => false
        };

        if matched {
            println!("MATCHED KEYWORD: curr {:?}, delim {:?}", self.curr(), keyword);
            self.advance()
        }

        matched
    }

    fn match_ident(&mut self) -> String {
        match self.curr() {
            Token::Ident{ident, fp: _} => {
                println!("MATCHED IDENT: curr {:?}, ident {:?}", self.curr(), ident);
                self.advance();
                ident
            },
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
            _ => self.parse_simple_expression()
        }
    }

    fn parse_let(&mut self) -> Exp {
        let token = self.curr();
        let ident = self.match_ident();
        
        let mut let_type = UnknownType;
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
                exp: Expression::Primitive{
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
        let token = self.curr();
        let mut operator: Option<Operator> = None;
        if self.match_optional_keyword(Keyword::Not) {
            operator = Some(self.curr().to_operator().unwrap())
        } else if self.match_optional_delimiter(Delimiter::Minus) {
            operator = Some(self.curr().to_operator().unwrap())
        }

        let right = self.parse_tight();
        match operator {
            Some(Operator::Not) => Exp{
                exp: Expression::Primitive{
                    operator: Operator::And,
                    left: Box::new(Exp{
                        exp: Expression::Lit{lit: Literal::BoolLit{literal: false}},
                        exp_type: BoolType,
                        token: token.clone()
                    }),
                    right: Box::new(right)},
                exp_type: UnknownType,
                token: token.clone()
            },
            Some(Operator::Minus) => Exp{
                exp: Expression::Primitive{
                    operator: Operator::Minus,
                    left: Box::new(Exp{
                        exp: Expression::Lit{lit: Literal::IntLit{literal: 0}},
                        exp_type: IntType,
                        token: token.clone()
                    }),
                    right: Box::new(right)},
                exp_type: UnknownType,
                token: token.clone()
            },
            _ => right
        }
    }

    fn parse_tight(&mut self) -> Exp {
        match self.curr() {
            Token::Delimiter{..}
                if self.match_optional_delimiter(Delimiter::BraceLeft)
            => {
                let exp = self.parse_simple_expression();
                self.match_required_delimiter(Delimiter::BraceRight);
                exp
            },
            _ => {
                let mut inner_app = self.parse_application();
                while self.match_optional_delimiter(Delimiter::Bird) {
                    let mut outer_app = self.parse_application();
                    match &mut outer_app.exp {
                        Expression::Application{ident: _, ref mut args} => {
                            args.insert(0, inner_app);
                        },
                        _ => () // TODO: throw error if outer app is not application expression
                    }
                    inner_app = outer_app
                }
                inner_app
            }
        }
    }

    fn parse_access_index(&mut self) -> i32 {
        match self.parse_literal().exp {
            Expression::Lit{lit} => {
                match lit {
                    IntLit{literal} => literal,
                    _ => -1 // TODO: Throw error here
                }
            },
            _ => -1 // TODO: Throw error here
        }
    }

    fn parse_atom(&mut self) -> Exp {
        match self.curr() {
            Token::Ident{ident, fp: _} => {
                let reference = Exp{
                    exp: Expression::Reference{ident},
                    exp_type: UnknownType,
                    token: self.curr().clone()
                };

                self.advance();

                if self.match_optional_delimiter(Delimiter::TupleAccess) {
                    let token = self.curr().clone();
                    let access_index = self.parse_access_index();
                    let mut tuple_access = Exp{
                        exp: Expression::TupleAccess{
                            ident: Box::new(reference),
                            index: access_index
                        },
                        exp_type: UnknownType,
                        token: token.clone()
                    };

                    while self.match_optional_delimiter(Delimiter::TupleAccess) {
                        tuple_access.exp = Expression::TupleAccess{
                            ident: Box::new(tuple_access.clone()),
                            index: self.parse_access_index()
                        }
                    }
                    tuple_access
                } else {
                    reference
                }
            },
            _ => self.parse_literal()
        }
    }

    fn parse_literal(&mut self) -> Exp {
        let token = self.curr();
        match self.curr() {
            Token::Keyword{..} if self.match_optional_keyword(Keyword::True)
                => {
                    Exp{
                        exp: Expression::Lit{lit: BoolLit{literal: true}},
                        exp_type: BoolType,
                        token: token.clone()
                    }
            },
            Token::Keyword{..} if self.match_optional_keyword(Keyword::False)
                => {
                    Exp{
                        exp: Expression::Lit{lit: BoolLit{literal: false}},
                        exp_type: BoolType,
                        token: token.clone()
                    }
            },
            Token::Keyword{..} if self.match_optional_keyword(Keyword::Null)
                => {
                    Exp{
                        exp: Expression::Lit{lit: NullLit},
                        exp_type: NullType,
                        token: token.clone()
                    }
            },
            Token::Value{value, fp: _} => {
                if value.starts_with('\'') {
                    self.advance();
                    Exp{
                        exp: Expression::Lit{lit: CharLit{literal: value}},
                        exp_type: CharType,
                        token: token.clone()
                    }
                } else if value.starts_with('\"') {
                    self.advance();
                    Exp{
                        exp: Expression::Lit{lit: StringLit{literal: value}},
                        exp_type: StringType,
                        token: token.clone()
                    }
                } else {
                    let int_literal = Exp{
                        exp: Expression::Lit{lit: IntLit{literal: value.parse().unwrap()}},
                        exp_type: IntType,
                        token: token.clone()
                    };
                    self.advance();
                    if self.match_optional_delimiter(Delimiter::Range) {
                        let int_literal_range_bound_noninclusive = self.parse_literal();
                        let range_start = get_int_literal_value(int_literal.exp.clone());
                        let range_end = get_int_literal_value(int_literal_range_bound_noninclusive.exp);
                        if range_end <= range_start {
                            // TODO: Throw error here, invalid range bounds
                            int_literal
                        } else {
                            Exp{
                                exp: Expression::ListDef{
                                    values: make_list_def_range(range_start, range_end, &token)
                                },
                                exp_type: ListType{list_type: Box::new(IntType)},
                                token
                            }
                        }
                    } else {
                        int_literal
                    }
                }
            }
            _ => self.make_empty_exp_todo() // TODO: Throw error here
        }
    }
    
    fn parse_branch(&mut self) -> Exp {
        let token = self.curr();
        self.match_required_delimiter(Delimiter::ParenLeft);
        let condition = self.parse_simple_expression();
        self.match_required_delimiter(Delimiter::ParenRight);
        self.match_required_delimiter(Delimiter::BraceLeft);
        let if_branch = self.parse_simple_expression();
        self.match_required_delimiter(Delimiter::BraceRight);

        let mut else_branch = None;
        if self.match_optional_keyword(Keyword::Else) {
            self.match_required_delimiter(Delimiter::BraceLeft);
            else_branch = Some(self.parse_simple_expression());
            self.match_required_delimiter(Delimiter::BraceRight);
        }
        let exp_type = if else_branch == None { NullType } else { if_branch.exp_type.clone() };
        
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
        let token = self.curr().clone();
        let first_element = self.parse_simple_expression();

        if self.match_optional_delimiter(Delimiter::Comma) {
            let mut elements: Vec<Exp> = vec![first_element, self.parse_simple_expression()];
            while self.match_optional_delimiter(Delimiter::Comma) &&
                !self.match_optional_delimiter(Delimiter::BracketRight) {
                elements.push(self.parse_simple_expression())
            }
            Exp{
                exp: Expression::ListDef{values: elements},
                exp_type: ListType{list_type: Box::new(UnknownType)},
                token
            }
        } else if self.match_optional_delimiter(Delimiter::DenoteType) {
            let mut mapping: HashMap<Literal, Exp> = HashMap::new();
            let first_key = get_exp_literal(first_element);
            mapping.insert(first_key, self.parse_simple_expression());

            while self.match_optional_delimiter(Delimiter::Comma) &&
                !self.match_optional_delimiter(Delimiter::BracketRight) {
                let key = get_exp_literal(self.parse_simple_expression());
                self.match_required_delimiter(Delimiter::DenoteType);
                let value = self.parse_simple_expression();
                mapping.insert(key, value);
            }
            Exp{
                exp: Expression::DictDef{mapping},
                exp_type: DictType{
                    key_type: Box::new(UnknownType),
                    value_type: Box::new(UnknownType)
                },
                token
            }
        } else {
            // TODO: Throw error here, not a valid list
            first_element
        }
    }
    
    fn parse_tuple_def(&mut self) -> Exp {
        self.make_empty_exp_todo()
    }

    fn parse_schema_def(&mut self) -> Exp {
        let token = self.curr().clone();
        self.match_required_delimiter(Delimiter::BraceLeft);
        let mut mapping: HashMap<String, Type> = HashMap::new();

        while self.match_optional_delimiter(Delimiter::Comma) && // TODO: Make function for this operation
            !self.match_optional_delimiter(Delimiter::BraceRight) {
            let ident = self.match_ident();
            self.match_required_delimiter(Delimiter::DenoteType);
            let col_type = self.parse_type();
            mapping.insert(ident, col_type);
        }

        Exp {
            exp: Expression::SchemaDef{mapping},
            exp_type: SchemaType,
            token
        }
    }
    
    fn parse_match(&mut self) -> Exp {
        self.make_empty_exp_todo()
    }
    
    fn parse_lambda(&mut self) -> Exp {
        self.make_empty_exp_todo()
    }

    fn parse_arguments(&mut self) -> Vec<Exp> {
        let mut args: Vec<Exp> = vec![self.parse_simple_expression()];
        while self.match_optional_delimiter(Delimiter::Comma) &&
            !self.match_optional_delimiter(Delimiter::ParenRight) {
            args.push(self.parse_simple_expression())
        }
        args
    }

    fn parse_application(&mut self) -> Exp {
        let token = self.curr().clone();
        let ident: Exp = self.parse_atom();

        match ident.exp {
            Expression::Lit{..} => ident,
            _ => {
                self.match_required_delimiter(Delimiter::ParenLeft);
                let args: Vec<Exp> = self.parse_arguments();
                let mut app = Exp{
                    exp: Expression::Application{ident: Box::new(ident), args},
                    exp_type: UnknownType,
                    token
                };

                while self.match_optional_delimiter(Delimiter::ParenLeft) {
                    let outer_args = self.parse_arguments();
                    app.exp = Expression::Application{ident: Box::new(app.clone()), args: outer_args}
                }
                app
            }
        }
    }

    fn parse_type(&mut self) -> Type {
        let first_type = match self.curr() {
            Token::Keyword{..} if self.match_optional_keyword(Keyword::Int) => IntType,
            Token::Keyword{..} if self.match_optional_keyword(Keyword::Bool) => BoolType,
            Token::Keyword{..} if self.match_optional_keyword(Keyword::Char) => CharType,
            Token::Keyword{..} if self.match_optional_keyword(Keyword::String) => StringType,
            Token::Keyword{..} if self.match_optional_keyword(Keyword::Null) => NullType,
            Token::Keyword{..} if self.match_optional_keyword(Keyword::List) => {
                self.match_required_delimiter(Delimiter::BracketLeft);
                let list_type = self.parse_type();
                self.match_required_delimiter(Delimiter::BracketRight);
                ListType{list_type: Box::new(list_type)}
            },
            Token::Keyword{..} if self.match_optional_keyword(Keyword::Dict) => {
                self.match_required_delimiter(Delimiter::BracketLeft);
                let key_type = self.parse_type();
                self.match_required_delimiter(Delimiter::DenoteType);
                let value_type = self.parse_type();
                self.match_required_delimiter(Delimiter::BracketRight);
                DictType{key_type: Box::new(key_type), value_type: Box::new(value_type)}
            },
            Token::Keyword{..} if self.match_optional_keyword(Keyword::Tuple) => {
                self.match_required_delimiter(Delimiter::BracketLeft);
                let mut tuple_types = vec![self.parse_type()];

                while self.match_optional_delimiter(Delimiter::Comma) &&
                    !self.match_optional_delimiter(Delimiter::BracketRight) {
                    tuple_types.push(self.parse_type())
                }
                TupleType{tuple_types}
            },
            Token::Keyword{..} if self.match_optional_keyword(Keyword::Schema) => SchemaType,
            Token::Delimiter{..} if self.match_optional_delimiter(Delimiter::ParenLeft) => {
                let mut param_types = vec![self.parse_type()];
                while self.match_optional_delimiter(Delimiter::Comma) &&
                    !self.match_optional_delimiter(Delimiter::ParenRight) {
                    param_types.push(self.parse_type())
                }

                self.match_required_delimiter(Delimiter::ReturnType);
                let return_type = self.parse_type();
                FuncType{param_types, return_type: Box::new(return_type)}
            }
            _ => UnknownType // TODO: Throw error here
        };

        if self.match_optional_delimiter(Delimiter::ReturnType) {
            FuncType{param_types: vec![first_type], return_type: Box::new(self.parse_type())}
        } else {
            first_type
        }
    }
}