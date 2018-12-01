/*
 * maysick
 *
 * 2018 - murueka
 */

pub type Program = Block;
pub type Block = Vec<Stmt>;
pub type Ident = String;
pub type Type = Ident;

pub type Args = Vec<Ident>;

#[derive(Clone, Debug, PartialEq)]
pub struct Arg {
    pub type_name: Type,
    pub arg_name: Ident,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    FnDef(Ident, Args, Block),
    Let(Ident, Option<Type>, Expr),
    Var(Ident, Option<Type>, Expr),

    Subst(Ident, Expr),

    Return(Option<Expr>),
    Expr(Expr),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Prefix {
    Positive,
    Negative,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Infix {
    EqualOp,
    AddOp,
    SubOp,
    ModOp,
    DivOp,
    MulOp,
    AndOp,
    OrOp,
    BinFnOp,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Ident(Ident),
    Literal(Literal),

    Prefix(Prefix, Box<Expr>),
    Infix(Infix, Box<Expr>, Box<Expr>),

    FnCall(Ident, Vec<Expr>),

    If(Box<Expr>, Block),
    While(Box<Expr>, Block),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(i64),
    String(String),
    Bool(bool),
}
