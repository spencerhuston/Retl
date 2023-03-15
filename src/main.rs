mod scanner;
mod parser;
mod defs;
mod utils;
mod interpreter;

use std::collections::HashMap;
use std::error::Error;
use clap::Parser;
use std::path::PathBuf;
use std::fs;
use env_logger::Env;
use log::{debug, error};
use crate::defs::expression::Exp;
use crate::defs::retl_type::Type;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

use crate::scanner::scanner::Scanner;
use crate::parser::parser::Parser as RetlParser;

/// RETL script and REPL runner
#[derive(Debug, Parser)]
#[clap(name = "RETL", version = "0.1.0", author = "Spencer Huston")]
struct RetlApp {
    /// Enable debug mode (optional)
    #[clap(long, short = 'd')]
    debug: bool,

    /// Enable trace mode (optional)
    #[clap(long, short = 't')]
    trace: bool,

    /// Retl script file (optional)
    #[clap(long, short = 'f')]
    file: Option<PathBuf>
}

fn read_retl_file(path_buf: &PathBuf) -> Result<String, Box<dyn Error>> {
    let make_file_err = || -> Result<String, Box<dyn Error>> {
        let path = path_buf.display();
        let err = format!("RETL script \"{path}\" requires extension \x1b[3m.retl\x1b[0m");
        Err(err.into())
    };

    match path_buf.extension() {
        Some(ext) => {
            if ext == "retl" { Ok(fs::read_to_string(path_buf)?) }
            else { make_file_err() }
        },
        None => make_file_err()
    }
}

fn make_ast(script: &String) -> Result<Exp, Box<dyn Error>> {
    let scanner = &mut Scanner::init();
    scanner.scan(&script);

    if scanner.error {
        return Err("One more errors occurred, exiting.".into())
    }

    let parser = &mut RetlParser::init();
    parser.parse(&scanner.tokens);

    if parser.error {
        Err("One more errors occurred, exiting.".into())
    } else {
        Ok(parser.root_exp.clone())
    }
}

fn run_retl(script: &String) -> Result<(), Box<dyn Error>> {
    let interpreter = &mut Interpreter::init();
    let env = interpreter::value::Env::new();
    let result = interpreter.interpret(
        &make_ast(script)?,
        &env,
        &Type::UnknownType
    );

    if interpreter.error {
        Err("One more errors occurred, exiting.".into())
    } else {
        debug!("{:?}", result);
        Ok(())
    }
}

fn run_retl_repl() -> Result<(), Box<dyn Error>> {
    // TODO
    // Read line, if it ends with \, append to script string and newline
    // Once no \, run with entire string
    Ok(())
}

fn main() {
    let retl_args = RetlApp::parse();

    if retl_args.debug {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .format_target(false).format_timestamp(None).init();
    } else if retl_args.trace {
        env_logger::builder()
            .filter_level(log::LevelFilter::Trace)
            .format_target(false).format_timestamp(None).init();
    } else {
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .format_target(false).format_timestamp(None).init();
    }

    let result = match retl_args.file {
        Some(path_buf) => {
            match read_retl_file(&path_buf) {
                Ok(script) => run_retl(&script),
                Err(e) => Err(e)
            }
        },
        None => run_retl_repl()
    };

    match result {
        Ok(_) => (),
        Err(e) => error!("{}", e.to_string())
    }
}