/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

#[macro_use]
extern crate nom;

use std::env;

pub mod ast;
// pub mod eval;
pub mod lexer;
pub mod loader;
pub mod parser;
pub mod token;

fn main() {
    let cpath = env::args().nth(1).unwrap_or(".".to_string());

    match loader::run(&cpath.as_str()) {
        Ok(_) => (),
        Err(_) => println!("Hello, world!"),
    }
}
