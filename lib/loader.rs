/*
 * maysick
 *
 * 2018 - murueka
 */

use std::fs::File;
use std::io::{BufReader, Read};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use crate::ast::*;
use crate::parser::*;

use crate::codegen;

use crate::eval;
use crate::eval::env::*;

fn get_token_from_directory(pbuf: &PathBuf) -> Result<String, Error> {
    let mut program = String::new();

    if let Ok(entries) = pbuf.read_dir() {
        let mut entries_: Vec<_> = entries.map(Result::unwrap).collect();
        entries_.sort_by_key(std::fs::DirEntry::path);

        for entry in entries_ {
            let p = entry.path();

            // ファイルだったら、無視
            if p.is_file() {
                continue;
            }

            let input = match p.file_name().unwrap().to_str() {
                Some(t) => t,
                None => return Err(Error::new(ErrorKind::Other, "Cannot obtain file name.")),
            };

            let input = input.trim_start_matches(|p: char| p.is_numeric() || p.is_whitespace());

            let child = get_token_from_directory(&p)?;

            match child.len() {
                0 => {
                    program.push_str(input);
                    program.push(';');
                }
                _ => {
                    program.push('{');
                    program.push_str(&child);
                    program.push('}');
                }
            }
        }
    }

    Ok(program)
}

fn obtain_program(path: &str) -> Result<Program, Error> {
    // 現在位置から"run"ディレクトリを探してみる
    let mut pbuf = PathBuf::from(path);
    let file_exists = pbuf.is_file();
    pbuf.push("run");

    //"run"がディレクトリなら、探索を始める
    let is_dir = !pbuf.is_file();
    let exists = pbuf.exists();

    if is_dir && exists {
        let tokens = get_token_from_directory(&pbuf).unwrap();
        full_parse_program(&tokens)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Failed to parse."))
    } else if !file_exists {
        // 存在しないとき
        Err(Error::new(
            ErrorKind::NotFound,
            "\"run\" directory not found.",
        ))
    } else {
        // ファイルのとき
        pbuf.pop();

        let fin = File::open(pbuf);
        let mut fin = BufReader::new(fin.unwrap());

        let mut contents = String::new();
        fin.read_to_string(&mut contents).unwrap();
        full_parse_program(&contents)
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Failed to parse."))
    }
}

pub fn run_interpreter(path: &str) {
    let p_ = obtain_program(path);
    match p_ {
        Ok(p) => {
            let e = Env::new_ref();
            eval::eval(e, p).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn compile(path: &str) -> String {
    let p_ = obtain_program(path);
    match p_ {
        Ok(p) => codegen::generate_code(p),
        Err(e) => {
            println!("Error: {}", e);
            panic!()
        }
    }
}
