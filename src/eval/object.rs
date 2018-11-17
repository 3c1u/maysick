/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use ast::*;
use eval::runtime_error::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MayObject {
    Integer(i64),
    String(String),

    Fn(Args, Block),
    NativeFn(NativeFunction),

    RetVal(Box<MayObject>),

    Nil,
}

pub type NativeFunction = fn(Vec<MayObject>) -> Vec<MayObject>;

impl MayObject {
    pub fn from_integer(i: i64) -> Self {
        MayObject::Integer(i)
    }

    pub fn from_str(s: &str) -> Self {
        MayObject::String(s.to_string())
    }

    pub fn from_string(s: String) -> Self {
        MayObject::String(s)
    }

    pub fn to_raw_string(&self) -> Result<String, RuntimeError> {
      if let MayObject::String(r) = self.to_string()? {
        Ok(r)
      } else {
        Err(RuntimeError::UnknownErr)
      }
    }

    pub fn to_raw_integer(&self) -> Result<i64, RuntimeError> {
      if let MayObject::Integer(r) = self.to_integer()? {
        Ok(r)
      } else {
        Err(RuntimeError::UnknownErr)
      }
    }

    pub fn to_integer(&self) -> Result<Self, RuntimeError> {
        match self {
            MayObject::Integer(_) => Ok(self.clone()),
            MayObject::String(s) => s
                .parse::<i64>()
                .ok()
                .map(|s: i64| MayObject::Integer(s))
                .ok_or(RuntimeError::CastError),
            MayObject::Nil => Ok(MayObject::Integer(0)),
            _ => Err(RuntimeError::CastError),
        }
    }

    pub fn to_string(&self) -> Result<Self, RuntimeError> {
        match self {
            MayObject::Integer(a) => Ok(MayObject::String(a.to_string())),
            MayObject::String(_) => Ok(self.clone()),
            MayObject::Fn(_, _) => Ok(MayObject::String("fn() {}".to_string())),
            MayObject::NativeFn(_) => Ok(MayObject::String(
                "fn() {\n  [native function]\n}".to_string(),
            )),
            MayObject::Nil => Ok(MayObject::String("(Nil)".to_string())),
            _ => Err(RuntimeError::CastError),
        }
    }

    pub fn operator_add(a: &MayObject, b: &MayObject) -> Result<MayObject, RuntimeError> {
        if let MayObject::Integer(va) = a {
            match b {
                MayObject::Integer(vb) => Ok(MayObject::Integer(va + vb)),
                _ => Err(RuntimeError::IncompatibleTypeError),
            }
        } else if let MayObject::String(sa) = a {
            match b {
                MayObject::String(sb) => {
                    let mut sr = String::new();
                    sr.push_str(sa);
                    sr.push_str(sb);
                    Ok(MayObject::String(sr))
                }
                _ => Err(RuntimeError::IncompatibleTypeError),
            }
        } else {
            Err(RuntimeError::IncompatibleTypeError)
        }
    }

    pub fn operator_sub(a: &MayObject, b: &MayObject) -> Result<MayObject, RuntimeError> {
        if let MayObject::Integer(va) = a {
            match b {
                MayObject::Integer(vb) => Ok(MayObject::Integer(va - vb)),
                _ => Err(RuntimeError::IncompatibleTypeError),
            }
        } else {
            Err(RuntimeError::IncompatibleTypeError)
        }
    }

    pub fn operator_mod(a: &MayObject, b: &MayObject) -> Result<MayObject, RuntimeError> {
        if let MayObject::Integer(va) = a {
            match b {
                MayObject::Integer(vb) => Ok(MayObject::Integer(va % vb)),
                _ => Err(RuntimeError::IncompatibleTypeError),
            }
        } else {
            Err(RuntimeError::IncompatibleTypeError)
        }
    }
}
