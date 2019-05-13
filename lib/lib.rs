#[macro_use]
extern crate nom;

use clap::{App, Arg, SubCommand};

use encoding_rs::*;
use std::io::{stdout, Error, ErrorKind, Write};

pub mod ast;
pub mod codegen;
pub mod eval;
pub mod lexer;
pub mod loader;
pub mod parser;
pub mod prelude;
pub mod token;

/**
 * Parse arguments as a maysick interpreter.
 *
 * This function parses command-line arguments as a maysick interpreter.
 * The executable "maysick" uses this internally.
 */
pub fn parse_args() -> Result<(), std::io::Error> {
    let matches = App::new("maysick")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("run")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .about("Runs a maysick program with interpreter")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Specify a directory which includes \"run/\"")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("compile")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .about("Compiles a maysick program into native code")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Specify a directory which includes \"run/\"")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("bin_output")
                        .short("o")
                        .long("output")
                        .value_name("FILE")
                        .help("Writes output to the file")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("emitllvm")
                        .short("e")
                        .long("emit-llvm")
                        .help("Emits an LLVM code"),
                )
                .arg(
                    Arg::with_name("shiftjis")
                        .short("j")
                        .long("shift-jis")
                        .help("Emits in Shift_JIS encoding"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        let cpath = matches
            .value_of("INPUT")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Input file is not specified."))?;
        loader::run_interpreter(cpath);
    } else if let Some(matches) = matches.subcommand_matches("compile") {
        let cpath = matches
            .value_of("INPUT")
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Input file is not specified."))?;
        let _bpath = matches.value_of("INPUT").unwrap_or("a.out");
        let sjis = matches.occurrences_of("shiftjis") > 0;
        let res = loader::compile(cpath);
        if sjis {
            let (c, _, _) = SHIFT_JIS.encode(&res);
            stdout().write_all(&c).unwrap();
        } else {
            println!("{}", res);
        }
    }

    Ok(())
}
