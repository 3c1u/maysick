/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

pub mod env;
pub mod object;
pub mod runtime_error;

use ast::*;
use eval::env::*;
use eval::object::*;
use eval::runtime_error::*;

pub fn eval(e: &mut Env, p: Program) -> Result<MayObject, RuntimeError> {
    for s in p {
        let r = eval_stmt(e, &s)?;
        if let MayObject::RetVal(v) = r {
            return Ok(v.as_ref().clone());
        }
    }
    // fallback
    Ok(MayObject::Nil)
}

pub fn eval_literal(l: &Literal) -> Result<MayObject, RuntimeError> {
    match l {
        Literal::Integer(n) => Ok(MayObject::Integer(*n)),
        Literal::String(s) => Ok(MayObject::String(s.clone())),
        _ => Err(RuntimeError::UnimplementedErr),
    }
}

pub fn eval_infix(i: &Infix, a: MayObject, b: MayObject) -> Result<MayObject, RuntimeError> {
    match i {
        Infix::AddOp => MayObject::operator_add(&a, &b),
        Infix::SubOp => MayObject::operator_sub(&a, &b),
        Infix::ModOp => MayObject::operator_mod(&a, &b),
        _ => Err(RuntimeError::UnimplementedErr),
    }
}

pub fn eval_expr(e: &mut Env, x: &Expr) -> Result<MayObject, RuntimeError> {
    match x {
        Expr::Ident(n) => Ok(e.get(&n)),
        Expr::Literal(l) => eval_literal(l),
        Expr::Infix(i, a, b) => eval_infix(i, eval_expr(e, a.as_ref())?, eval_expr(e, b.as_ref())?),
        Expr::FnCall(n, arg) => {
            let a = arg
                .into_iter()
                .map(|a: &Expr| eval_expr(e, a).unwrap())
                .collect();
            e.call(n.clone(), a)
        }
        _ => Err(RuntimeError::UnimplementedErr),
    }
}

pub fn eval_stmt(e: &mut Env, s: &Stmt) -> Result<MayObject, RuntimeError> {
    match s {
        Stmt::FnDef(i, a, b) => {
            let f = MayObject::Fn(a.clone(), b.clone());
            e.set(i.clone(), &f);
            Ok(MayObject::Nil)
        }
        Stmt::Let(i, _t, x) => {
            let r = eval_expr(e, x)?;
            e.set(i.clone(), &r);
            Ok(r)
        }
        Stmt::Return(orv) => match orv {
            Some(rv) => Ok(MayObject::RetVal(Box::new(eval_expr(e, rv)?))),
            None => Ok(MayObject::RetVal(Box::new(MayObject::Nil))),
        },
        Stmt::Expr(x) => eval_expr(e, x),
        _ => Err(RuntimeError::UnimplementedErr),
    }
}
