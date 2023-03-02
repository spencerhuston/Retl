mod scanner;
mod defs;
mod utils;

use std::error::Error;
use clap::Parser;
use std::path::PathBuf;
use std::fs;

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

fn run_retl(script: &String, debug: bool) -> Result<(), Box<dyn Error>> {
    println!("{}", script);

    Ok(())
}

fn run_retl_repl(debug: bool) -> Result<(), Box<dyn Error>> {
    // TODO
    Ok(())
}

fn main() {
    let retl_args = RetlApp::parse();

    let debug = retl_args.debug;
    let result = match retl_args.file {
        Some(path_buf) => {
            match read_retl_file(&path_buf) {
                Ok(script) => run_retl(&script, debug),
                Err(e) => Err(e)
            }
        },
        None => run_retl_repl(debug)
    };

    match result {
        Ok(_) => (),
        Err(e) => println!("\x1b[31mERROR: {e}\x1b[0m")
    }
}