/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use env::*;
use std::rc::*;

use eval::env::*;
use eval::object::*;

pub type Args    = Vec<Rc<MayObject>>;
pub type VoidPtr = *mut u8;

pub enum MayType {
    Integer,
    String,
    Function,
}

pub trait MayFunction {
    fn call(args: Args, env: Env);
    fn as_obj(&self) -> Box<MayObject>;
}

#[derive(Copy, Clone, Debug)]
pub struct MayInteger {
    pub number: i64,
}

impl MayObject for MayInteger {
    fn get_type() -> MayType { MayType::Integer }

    fn cast_to(&self, target: MayType) -> Option<MayObject>{
        if targe == MayType::Integer {
            Some(Box::new(self))
        }
    }

    fn operator_add(&self, a: MayInteger) -> Option<MayInteger> {
        Some(
            MayInteger {
                number: self.number + a.number
            }
        )
    }

    fn operator_sub(&self, a: MayInteger) -> Option<MayInteger> {
        Some(
            MayInteger {
                number: self.number - a.number
            }
        )
    }

    fn operator_mod(&self, a: MayInteger) -> Option<MayInteger> {
        Some(
            MayInteger {
                number: self.number % a.number
            }
        )
    }
}

pub trait MayObject {
    // 型に関するエトセトラ
    fn get_type() -> MayType;
    fn cast_to(&self, target: MayType) -> Option<Box<MayObject>>;

    // オペレーター
    fn operator_add(&self, a: T) -> Option<U>
        where T: MayObject, U: MayObject;
    fn operator_sub(&self, a: T) -> Option<U>
        where T: MayObject, U: MayObject;
    fn operator_mod(&self, a: T) -> U
        where T: MayObject, U: MayObject;
}
