use crate::defs::expression::{Exp, Expression, Literal};
use crate::defs::retl_type::Type;
use crate::interpreter::value::Value;
use crate::scanner::token::{make_empty_token, Token};

pub struct Interpreter {
    pub error: bool,
    root_exp: Exp
}

impl Interpreter {
    pub fn init() -> Interpreter {
        Interpreter{
            error: false,
            root_exp: Exp{
                exp: Expression::Empty,
                exp_type: Type::NullType,
                token: make_empty_token()
            }
        }
    }

    pub fn interpret(&mut self, exp: &Exp) -> Value {
        match exp.exp.clone() {
            Expression::Lit{..} => self.interpret_literal(&exp),
            Expression::Branch{..} => self.interpret_branch(&exp),
            _ => {
                // TODO: Throw error here, invalid expression
                Value::Error
            }
        }
    }

    fn interpret_literal(&mut self, exp: &Exp) -> Value {
        match &exp.exp {
            Expression::Lit{lit} => {
                match lit {
                    Literal::IntLit{literal} => Value::IntValue{value: literal.clone()},
                    Literal::BoolLit{literal} => Value::BoolValue{value: literal.clone()},
                    Literal::CharLit{literal} => Value::CharValue{value: literal.clone()},
                    Literal::StringLit{literal} => Value::StringValue{value: literal.clone()},
                    Literal::NullLit => Value::NullValue,
                }
            },
            _ => {
                // TODO: Throw error here, invalid expression
                Value::Error
            }
        }
    }

    // fn interpret_let(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_alias(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_lambda(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_match(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_primitive(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_reference(&mut self) -> Value {
    //
    // }

    fn interpret_branch(&mut self, exp: &Exp) -> Value {
        match &exp.exp {
            Expression::Branch{
                condition,
                if_branch,
                else_branch
            } => {
                // Type check condition is boolean
                // Type check if and else return same type
                match self.interpret(&**condition) {
                    Value::BoolValue{value} => {
                        if value {
                            self.interpret(&if_branch)
                        } else {
                            match &**else_branch {
                                Some(else_exp) => self.interpret(&else_exp),
                                _ => Value::NullValue
                            }
                        }
                    },
                    _ => {
                        // TODO: Throw error here, not a valid condition
                        Value::Error
                    }
                }
            },
            _ => {
                // TODO: Throw error here, invalid expression
                Value::Error
            }
        }
    }

    // fn interpret_list_def(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_tuple_def(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_tuple_access(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_dict_def(&mut self) -> Value {
    //
    // }
    //
    // fn interpret_schema_def(&mut self) -> Value {
    //
    // }
}