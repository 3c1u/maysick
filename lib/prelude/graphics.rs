/*
 * maysick
 *
 * 2018 - murueka
 */

use crate::eval::object::*;
use crate::eval::runtime_error::*;

pub fn map_graphic_builtin(name: &str, _args: &[MayObject]) -> Result<MayObject, RuntimeError> {
    match name {
        _ => Err(RuntimeError::UnimplementedErr),
    }
}
