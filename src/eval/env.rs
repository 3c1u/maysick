/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::{Error, ErrorKind};

use eval::types::*;
use eval::object::*;

pub struct Env {
    variable: HashMap<String, Rc<MayObject>>,
    parent  : Option<Rc<RefCell<Env>>>,
}

impl Drop for Env {
    fn drop(&mut self) {
        if let Some(a) = self.parent.as_mut() {
            a.detach(&self);
            self.parent = None;
        }

        self.variable.clear();
    }
}

impl Env {
    fn detach(&mut self, child: &Env) {
        // 現時点では何もすることはないよ...
    }

    fn set_global(&mut self, name: String, obj: Rc<T>) -> Result<(), Error>
        where T: MayObject {
        if let Some(p) = self.parent.as_mut() {
            p.set_global(name, obj)
        } else {
            self.set(name, obj)
        }
    }

    fn set(&mut self, name: String, obj: Rc<T>) -> Result<(), Error>
        where T: MayObject {
        self.variable.insert(name, obj);
        Ok(())
    }

    fn get(&mut self, name: String) -> Result<Rc<T>, Error>
        where T: MayObject {
       if let Some(val) = self.variable.get(name.as_str()) {
           Ok(val)
       } else if let Some(p) = self.parent.as_mut() {
           p.get(name)
       } else {
           Err(Error::new(ErrorKind::NotFound, "Missing variable."))
       }
    }
}
