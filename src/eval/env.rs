/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use std::cell::*;
use std::collections::*;
use std::rc::*;

use eval::object::*;
use eval::runtime_error::*;

use std::io;
use libc::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    items: HashMap<String, MayObject>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            parent: None,
            items: HashMap::new(),
        }
    }

    pub fn new_parent(p: Rc<RefCell<Env>>) -> Self {
        Env {
            parent: Some(p),
            items: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: &MayObject) {
        self.items.insert(key, value.clone());
    }

    pub fn get(&self, key: &String) -> MayObject {
        match self.items.get(key) {
            Some(v) => v.clone(),
            None => {
                if let Some(ref p) = self.parent {
                    p.borrow().get(key)
                } else {
                    MayObject::Nil
                }
            }
        }
    }

    pub fn call(&mut self, name: String, args: Vec<MayObject>) -> Result<MayObject, RuntimeError> {
        match name.as_str() {
            "println" => {
                if args.len() == 1 {
                    let s = args[0].to_raw_string()?;
                    println!("{}", s);
                    Ok(MayObject::Nil)
                } else {
                    Err(RuntimeError::ArgumentErr)
                }
            },
            "print" => {
                if args.len() == 1 {
                    let s = args[0].to_raw_string()?;
                    print!("{}", s);
                    Ok(MayObject::Nil)
                } else {
                    Err(RuntimeError::ArgumentErr)
                }
            },
            "random" => {
                let mut r: i64;
                unsafe {
                  r = rand() as i64;
                }
                Ok(MayObject::Integer(r))
            },
            "char_at" => {
                if args.len() == 2 {
                    let s = args[0].to_raw_string()?;
                    let n = args[1].to_raw_integer()?;
                    let c = s.chars().nth(n as usize).ok_or(RuntimeError::OutOfIndexError)?;
                    Ok(MayObject::Integer(c as i64))
                } else {
                    Err(RuntimeError::ArgumentErr)
                }
            },
            "char_from" => {
                if args.len() == 1 {
                    let n = args[0].to_raw_integer()? as u8;
                    let mut s = String::new();
                    s.push(n as char);
                    Ok(MayObject::String(s))
                } else {
                    Err(RuntimeError::ArgumentErr)
                }
            },
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
            },
            "readline" => {
                let mut s = String::new();
                match io::stdin().read_line(&mut s) {
                  Ok(n) => Ok(MayObject::String(s[0..n - 1].to_string())),
                  Err(_) => Err(RuntimeError::IOError),
                }
            },
            _ => {
                println!("'{}' called with args:\n{:#?}.", name, args);
                Err(RuntimeError::UnimplementedErr)
            }
        }
    }
}
