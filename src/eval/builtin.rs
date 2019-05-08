/*
 * maysick
 *
 * 2018 - murueka
 */

use crate::eval::object::*;
use crate::eval::runtime_error::*;

use rand::prelude::*;
use std::io::{self, Write, Read, stdin};
use std::convert::TryFrom;
use std::u32;

fn get_utf8_char() -> Option<u32> {
    let s = stdin();
    let s = s.lock();

    let mut b = s.bytes();

    if let Some(Ok(c)) = b.next() {
        let c_ = c;
        let c  = u32::from(c);

        if 0 < c && c <= 0x7F {
            // ASCII文字
            Some(c)
        } else {
            // UTF-8符号化文字
            let cnt = (!c_).leading_zeros() - 1;

            let mut v  = c & (0x3F >> cnt);

            assert!(cnt > 0, "Fragmented UTF-8 character.");
            assert!(cnt < 5, "Unicode code point that cannot represented by 32-bit value.");

            for _ in 0..cnt {
                let c = u32::from(
                        b.next().expect("Invalid UTF-8 character.")
                                .expect("Failed to read from standard input.")
                );

                v = (v << 6) | (c & 0x3F);
            }

            Some(v)
        }
    } else {
        None
    }
}

fn getchar() -> i64 {
    if let Some(c) = get_utf8_char() {
        // surrogate-pair check
        if 0xD800 <= c && c <= 0xDBFF {
            eprintln!("Warning: surrogate pairs should not be used in UTF-8 encoding.");
            if let Some(lo) = get_utf8_char() {
                return i64::from(0x10000 + (c - 0xD800) * 0x400 + (lo - 0xDC00));
            } else {
                panic!("Invalid UTF-8 character. Missing LO value.");
            }
        } else if 0xDC00 <= c && c <= 0xDFFF {
            panic!("Invalid UTF-8 character. Missing HI value.");
        }

        i64::from(c)
    } else {
        -1
    }
}

pub fn call_builtin_function(
    name: &str,
    args: &[MayObject],
) -> Result<MayObject, RuntimeError> {
    match name {
        // IO系
        "print" => {
            if args.len() == 1 {
                let s = args[0].to_raw_string()?;
                print!("{}", s);
                io::stdout().flush().unwrap(); // TODO
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
            let rnum = rng.next_u64() as i64;
            Ok(MayObject::Integer(rnum))
        }
        "getchar" => {
            let r: i64 = getchar();
            if r < 0 {
                Ok(MayObject::Nil)
            } else {
                let mut s = String::new();
                s.push(char::try_from(r as u32).unwrap());
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
                s.push(char::try_from(n).unwrap());
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
        _ => Err(RuntimeError::UnimplementedErr),
    }
}
