/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

#[macro_use]
extern crate nom;
extern crate libc;

use std::env;

pub mod ast;
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

fn main() {
    unsafe {
        let t = cl::clock();
        libc::srand(t as u32);
    }

    let cpath = env::args().nth(1).unwrap_or(".".to_string());
    loader::run(&cpath.as_str());
}
