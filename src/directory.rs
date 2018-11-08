/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub enum MaysickCode {
    Block(Vec<MaysickCode>),
    Line(String),
}

use self::MaysickCode::*;

pub fn get_current_path() -> Result<Box<Path>, Error> {
    let current_path = Path::new(".");
    let full_pathbuf = current_path.canonicalize();

    return Ok(Box::from(full_pathbuf?.as_path()));
}

fn fetch_dir_at(pbuf: &PathBuf) -> MaysickCode {
    let mut tokens: Vec<MaysickCode> = Vec::new();

    if let Ok(entries) = pbuf.read_dir() {
        for entry in entries {
            if let Ok(entry) = entry {
                let p = entry.path();

                // ファイルだったら、無視
                if p.is_file() {
                    continue;
                }

                let token = p.file_name().unwrap().to_str().unwrap();
                let child = fetch_dir_at(&p);

                tokens.push(Line(token.to_string()));

                match child {
                    Block(_) => tokens.push(child.clone()),
                    Line(s) => tokens.push(Block(vec![Line(s.to_string())])),
                }
            }
        }
    }

    match tokens.len() {
        1 => tokens[0].clone(),
        _ => Block(tokens),
    }
}

pub fn fetch_runcode(path: &str) -> Result<MaysickCode, Error> {
    // 現在位置から"run"ディレクトリを探してみる
    let mut pbuf = PathBuf::from(path);
    pbuf.push("run");

    //"run"がディレクトリなら、探索を始める
    let is_dir = !pbuf.is_file();
    let exists = pbuf.exists();
    if is_dir && exists {
        Ok(fetch_dir_at(&pbuf))
    } else if !exists {
        // 存在しないとき
        Err(Error::new(ErrorKind::Other, "\"run\" directory not found."))
    } else {
        // ディレクトリのとき
        Err(Error::new(ErrorKind::Other, "\"run\" is not a directory."))
    }
}
