mod scanner;
mod defs;
mod utils;

use std::error::Error;
use clap::Parser;
use std::path::PathBuf;
use std::fs;
use log::error;

use scanner::scanner::Scanner;

/// RETL script and REPL runner
#[derive(Debug, Parser)]
#[clap(name = "RETL", version = "0.1.0", author = "Spencer Huston")]
struct RetlApp {
    /// Enable debug mode
    #[clap(long, short = 'd')]
    debug: bool,

    /// Input file (optional)
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

fn run_retl(script: &String) -> Result<(), Box<dyn Error>> {
    let scanner = &mut Scanner { tokens: vec![] };
    scanner.scan(&script);
    Ok(())
}

fn run_retl_repl() -> Result<(), Box<dyn Error>> {
    // TODO
    Ok(())
}

fn main() {
    let retl_args = RetlApp::parse();

    env_logger::init();
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