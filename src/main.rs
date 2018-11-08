/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

#[macro_use]
extern crate nom;

use std::env;

pub mod directory;
pub mod lexer;

fn main() {
    let cpath = env::args().nth(1).unwrap_or(".".to_string());

    match directory::fetch_runcode(&cpath.as_str()) {
        Ok(res) => println!("{:?}", res),
        Err(_) => println!("Hello, world!"),
    }
}
