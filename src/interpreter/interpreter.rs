use crate::defs::expression::{Exp, Expression, Literal};
use crate::defs::retl_type::type_conforms;
use crate::defs::retl_type::Type;
use crate::interpreter::value::{Value, Env, Val};
use crate::scanner::token::{get_fp_from_token, make_empty_token, Token};

pub struct Interpreter {
    pub error: bool,
    root_exp: Exp
}

fn make_error_value() -> Value { Value{value: Val::Error, val_type: Type::UnknownType} }

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

    // TODO: For REPL
    // fn run(&mut self, script: &String) -> Value {
    //     let env = interpreter::value::Env::new();
    //     let result = interpreter.interpret(
    //         &make_ast(script)?,
    //         &env,
    //         &Type::UnknownType
    //     );
    // }

    pub fn interpret(&mut self, exp: &Exp, env: &Env, expected_type: &Type) -> Value {
        match &exp.exp {
            Expression::Lit{..} => self.interpret_literal(&exp, &expected_type),
            //Expression::Let{..} => self.interpret_let(&exp, env),
            Expression::Branch{..} => self.interpret_branch(&exp, env, expected_type),
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
            }
        }
    }

    fn interpret_literal(&mut self, exp: &Exp, expected_type: &Type) -> Value {
        type_conforms(&exp.exp_type, expected_type, &exp.token);
        match &exp.exp {
            Expression::Lit{lit} => {
                match lit {
                    Literal::IntLit{literal} =>
                        Value{value: Val::IntValue{value: literal.clone()}, val_type: Type::IntType},
                    Literal::BoolLit{literal} =>
                        Value{value: Val::BoolValue{value: literal.clone()}, val_type: Type::BoolType},
                    Literal::CharLit{literal} =>
                        Value{value: Val::CharValue{value: literal.clone()}, val_type: Type::CharType},
                    Literal::StringLit{literal} =>
                        Value{value: Val::StringValue{value: literal.clone()}, val_type: Type::StringType},
                    Literal::NullLit =>
                        Value{value: Val::NullValue, val_type: Type::NullType}
                }
            },
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
            }
        }
    }

    // fn interpret_let(&mut self, exp: &Exp, env: Env, expected_type: Type) -> Value {
    //     match &exp.exp {
    //         l@Expression::Let{..} => {
    //
    //             Value::Error
    //         },
    //         _ => {
    //             // TODO: Throw error here, invalid expression
    //             make_error_value()
    //         }
    //     }
    // }

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

    fn interpret_branch(&mut self, exp: &Exp, env: &Env, expected_type: &Type) -> Value {
        match &exp.exp {
            Expression::Branch{
                condition,
                if_branch,
                else_branch
            } => {
                // Type check if and else return same type
                match &**else_branch {
                    Some(else_exp) => type_conforms(&if_branch.exp_type, &else_exp.exp_type, &exp.token),
                    _ => type_conforms(&if_branch.exp_type, &Type::NullType, &exp.token)
                };

                match self.interpret(&**condition, env, &Type::BoolType).value {
                    Val::BoolValue{value} => {
                        if value {
                            self.interpret(&if_branch, env, expected_type)
                        } else {
                            match &**else_branch {
                                Some(else_exp) => self.interpret(&else_exp, env, expected_type),
                                _ => Value{value: Val::NullValue, val_type: Type::NullType }
                            }
                        }
                    },
                    _ => {
                        // TODO: Throw error here, not a valid condition
                        make_error_value()
                    }
                }
            },
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
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