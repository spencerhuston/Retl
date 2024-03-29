mod scanner;
mod parser;
mod defs;
mod utils;
mod interpreter;
mod builtin;

use log::{error, trace};
use std::error::Error;
use clap::Parser;
use std::path::PathBuf;
use std::{fs, io, panic};
use std::io::Write;
use substring::Substring;
use crate::builtin::builtin::Builtin;

use crate::scanner::scanner::Scanner;
use crate::parser::parser::Parser as RetlParser;
use crate::interpreter::interpreter::Interpreter;

use crate::defs::expression::Exp;
use crate::defs::retl_type::Type;
use crate::interpreter::value::Value;

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
        trace!("SCANNER ERROR");
        return Err("One more errors occurred, exiting.".into())
    }

    let parser = &mut RetlParser::init();
    parser.parse(&scanner.tokens);

    if parser.error {
        trace!("PARSER ERROR");
        Err("One more errors occurred, exiting.".into())
    } else {
        Ok(parser.root_exp.clone())
    }
}

fn run_retl(script: &String) -> Result<(), Box<dyn Error>> {
    let builtin = Builtin::init();
    let mut env = builtin.load_builtins(&interpreter::value::Env::new());
    let interpreter = &mut Interpreter::init(&builtin);
    let result = interpreter.interpret(
        &make_ast(script)?,
        &mut env,
        &Type::UnknownType
    );

    if interpreter.error {
        trace!("INTERPRETER ERROR");
        Err("One more errors occurred, exiting.".into())
    } else {
        trace!("{:?}", result);
        Ok(())
    }
}

fn run_retl_repl() -> Result<(), Box<dyn Error>> {
    let builtin = Builtin::init();
    let mut env = builtin.load_builtins(&interpreter::value::Env::new());
    let interpreter = &mut Interpreter::init(&builtin);
    let mut repl_input = String::new();
    println!("Retl REPL\n=========");
    loop {
        trace!("REPL Input: {}", repl_input);
        let mut line = String::new();
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut line).expect("Expected REPL input");
        trace!("Line: {}", line);
        line = line.trim().to_string();
        if line == "quit" {
            break;
        } else if line.ends_with('\\') {
            line = line.substring(0, line.len() - 1).to_string();
            trace!("Trimmed line: {}", line);
            repl_input.push_str(&*line);
            repl_input.push_str("\n")
        } else if line.ends_with(';') {
            trace!("Trimmed line: {}", line);
            repl_input.push_str(&*line);
            repl_input.push_str("\n")
        } else {
            repl_input.push_str(&*line);
            let result = interpreter.interpret(&make_ast(&repl_input)?, &mut env, &Type::UnknownType);
            trace!("{:?}", result);
            repl_input.clear()
        }
    }
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

    panic::catch_unwind(|| {
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
    }).expect("Fatal error occurred")
}