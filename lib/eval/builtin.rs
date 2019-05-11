/*
 * maysick
 *
 * 2018 - murueka
 */

use crate::eval::object::*;
use crate::eval::runtime_error::*;

use crate::prelude::*;

use rand::prelude::*;
use std::convert::TryFrom;
use std::io::{self, Write};
use std::u32;

pub fn call_builtin_function(name: &str, args: &[MayObject]) -> Result<MayObject, RuntimeError> {
    match name {
        // IO系
        "print" => {
            if args.len() == 1 {
                let s = args[0].to_raw_string()?;
                print!("{}", s);
                io::stdout().flush().map_err(|_| RuntimeError::IOError)?;
                Ok(MayObject::Nil)
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "println" => {
            if args.len() == 1 {
                let s = args[0].to_raw_string()?;
                println!("{}", s);
                Ok(MayObject::Nil)
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "random" => {
            let mut rng = rand::thread_rng();
            let rnum = (rng.next_u64() >> 1) as i64;
            Ok(MayObject::Integer(rnum))
        }
        "getchar" => {
            let r: i64 = getchar();
            if r < 0 {
                Ok(MayObject::Nil)
            } else {
                let mut s = String::new();
                s.push(char::try_from(r as u32).map_err(|_| RuntimeError::CastError)?);
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
        // 比較
        "lt" => {
            if args.len() == 2 {
                let s = args[0].to_raw_integer()?;
                let n = args[1].to_raw_integer()?;
                Ok(MayObject::Bool(s < n))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "gt" => {
            if args.len() == 2 {
                let s = args[0].to_raw_integer()?;
                let n = args[1].to_raw_integer()?;
                Ok(MayObject::Bool(s > n))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "eqlt" => {
            if args.len() == 2 {
                let s = args[0].to_raw_integer()?;
                let n = args[1].to_raw_integer()?;
                Ok(MayObject::Bool(s <= n))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "eqgt" => {
            if args.len() == 2 {
                let s = args[0].to_raw_integer()?;
                let n = args[1].to_raw_integer()?;
                Ok(MayObject::Bool(s >= n))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "eq" => {
            if args.len() == 2 {
                let s = &args[0];
                let n = &args[1];
                Ok(MayObject::Bool(s == n))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "neq" => {
            if args.len() == 2 {
                let s = &args[0];
                let n = &args[1];
                Ok(MayObject::Bool(s != n))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        // 演算子
        "mul" => {
            if args.len() == 2 {
                let s = args[0].to_raw_integer()?;
                let n = args[1].to_raw_integer()?;
                Ok(MayObject::Integer(s * n))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "div" => {
            if args.len() == 2 {
                let s = args[0].to_raw_integer()?;
                let n = args[1].to_raw_integer()?;
                Ok(MayObject::Integer(s / n))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "and" => {
            if args.len() == 2 {
                if let MayObject::Bool(sb) = args[0] {
                    let nb = args[1].to_raw_bool()?;
                    Ok(MayObject::Bool(sb && nb))
                } else {
                    let s = args[0].to_raw_integer()?;
                    let n = args[1].to_raw_integer()?;
                    Ok(MayObject::Integer(s & n))
                }
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "or" => {
            if args.len() == 2 {
                if let MayObject::Bool(sb) = args[0] {
                    let nb = args[1].to_raw_bool()?;
                    Ok(MayObject::Bool(sb || nb))
                } else {
                    let s = args[0].to_raw_integer()?;
                    let n = args[1].to_raw_integer()?;
                    Ok(MayObject::Integer(s & n))
                }
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "not" => {
            if args.len() == 1 {
                Ok(MayObject::Bool(!args[0].to_raw_bool()?))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        // キャストなど
        "to_string" => {
            if args.len() == 1 {
                let s = &args[0];
                s.to_string()
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "integer_as_hex" => {
            if args.len() == 1 {
                let n = args[0].to_raw_integer()?;
                Ok(MayObject::String(format!("{:X}", n)))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "nil" => Ok(MayObject::Nil),
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
                let n = args[0].to_raw_integer()? as u32;
                let mut s = String::new();
                s.push(char::try_from(n).map_err(|_| RuntimeError::CastError)?);
                Ok(MayObject::String(s))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        "slen" => {
            if args.len() == 1 {
                let n = args[0].to_raw_string()?;
                Ok(MayObject::Integer(n.len() as i64))
            } else {
                Err(RuntimeError::ArgumentErr)
            }
        }
        _ => map_graphic_builtin(name, args),
    }
}
