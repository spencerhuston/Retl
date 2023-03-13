use crate::defs::expression::Exp;

pub struct Interpreter {
    error: bool,
    root_exp: Exp
}