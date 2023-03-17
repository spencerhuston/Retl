use crate::defs::expression::{Exp, Expression, Literal};
use crate::defs::retl_type::type_conforms;
use crate::defs::retl_type::Type;
use crate::interpreter::value::{Value, Env, Val};
use crate::scanner::token::{get_fp_from_token, make_empty_token, Token};

pub struct Interpreter {
    pub error: bool,
    root_exp: Exp
}

pub fn make_error_value() -> Value { Value{value: Val::Error, val_type: Type::UnknownType} }

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

    pub fn interpret(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        match &exp.exp {
            Expression::Lit{..} => self.interpret_literal(&exp, expected_type),
            Expression::Let{..} => self.interpret_let(&exp, env, expected_type),
            Expression::Primitive{..} => self.interpret_primitive(&exp, env, expected_type),
            Expression::Reference{..} => self.interpret_reference(&exp, env, expected_type),
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

    fn interpret_let(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        match &exp.exp {
            Expression::Let{ident, let_type, let_exp, after_let_exp} => {
                let resolved_exp = self.interpret(let_exp, env, let_type);
                env.insert(ident.clone(), resolved_exp);
                match &**after_let_exp {
                    Some(after) => self.interpret(after, env, expected_type),
                    _ => Value{value: Val::NullValue, val_type: Type::NullType}
                }
            },
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
            }
        }
    }

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

    fn interpret_primitive(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        match &exp.exp {
            Expression::Primitive{operator, left, right} => {
                let op_type = exp.exp_type.clone();
                let left_value = self.interpret(left, env, &op_type);
                let right_value = self.interpret(right, env, &op_type);
                let result = operator.interpret(&left_value, &right_value);
                type_conforms(&result.val_type, expected_type, &exp.token);
                result
            },
            _ => {
                // TODO: Throw error here, invalid expression
                make_error_value()
            }
        }
    }

    fn interpret_reference(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
        match &exp.exp {
            Expression::Reference{ident} => {
                let ref_value = env.get(ident).clone();
                match ref_value {
                    Some(r) => {
                        type_conforms(&r.val_type, expected_type, &exp.token);
                        r.clone()
                    },
                    _ => {
                        // TODO: Throw error here, reference does not exist
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

    fn interpret_branch(&mut self, exp: &Exp, env: &mut Env, expected_type: &Type) -> Value {
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