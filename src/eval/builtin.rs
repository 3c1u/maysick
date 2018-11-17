/*
 * maysick
 *
 * 2018 - murueka
 */

use eval::object::*;
use eval::runtime_error::*;

use libc::*;
use std::io;

pub fn call_builtin_function(name: &String, args: &Vec<MayObject>) -> Result<MayObject, RuntimeError> {
    match name.as_str() {
        "println" => {
            if args.len() == 1 {
                let s = args[0].to_raw_string()?;
                println!("{}", s);
                Ok(MayObject::Nil)
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "print" => {
            if args.len() == 1 {
                let s = args[0].to_raw_string()?;
                print!("{}", s);
                Ok(MayObject::Nil)
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "random" => {
            let mut r: i64;
            unsafe {
                r = rand() as i64;
            }
            Ok(MayObject::Integer(r))
        }
        "char_at" => {
            if args.len() == 2 {
                let s = args[0].to_raw_string()?;
                let n = args[1].to_raw_integer()?;
                let c = s
                    .chars()
                    .nth(n as usize)
                    .ok_or(RuntimeError::OutOfIndexError)?;
                Ok(MayObject::Integer(c as i64))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "char_from" => {
            if args.len() == 1 {
                let n = args[0].to_raw_integer()? as u8;
                let mut s = String::new();
                s.push(n as char);
                Ok(MayObject::String(s))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "getchar" => {
            let mut r: i32;
            unsafe {
                r = getchar();
            }
            if r < 0 {
                Ok(MayObject::Nil)
            } else {
                let mut s = String::new();
                s.push((r as u8) as char);
                Ok(MayObject::String(s))
            }
        }
        "readline" => {
            let mut s = String::new();
            match io::stdin().read_line(&mut s) {
                Ok(n) => Ok(MayObject::String(s[0..n - 1].to_string())),
                Err(_) => Err(RuntimeError::IOError),
            }
        }
        _ => {
            Err(RuntimeError::UnimplementedErr)
        }
    }
}
