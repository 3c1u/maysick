/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

#[macro_use]
extern crate nom;

use std::env;

pub mod loader;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod eval;

fn main() {
    let cpath = env::args().nth(1).unwrap_or(".".to_string());

    match loader::run(&cpath.as_str()) {
        Ok(res) => println!("{:?}", res),
        Err(_) => println!("Hello, world!"),
    }
}
