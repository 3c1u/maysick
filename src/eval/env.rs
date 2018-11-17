/*
 * maysick
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
    parent:    Option<Rc<RefCell<Env>>>,
    items_var: HashMap<String, MayObject>,
    items_let: HashMap<String, MayObject>,
}

pub type EnvRef = Rc<RefCell<Env>>;

#[derive(Copy, Clone, Debug, PartialEq)]
enum VariableType {
    Let,
    Var,
    Nil,
}

impl Env {
    pub fn new_ref() -> EnvRef {
        Rc::new(
            RefCell::new(Env::new())
        )
    }

    pub fn new_parent_ref(p: EnvRef) -> EnvRef {
        Rc::new(
            RefCell::new(Env::new_parent(p))
        )
    }

    pub fn new() -> Self {
        Env {
            parent: None,
            items_var: HashMap::new(),
            items_let: HashMap::new(),
        }
    }

    pub fn new_parent(p: EnvRef) -> Self {
        Env {
            parent: Some(p),
            items_var: HashMap::new(),
            items_let: HashMap::new(),
        }
    }

    fn insert_var(&mut self, key: String, value: &MayObject) {
        self.items_var.insert(key, value.clone());
    }

    pub fn set_var(&mut self, key: String, value: &MayObject) -> Result<(), RuntimeError> {
        match self.items_let.get(&key) {
            Some(_) => Err(RuntimeError::InvalidAccessError),
            None => {
                match self.items_var.get(&key) {
                    Some(_) => Err(RuntimeError::InvalidAccessError),
                    None => {
                        self.items_var.insert(key, value.clone());
                        Ok(())
                    }
                }
            }
        }
    }

    pub fn substitute(&mut self, key: String, value: &MayObject) -> Result<(), RuntimeError> {
        let (vtype, er) = self.find_owner_mut(&key, None);

        if vtype == VariableType::Let {
            return Err(RuntimeError::InvalidAccessError);
        }

        match er {
            Some(e) => {
                e.borrow_mut().insert_var(key, value);
                Ok(())
            },
            None => {
                self.items_var.insert(key, value.clone());
                Ok(())
            }
        }
    }

    pub fn set_let(&mut self, key: String, value: &MayObject) -> Result<(), RuntimeError> {
        match self.items_let.get(&key) {
            Some(_) => Err(RuntimeError::InvalidAccessError),
            None   => { 
                self.items_let.insert(key, value.clone());
                Ok(())
            }
        }
    }

    pub fn get(&self, key: &String) -> MayObject {
        self.get_opt(key)
            .unwrap_or(MayObject::Nil)
    }

    pub fn get_opt(&self, key: &String) -> Option<MayObject> {
        match self.items_let.get(key) {
            Some(v) => Some(v.clone()),
            None => {
                match self.items_var.get(key) {
                    Some(v) => Some(v.clone()),
                    None => {
                        if let Some(ref p) = self.parent {
                            Some(p.borrow().get(key))
                        } else {
                        None
                        }
                    }
                }
            }
        }
    }

    fn find_owner_mut(&self, key: &String, me: Option<EnvRef>) -> (VariableType, Option<EnvRef>) {
        match self.items_let.get(key) {
            Some(_) => (VariableType::Let, me.clone()),
            None => {
                match self.items_var.get(key) {
                    Some(_) => (VariableType::Var, me.clone()),
                    None => {
                        if let Some(ref p) = self.parent {
                            p.borrow().find_owner_mut(key, self.parent.clone())
                        } else {
                            (VariableType::Nil, None)
                        }
                    }
                }
            }
        }
    }
}
