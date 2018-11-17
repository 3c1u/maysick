/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use nom::types::CompleteStr;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use lexer::*;
use parser::*;
use token::*;

use eval;
use eval::env::*;

pub fn get_current_path() -> Result<Box<Path>, Error> {
    let current_path = Path::new(".");
    let full_pathbuf = current_path.canonicalize();

    return Ok(Box::from(full_pathbuf?.as_path()));
}

fn get_token_from_directory(pbuf: &PathBuf) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();

    if let Ok(entries) = pbuf.read_dir() {
        let mut entries_: Vec<_> = entries.map(|d| d.unwrap()).collect();
        entries_.sort_by_key(|dir| dir.path());

        for entry in entries_ {
            let p = entry.path();

            // ファイルだったら、無視
            if p.is_file() {
                continue;
            }

            let token = match p.file_name().unwrap().to_str() {
                Some(t) => t,
                None => return Err(Error::new(ErrorKind::Other, "Cannot obtain file name.")),
            };

            let parsed = match token_maysick_line(CompleteStr::from(token)) {
                Ok((r, res)) => {
                    if r.0.is_empty() {
                        res
                    } else {
                        println!("Unconsumed token\"{}\"", r);
                        return Err(Error::new(ErrorKind::Other, "Unconsumed token."));
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Failed to lex due to lexer error.",
                    ));
                }
            };

            let child = get_token_from_directory(&p)?;

            tokens.extend(parsed);

            match child.len() {
                0 => tokens.push(Token::EndLine),
                _ => {
                    tokens.push(Token::LBlock);
                    tokens.extend(child);
                    tokens.push(Token::RBlock);
                }
            }
        }
    }

    Ok(tokens)
}

pub fn run(path: &str) {
    // 現在位置から"run"ディレクトリを探してみる
    let mut pbuf = PathBuf::from(path);
    pbuf.push("run");

    //"run"がディレクトリなら、探索を始める
    let is_dir = !pbuf.is_file();
    let exists = pbuf.exists();
    if is_dir && exists {
        let tokens = get_token_from_directory(&pbuf).unwrap();
        let prog = parse_program(Tokens::new(&tokens));
        let mut e = Env::new();
        match prog {
            Ok((_, p)) => {
                eval::eval(&mut e, p).unwrap();
            }
            Err(e) => println!("Error: {:#?}", e),
        }
    } else if !exists {
        // 存在しないとき
        println!("\"run\" directory not found.");
    } else {
        // ディレクトリのとき
        println!("\"run\" is not a directory.");
    }
}
