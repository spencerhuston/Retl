use log::{debug, trace, error};
use std::collections::HashMap;

use crate::scanner::token::{Token, make_empty_token, get_fp_from_token};
use crate::defs::keyword::Keyword;
use crate::defs::delimiter::Delimiter;
use crate::defs::expression::{Exp, Expression, Literal, Parameter, Case, Pattern};
use crate::defs::expression::Literal::*;
use crate::defs::operator::Operator;
use crate::defs::retl_type::Type;
use crate::defs::retl_type::Type::*;

pub struct Parser {
    pub error: bool,
    pub root_exp: Exp,
    tokens: Vec<Token>,
    index: usize,
    dummy_count: i32
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

fn make_list_def_range(start: usize, end: usize, token: &Token) -> Vec<Exp> {
    let mut range: Vec<Exp> = vec![];
    let mut index = start;
    while index < end + 1 {
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

impl Parser {
    pub fn init() -> Parser {
        Parser{
            error: false,
            root_exp: Exp{
                exp: Expression::Empty,
                exp_type: NullType,
                token: make_empty_token()
            },
            tokens: vec![], index: 0, dummy_count: 0
        }
    }

    fn make_empty_exp(&mut self) -> Exp {
        Exp{
            exp: Expression::Empty,
            exp_type: NullType,
            token: self.curr().unwrap()
        }
    }

    fn curr(&self) -> Option<Token> {
        if self.index >= self.tokens.len() {
            trace!("curr: EMPTY");
            None
        } else {
            trace!("curr: {:?}", self.tokens[self.index].clone());
            Some(self.tokens[self.index].clone())
        }
    }

    fn advance(&mut self) {
        self.index += 1
    }

    fn match_required_delimiter(&mut self, delim: Delimiter) -> bool {
        let matched = match self.curr() {
            Some(Token::Delimiter{delim: d, fp: _, .. }) if d == delim => true,
            _ => false
        };

        if !matched {
            self.error = true;
            error!("Expected {:?}, got {:?}: {}",
                delim.to_string(),
                get_token_as_string(self.curr().unwrap().clone()).to_string(),
                get_fp_from_token(&self.curr().unwrap()));
        } else {
            trace!("MATCHED DELIM: curr {:?}, delim {:?}", self.curr(), delim);
        }

        self.advance();
        matched
    }

    fn match_required_keyword(&mut self, keyword: Keyword) -> bool {
        let matched = match self.curr() {
            Some(Token::Keyword{keyword: k, fp: _, .. }) if k == keyword => true,
            _ => false
        };

        if !matched {
            self.error = true;
            error!("Expected {:?}, got {:?}: {}",
                keyword.to_string(),
                get_token_as_string(self.curr().unwrap().clone()),
                get_fp_from_token(&self.curr().unwrap()))
        } else {
            trace!("MATCHED KEYWORD: curr {:?}, delim {:?}", self.curr(), keyword);
        }

        self.advance();
        matched
    }

    fn match_optional_delimiter(&mut self, delim: Delimiter) -> bool {
        let matched = match self.curr() {
            Some(Token::Delimiter{delim: d, fp: _, .. }) if d == delim => true,
            _ => false
        };

        if matched {
            trace!("MATCHED DELIM: curr {:?}, delim {:?}", self.curr(), delim);
            self.advance()
        }

        matched
    }

    fn match_optional_keyword(&mut self, keyword: Keyword) -> bool {
        let matched = match self.curr() {
            Some(Token::Keyword{keyword: k, fp: _, .. }) if k == keyword => true,
            _ => false
        };

        if matched {
            trace!("MATCHED KEYWORD: curr {:?}, delim {:?}", self.curr(), keyword);
            self.advance()
        }

        matched
    }

    fn match_ident(&mut self) -> String {
        match self.curr() {
            Some(Token::Ident{ident, fp: _}) => {
                trace!("MATCHED IDENT: curr {:?}, ident {:?}", self.curr(), ident);
                self.advance();
                ident
            },
            _ => {
                self.error = true;
                error!("Expected identifier, got {:?}: {}",
                    get_token_as_string(self.curr().unwrap()),
                    get_fp_from_token(&self.curr().unwrap()));
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

    fn is_binary_op(&self, min: i32) -> bool {
        match self.curr() {
            Some(token) => {
                match token.to_operator() {
                    Some(op) => op.is_binary_op(min),
                    None => false
                }
            },
            _ => false
        }
    }

    fn get_exp_literal(&mut self, exp: Exp) -> Literal {
        match exp.exp {
            Expression::Lit{lit} if lit != NullLit => lit,
            _ => {
                self.error = true;
                error!("Expected non-null literal value: {:?}", get_fp_from_token(&exp.token));
                NullLit
            }
        }
    }

    fn get_int_literal_value(&mut self, exp: Expression) -> usize {
        match exp {
            Expression::Lit{lit} => {
                match lit {
                    IntLit{literal} => {
                        if literal < 0 {
                            self.error = true;
                            error!("Range requires positive integer value: {}",
                                get_fp_from_token(&self.curr().unwrap()));
                            0
                        } else {
                            literal as usize
                        }
                    },
                    _ => {
                        self.error = true;
                        error!("Range requires positive integer value: {}",
                            get_fp_from_token(&self.curr().unwrap()));
                        0
                    }
                }
            },
            _ => {
                self.error = true;
                error!("Range requires positive integer value: {}",
                    get_fp_from_token(&self.curr().unwrap()));
                0
            }
        }
    }

    pub fn parse(&mut self, tokens: &Vec<Token>) {
        self.tokens = tokens.clone();
        self.root_exp = self.parse_expression();
        debug!("ROOT EXPRESSION\n====================\n{:#?}\n====================\n", self.root_exp)
    }

    fn parse_expression(&mut self) -> Exp {
        match self.curr() {
            Some(Token::Keyword{..})
                if self.match_optional_keyword(Keyword::Let)
                    => self.parse_let(),
            Some(_) => {
                let smp = self.parse_simple_expression();
                if self.match_optional_delimiter(Delimiter::StatementEnd) {
                    let token = smp.token.clone();
                    let dummy_ident = self.dummy();
                    let let_type = get_return_type(&Some(smp.clone()));
                    let after_let = self.parse_expression();
                    Exp{
                        exp: Expression::Let{
                            ident: dummy_ident,
                            let_type,
                            let_exp: Box::new(smp),
                            after_let_exp: Box::new(Some(after_let.clone()))
                        },
                        exp_type: get_return_type(&Some(after_let)),
                        token
                    }
                } else {
                    smp
                }
            },
            _ => self.make_empty_exp()
        }
    }

    fn parse_let(&mut self) -> Exp {
        let token = self.curr().unwrap();
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
            Some(Token::Keyword{..})
                if self.match_optional_keyword(Keyword::If)
                    => self.parse_branch(),
            Some(Token::Delimiter{..})
                if self.match_optional_delimiter(Delimiter::BracketLeft) => {
                let first_collection = self.parse_collection_def();
                if self.match_optional_delimiter(Delimiter::ListConcat) {
                    self.match_required_delimiter(Delimiter::BracketLeft);
                    let second_collection = self.parse_collection_def();
                    Exp{
                        exp: Expression::Primitive{
                            operator: Operator::CollectionConcat,
                            left: Box::new(first_collection),
                            right: Box::new(second_collection)
                        },
                        exp_type: IntType,
                        token: self.curr().unwrap().clone()
                    }
                } else {
                    first_collection
                }
            },
            Some(Token::Delimiter{..})
                if self.match_optional_delimiter(Delimiter::ParenLeft)
                    => self.parse_tuple_def_or_simple_expression(),
            Some(Token::Delimiter{..})
                if self.match_optional_delimiter(Delimiter::SchemaStart)
                    => self.parse_schema_def(),
            Some(Token::Keyword{..})
                if self.match_optional_keyword(Keyword::Match)
                    => self.parse_match(),
            Some(Token::Delimiter{..})
                if self.match_optional_delimiter(Delimiter::LambdaSig)
                    => self.parse_lambda(),
            Some(Token::Keyword{..})
                if self.match_optional_keyword(Keyword::Alias)
                    => self.parse_alias(),
            Some(_) => self.parse_utight_with_min(0),
            _ => self.make_empty_exp()
        }
    }

    fn parse_alias(&mut self) -> Exp {
        let token = self.curr().unwrap();
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
        let token = self.curr().unwrap();
        let mut left = self.parse_utight();
        while self.is_binary_op(min) {
            let operator = self.curr().unwrap().clone().to_operator().unwrap();
            let temp_min = operator.get_precedence() + 1;
            self.advance();
            let mut right = self.parse_utight_with_min(temp_min);
            let operator_type = operator.clone().get_type(&mut left, &mut right);
            left = Exp{
                exp: Expression::Primitive{
                    operator,
                    left: Box::new(left),
                    right: Box::new(right)},
                exp_type: operator_type,
                token: token.clone()
            }
        }
        left
    }

    fn parse_utight(&mut self) -> Exp {
        let token = self.curr().unwrap();
        let mut operator: Option<Operator> = None;
        if self.match_optional_keyword(Keyword::Not) {
            operator = Some(Operator::Not)
        } else if self.match_optional_delimiter(Delimiter::Minus) {
            operator = Some(Operator::Minus)
        }

        let right = self.parse_tight();
        match operator {
            Some(Operator::Not) => Exp{
                exp: Expression::Primitive{
                    operator: Operator::Not,
                    left: Box::new(Exp{
                        exp: Expression::Lit{lit: BoolLit{literal: false}},
                        exp_type: BoolType,
                        token: token.clone()
                    }),
                    right: Box::new(right)},
                exp_type: BoolType,
                token: token.clone()
            },
            Some(Operator::Minus) => Exp{
                exp: Expression::Primitive{
                    operator: Operator::Minus,
                    left: Box::new(Exp{
                        exp: Expression::Lit{lit: IntLit{literal: 0}},
                        exp_type: IntType,
                        token: token.clone()
                    }),
                    right: Box::new(right)},
                exp_type: IntType,
                token: token.clone()
            },
            _ => right
        }
    }

    fn parse_tight(&mut self) -> Exp {
        match self.curr() {
            Some(Token::Delimiter{..})
                if self.match_optional_delimiter(Delimiter::BraceLeft)
                    => {
                let exp = self.parse_expression();
                self.match_required_delimiter(Delimiter::BraceRight);
                exp
            },
            Some(Token::Delimiter{..})
                if self.match_optional_delimiter(Delimiter::ParenLeft)
                    => {
                let exp = self.parse_simple_expression();
                self.match_required_delimiter(Delimiter::ParenRight);
                exp
            },
            Some(_) => {
                let mut inner_app = self.parse_application();
                while self.match_optional_delimiter(Delimiter::Bird) {
                    let mut outer_app = self.parse_application();
                    match &mut outer_app.exp {
                        Expression::Application{ident: _, ref mut args} => {
                            args.insert(0, inner_app.clone());
                            inner_app = outer_app
                        },
                        Expression::Reference{ident: _} => {
                            inner_app = Exp{
                                exp: Expression::Application{ident: Box::new(outer_app.clone()), args: vec![inner_app]},
                                exp_type: outer_app.exp_type.clone(),
                                token: outer_app.token.clone()
                            }
                        },
                        _ => {
                            self.error = true;
                            error!("Function chain requires valid function application: {}",
                                get_fp_from_token(&outer_app.token));
                            ()
                        }
                    }
                }
                inner_app
            },
            _ => self.make_empty_exp()
        }
    }

    fn parse_access_index(&mut self) -> i32 {
        let index = self.parse_literal();
        match index.exp {
            Expression::Lit{lit} => {
                match lit {
                    IntLit{literal} => literal,
                    _ => {
                        self.error = true;
                        error!("Tuple access requires integer literal for index: {}",
                                get_fp_from_token(&index.token));
                        -1
                    }
                }
            },
            _ => {
                self.error = true;
                error!("Tuple access requires integer literal for index: {}",
                                get_fp_from_token(&index.token));
                -1
            }
        }
    }

    fn parse_atom(&mut self) -> Exp {
        match self.curr() {
            Some(Token::Ident{ident, fp: _}) => {
                let reference = Exp{
                    exp: Expression::Reference{ident},
                    exp_type: UnknownType,
                    token: self.curr().unwrap().clone()
                };

                self.advance();

                if self.match_optional_delimiter(Delimiter::TupleAccess) {
                    let token = self.curr().unwrap().clone();
                    let access_index = self.parse_access_index() as usize;
                    let tuple_access = Exp{
                        exp: Expression::TupleAccess{
                            ident: Box::new(reference),
                            index: access_index
                        },
                        exp_type: UnknownType,
                        token: token.clone()
                    };
                    tuple_access
                } else {
                    reference
                }
            },
            Some(Token::Keyword{keyword, fp: _}) if keyword.is_builtin_function() => {
                let token = self.curr().unwrap().clone();
                self.advance();
                Exp{
                    exp: Expression::Reference{ident: keyword.to_string()},
                    exp_type: UnknownType,
                    token
                }
            },
            Some(Token::Delimiter{..})
                if self.match_optional_delimiter(Delimiter::ParenLeft) => {
                    let smp = self.parse_tuple_def_or_simple_expression();
                    self.match_required_delimiter(Delimiter::ParenRight);
                    smp
            }
            Some(_) => self.parse_literal(),
            _ => self.make_empty_exp()
        }
    }

    fn parse_literal(&mut self) -> Exp {
        let token = self.curr().unwrap();
        match self.curr() {
            Some(Token::Keyword{..}) if self.match_optional_keyword(Keyword::True) => {
                    Exp{
                        exp: Expression::Lit{lit: BoolLit{literal: true}},
                        exp_type: BoolType,
                        token: token.clone()
                    }
            },
            Some(Token::Keyword{..}) if self.match_optional_keyword(Keyword::False) => {
                    Exp{
                        exp: Expression::Lit{lit: BoolLit{literal: false}},
                        exp_type: BoolType,
                        token: token.clone()
                    }
            },
            Some(Token::Keyword{..}) if self.match_optional_keyword(Keyword::Null) => {
                    Exp{
                        exp: Expression::Lit{lit: NullLit},
                        exp_type: NullType,
                        token: token.clone()
                    }
            },
            Some(Token::Value{value, fp: _}) => {
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
                        let range_start = self.get_int_literal_value(int_literal.exp.clone());
                        let range_end = self.get_int_literal_value(int_literal_range_bound_noninclusive.exp);
                        if range_end <= range_start {
                            self.error = true;
                            error!("Integer range construction requires valid bounds: {}",
                                get_fp_from_token(&int_literal_range_bound_noninclusive.token));
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
            _ => self.make_empty_exp()
        }
    }

    fn parse_branch(&mut self) -> Exp {
        let token = self.curr().unwrap();
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
        let exp_type = if else_branch == None {
            NullType
        } else {
            else_branch.clone().unwrap().exp_type
        };

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
        let token = self.curr().unwrap().clone();
        if self.match_optional_delimiter(Delimiter::BracketRight) {
            return Exp{
                exp: Expression::ListDef{values: vec![]},
                exp_type: ListType{list_type: Box::new(UnknownType)},
                token
            }
        }

        let first_element = self.parse_simple_expression();

        if self.match_optional_delimiter(Delimiter::Comma) {
            let mut elements: Vec<Exp> = vec![first_element, self.parse_simple_expression()];
            while self.match_optional_delimiter(Delimiter::Comma) {
                elements.push(self.parse_simple_expression())
            }
            self.match_optional_delimiter(Delimiter::BracketRight);
            let list_type = if elements.is_empty() {
                UnknownType
            } else {
                elements.first().unwrap().clone().exp_type
            };
            Exp{
                exp: Expression::ListDef{values: elements},
                exp_type: ListType{list_type: Box::new(list_type)},
                token
            }
        } else if self.match_optional_delimiter(Delimiter::DenoteType) {
            let mut mapping: HashMap<Literal, Exp> = HashMap::new();
            let first_key = self.get_exp_literal(first_element.clone());
            let first_value = self.parse_simple_expression();
            mapping.insert(first_key, first_value.clone());

            while self.match_optional_delimiter(Delimiter::Comma) {
                let key_exp = self.parse_simple_expression();
                let key = self.get_exp_literal(key_exp);
                self.match_required_delimiter(Delimiter::DenoteType);
                let value = self.parse_simple_expression();
                mapping.insert(key, value);
            }
            self.match_optional_delimiter(Delimiter::BracketRight);
            Exp{
                exp: Expression::DictDef{mapping},
                exp_type: DictType{
                    key_type: Box::new(first_element.exp_type.clone()),
                    value_type: Box::new(first_value.exp_type.clone())
                },
                token
            }
        } else {
            self.match_optional_delimiter(Delimiter::BracketRight);
            let list_type = first_element.exp_type.clone();
            Exp{
                exp: Expression::ListDef{values: vec![first_element]},
                exp_type: ListType{list_type: Box::new(list_type)},
                token
            }
        }
    }

    fn parse_tuple_def_or_simple_expression(&mut self) -> Exp {
        let token= self.curr().unwrap().clone();
        let first_element = self.parse_simple_expression();

        let mut tuple_types = vec![first_element.exp_type.clone()];
        let mut tuple_elements = vec![first_element.clone()];
        while self.match_optional_delimiter(Delimiter::Comma) {
            let tuple_element = self.parse_simple_expression();
            tuple_types.push(tuple_element.exp_type.clone());
            tuple_elements.push(tuple_element);
        }
        self.match_optional_delimiter(Delimiter::ParenRight);

        if tuple_elements.len() == 1 {
            first_element
        } else {
            Exp{
                exp: Expression::TupleDef{values: tuple_elements},
                exp_type: TupleType{tuple_types},
                token
            }
        }
    }

    fn parse_schema_def(&mut self) -> Exp {
        let token = self.curr().unwrap().clone();
        self.match_required_delimiter(Delimiter::BraceLeft);
        let mut mapping: HashMap<String, Type> = HashMap::new();

        while self.match_optional_delimiter(Delimiter::Comma) ||
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

    fn parse_pattern(&mut self) -> Pattern {
        match self.curr() {
            Some(Token::Ident{ident, fp: _}) if ident != "_" => { // type case
                self.advance();
                self.match_required_delimiter(Delimiter::DenoteType);
                let case_type = self.parse_type();
                let mut predicate: Option<Exp> = None;
                if self.match_optional_keyword(Keyword::If) {
                    predicate = Some(self.parse_simple_expression())
                }
                Pattern::TypePattern{
                    ident,
                    case_type,
                    predicate
                }
            },
            Some(Token::Ident{ident, fp: _}) if ident == "_" => { // catch-all
                self.advance();
                Pattern::Any
            },
            Some(Token::Value{..}) => {
                let lit_pattern = self.parse_literal();
                match lit_pattern.exp.clone() {
                    list@Expression::ListDef{..} => Pattern::Range{range: list},
                    Expression::Lit{lit} => {
                        if self.match_optional_delimiter(Delimiter::LambdaSig) {
                            let mut literals: Vec<Literal> = vec![
                                self.get_exp_literal(lit_pattern),
                                {
                                    let second_value = self.parse_literal();
                                    self.get_exp_literal(second_value)
                                }
                            ];
                            while self.match_optional_delimiter(Delimiter::LambdaSig) {
                                let literal = self.parse_literal();
                                literals.push(self.get_exp_literal(literal))
                            }
                            Pattern::MultiLiteral{literals}
                        } else {
                            Pattern::Literal{literal: lit}
                        }
                    },
                    _ => {
                        self.error = true;
                        error!("Invalid pattern: {}",
                            get_fp_from_token(&lit_pattern.token));
                        Pattern::Any
                    }
                }
            },
            Some(_) => {
                self.error = true;
                error!("Invalid pattern: {}",
                    get_fp_from_token(&self.curr().unwrap()));
                Pattern::Any
            },
            _ => {
                self.error = true;
                error!("Invalid pattern or EOF");
                Pattern::Any
            }
        }
    }

    fn parse_case(&mut self) -> Case {
        self.match_required_keyword(Keyword::Case);
        let pattern = self.parse_pattern();
        self.match_required_delimiter(Delimiter::CaseExp);
        let case_exp = self.parse_simple_expression();
        Case{
            pattern,
            case_exp
        }
    }

    fn parse_match(&mut self) -> Exp {
        let token = self.curr().unwrap().clone();
        let value = self.parse_atom();
        self.match_required_delimiter(Delimiter::BraceLeft);
        let mut cases = vec![self.parse_case()];
        while self.match_optional_delimiter(Delimiter::Comma) {
            cases.push(self.parse_case())
        }
        self.match_required_delimiter(Delimiter::BraceRight);
        let exp_type = cases.first().unwrap().case_exp.exp_type.clone();
        Exp{
            exp: Expression::Match{
                match_exp: Box::new(value),
                cases
            },
            exp_type,
            token
        }
    }

    fn parse_parameter(&mut self) -> Parameter {
        let token = self.curr().unwrap().clone();
        let ident = self.match_ident();
        self.match_required_delimiter(Delimiter::DenoteType);
        let param_type = self.parse_type();
        Parameter{ident, param_type, token}
    }
    
    fn parse_lambda(&mut self) -> Exp {
        let token = self.curr().unwrap().clone();
        let mut params: Vec<Parameter> = vec![];
        if !self.match_optional_delimiter(Delimiter::LambdaSig) {
            while self.match_optional_delimiter(Delimiter::Comma) ||
                !self.match_optional_delimiter(Delimiter::LambdaSig) {
                params.push(self.parse_parameter());
            }
        }

        self.match_required_delimiter(Delimiter::ReturnType);
        let return_type = self.parse_type();
        self.match_required_delimiter(Delimiter::BraceLeft);
        let body = self.parse_expression();
        self.match_required_delimiter(Delimiter::BraceRight);

        let mut param_types: Vec<Type> = vec![];
        for p in params.iter() {
            param_types.push(p.param_type.clone())
        }
        let func_type = FuncType{
            param_types,
            return_type: Box::new(return_type.clone())
        };

        Exp{
            exp: Expression::Lambda{
                params,
                return_type: return_type.clone(),
                body: Box::new(body)
            },
            exp_type: func_type.clone(),
            token: token.clone()
        }
    }

    fn parse_arguments(&mut self) -> Vec<Exp> {
        let mut args: Vec<Exp> = vec![self.parse_simple_expression()];
        while self.match_optional_delimiter(Delimiter::Comma) ||
            !self.match_optional_delimiter(Delimiter::ParenRight) {
            args.push(self.parse_simple_expression())
        }
        args
    }

    fn parse_application(&mut self) -> Exp {
        let token = self.curr().unwrap().clone();
        let ident: Exp = self.parse_atom();

        match ident.exp {
            Expression::Lit{..} => ident,
            _ => {
                if self.match_optional_delimiter(Delimiter::ParenLeft) {
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
                } else {
                    ident
                }
            }
        }
    }

    fn parse_type(&mut self) -> Type {
        let first_type = match self.curr() {
            Some(Token::Keyword{..}) if self.match_optional_keyword(Keyword::Int) => IntType,
            Some(Token::Keyword{..}) if self.match_optional_keyword(Keyword::Bool) => BoolType,
            Some(Token::Keyword{..}) if self.match_optional_keyword(Keyword::Char) => CharType,
            Some(Token::Keyword{..}) if self.match_optional_keyword(Keyword::String) => StringType,
            Some(Token::Keyword{..}) if self.match_optional_keyword(Keyword::Null) => NullType,
            Some(Token::Keyword{..}) if self.match_optional_keyword(Keyword::List) => {
                self.match_required_delimiter(Delimiter::BracketLeft);
                let list_type = self.parse_type();
                self.match_required_delimiter(Delimiter::BracketRight);
                ListType{list_type: Box::new(list_type)}
            },
            Some(Token::Keyword{..}) if self.match_optional_keyword(Keyword::Dict) => {
                self.match_required_delimiter(Delimiter::BracketLeft);
                let key_type = self.parse_type();
                self.match_required_delimiter(Delimiter::DenoteType);
                let value_type = self.parse_type();
                self.match_required_delimiter(Delimiter::BracketRight);
                DictType{key_type: Box::new(key_type), value_type: Box::new(value_type)}
            },
            Some(Token::Keyword{..}) if self.match_optional_keyword(Keyword::Tuple) => {
                self.match_required_delimiter(Delimiter::BracketLeft);
                let mut tuple_types = vec![self.parse_type()];

                while self.match_optional_delimiter(Delimiter::Comma) {
                    tuple_types.push(self.parse_type())
                }
                self.match_optional_delimiter(Delimiter::BracketRight);
                TupleType{tuple_types}
            },
            Some(Token::Keyword{..}) if self.match_optional_keyword(Keyword::Schema) => SchemaType,
            Some(Token::Delimiter{..}) if self.match_optional_delimiter(Delimiter::ParenLeft) => {
                let mut param_types = vec![self.parse_type()];
                while self.match_optional_delimiter(Delimiter::Comma) {
                    param_types.push(self.parse_type())
                }
                self.match_optional_delimiter(Delimiter::ParenRight);

                self.match_required_delimiter(Delimiter::ReturnType);
                let return_type = self.parse_type();
                FuncType{param_types, return_type: Box::new(return_type)}
            }
            Some(_) => {
                self.error = true;
                error!("Invalid type signature: {}",
                    get_fp_from_token(&self.curr().unwrap()));
                UnknownType
            },
            _ => {
                self.error = true;
                error!("Invalid type signature or EOF");
                UnknownType
            }
        };

        if self.match_optional_delimiter(Delimiter::ReturnType) {
            FuncType{param_types: vec![first_type], return_type: Box::new(self.parse_type())}
        } else {
            first_type
        }
    }
}