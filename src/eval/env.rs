/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub type Type = String;

pub struct Env {
    variable: HashMap<String, Rc<MsObject>>,
    parent  : Option<RefCell<Env>>,
}

impl Env {

}