/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use nom::types::CompleteStr;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use parser::*;
use lexer::*;
use token::*;

pub fn get_current_path() -> Result<Box<Path>, Error> {
    let current_path = Path::new(".");
    let full_pathbuf = current_path.canonicalize();

    return Ok(Box::from(full_pathbuf?.as_path()));
}

fn get_token_from_directory(pbuf: &PathBuf) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();

    if let Ok(entries) = pbuf.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
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
    }

    Ok(tokens)
}

pub fn run(path: &str) -> Result<(), Error> {
    // 現在位置から"run"ディレクトリを探してみる
    let mut pbuf = PathBuf::from(path);
    pbuf.push("run");

    //"run"がディレクトリなら、探索を始める
    let is_dir = !pbuf.is_file();
    let exists = pbuf.exists();
    if is_dir && exists {
        let tokens = get_token_from_directory(&pbuf)?;
        let prog = parse_program(Tokens::new(&tokens));
        println!("{:#?}", prog);
        Ok(())
    } else if !exists {
        // 存在しないとき
        Err(Error::new(ErrorKind::Other, "\"run\" directory not found."))
    } else {
        // ディレクトリのとき
        Err(Error::new(ErrorKind::Other, "\"run\" is not a directory."))
    }
}
