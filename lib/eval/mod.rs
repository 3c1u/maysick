/*
 * maysick
 *
 * 2018 - murueka
 */

pub mod builtin;
pub mod env;
pub mod object;
pub mod runtime_error;

use crate::ast::*;

use crate::eval::env::*;
use crate::eval::object::*;
use crate::eval::runtime_error::*;

pub fn eval(e: EnvRef, p: Program) -> Result<MayObject, RuntimeError> {
    for s in p {
        let r = eval_stmt(e.clone(), &s)?;
        if let MayObject::RetVal(v) = r {
            return Ok(v.as_ref().clone());
        }
    }
    // fallback
    Ok(MayObject::Nil)
}

pub fn eval_block(e: EnvRef, p: Program) -> Result<MayObject, RuntimeError> {
    let scope = Env::new_parent_ref(e);
    for s in p {
        let r = eval_stmt(scope.clone(), &s)?;
        if let MayObject::RetVal(_) = r {
            return Ok(r.clone());
        }
    }
    // fallback
    Ok(MayObject::Nil)
}

pub fn eval_literal(l: &Literal) -> Result<MayObject, RuntimeError> {
    match l {
        Literal::Integer(n) => Ok(MayObject::Integer(*n)),
        Literal::String(s) => Ok(MayObject::String(s.clone())),
        Literal::Bool(b) => Ok(MayObject::Bool(*b)),
    }
}

pub fn eval_infix(i: Infix, a: MayObject, b: MayObject) -> Result<MayObject, RuntimeError> {
    match i {
        Infix::AddOp => MayObject::operator_add(&a, &b),
        Infix::SubOp => MayObject::operator_sub(&a, &b),
        Infix::ModOp => MayObject::operator_mod(&a, &b),
        Infix::MulOp => MayObject::operator_mul(&a, &b),
        Infix::DivOp => MayObject::operator_div(&a, &b),
        _ => Err(RuntimeError::UnimplementedErr),
    }
}

pub fn eval_expr(_e: EnvRef, x: &Expr) -> Result<MayObject, RuntimeError> {
    match x {
        Expr::Ident(n) => {
            let e = &mut _e.borrow_mut();
            Ok(e.get(&n))
        }
        Expr::Literal(l) => eval_literal(l),
        Expr::Infix(i, a, b) => eval_infix(
            *i,
            eval_expr(_e.clone(), a.as_ref())?,
            eval_expr(_e.clone(), b.as_ref())?,
        ),
        Expr::FnCall(n, arg) => {
            let a: Result<Vec<MayObject>, _> = arg
                .iter()
                .map(|a: &Expr| eval_expr(_e.clone(), a))
                .collect();
            let a = a?;
            let e = &mut _e.borrow_mut();
            match builtin::call_builtin_function(&n, &a) {
                retval @ Ok(_) => retval,
                Err(_) => {
                    // println!("'{}' called with args:\n{:#?}.", name, args);
                    if let MayObject::Fn(argname, block) = e.get(&n) {
                        let scope = Env::new_parent_ref(_e.clone());
                        for i in 0..argname.len() {
                            scope
                                .borrow_mut()
                                .set_var(argname[i].clone(), &a[i].clone())?;
                        }
                        eval(scope, block)
                    } else {
                        Err(RuntimeError::UnimplementedErr)
                    }
                }
            }
        }
        Expr::If(c, b) => {
            if eval_expr(_e.clone(), c.as_ref())?.to_raw_bool()? {
                eval_block(_e, b.clone())
            } else {
                Ok(MayObject::Nil)
            }
        }
        Expr::While(c, b) => {
            while eval_expr(_e.clone(), c.as_ref())?.to_raw_bool()? {
                eval_block(_e.clone(), b.clone())?;
            }
            Ok(MayObject::Nil)
        }
        _ => Err(RuntimeError::UnimplementedErr),
    }
}

pub fn eval_stmt(e: EnvRef, s: &Stmt) -> Result<MayObject, RuntimeError> {
    match s {
        Stmt::Block(..) => todo!(),
        Stmt::FnDef(i, args, block) => {
            let fndef = MayObject::Fn(args.clone(), block.clone());
            e.borrow_mut().set_let(i.clone(), &fndef)?;
            Ok(MayObject::Nil)
        }
        Stmt::Let(i, _t, x) => {
            let right_val = eval_expr(e.clone(), x)?;
            e.borrow_mut().set_let(i.clone(), &right_val)?;
            Ok(right_val)
        }
        Stmt::Var(i, _t, x) => {
            let right_val = eval_expr(e.clone(), x)?;
            e.borrow_mut().set_var(i.clone(), &right_val)?;
            Ok(right_val)
        }
        Stmt::Subst(i, x) => {
            let right_val = eval_expr(e.clone(), x)?;
            e.borrow_mut().substitute(i.clone(), &right_val)?;
            Ok(right_val)
        }
        Stmt::Return(orv) => match orv {
            Some(rv) => Ok(MayObject::RetVal(Box::new(eval_expr(e, rv)?))),
            None => Ok(MayObject::RetVal(Box::new(MayObject::Nil))),
        },
        Stmt::Expr(x) => eval_expr(e, x),
        Stmt::Import(_i) => {
            // debug!("New import: {:?}", i);
            Ok(MayObject::Nil)
        }
    }
}
