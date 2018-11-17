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
                    if let MayObject::String(s) = args[0].to_string()? {
                        println!("{}", s);
                        Ok(MayObject::Nil)
                    } else {
                        Err(RuntimeError::UnknownErr)
                    }
                } else {
                    Err(RuntimeError::ArgumentErr)
                }
            }
            _ => {
                println!("'{}' called with args:\n{:#?}.", name, args);
                Err(RuntimeError::UnimplementedErr)
            }
        }
    }
}
