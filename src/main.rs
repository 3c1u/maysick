/*
 * maysick
 *
 * 2018 - murueka
 */

#[macro_use]
extern crate nom;
extern crate clap;
extern crate libc;

use clap::{App, Arg, SubCommand};

use std::io::{Error, ErrorKind};

pub mod ast;
pub mod codegen;
pub mod eval;
pub mod lexer;
pub mod loader;
pub mod parser;
pub mod token;

mod cl {
    extern "C" {
        pub fn clock() -> ::libc::clock_t;
    }
}

fn parse_args() -> Result<(), std::io::Error> {
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
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        let cpath = matches.value_of("INPUT").ok_or_else(|| Error::new(
            ErrorKind::InvalidInput,
            "Input file is not specified.",
        ))?;
        loader::run_interpreter(cpath);
    } else if let Some(matches) = matches.subcommand_matches("compile") {
        let _cpath = matches.value_of("INPUT").ok_or_else(|| Error::new(
            ErrorKind::InvalidInput,
            "Input file is not specified.",
        ))?;
        let _bpath = matches.value_of("INPUT").unwrap_or("a.out");
        let _lflag = matches.occurrences_of("e") > 0;
        // Yah build!!
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    // libcの乱数生成器の初期化
    unsafe {
        let t = cl::clock();
        libc::srand(t as u32);
    }
    parse_args()
}
