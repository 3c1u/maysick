/*
 * maysick
 *
 * 2018 - murueka
 */

/*!
 * Abstract Syntax Tree for maysick.
 */

/** A collection of statements. */
pub type Program = Block;

/** A collection of statements, surrounded by {} */
pub type Block = Vec<Stmt>;

/** Idenifier */
pub type Ident = String;

/** Type identifer */
pub type Type = Ident;

/** Function argument list */
pub type Args = Vec<Ident>;

/** A function argument. */
#[derive(Clone, Debug, PartialEq)]
pub struct Arg {
    pub type_name: Type,
    pub arg_name: Ident,
}

/** # Statement
## Types
A Statement has those types:
* Function definition
* Let/Var statment
* Substitution
* Return
* Single expression (commonly terminated by a semicolon)
*/
#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    /**
    Function definition.

    ```plaintext
    fn ident([args]) { expr... }
    ```
    */
    FnDef(Ident, Args, Block),

    /**
    Let statement.

    ```plaintext
    let ident = expr;
    ```
    */
    Let(Ident, Option<Type>, Expr),
    /**
    Var statement.

    ```plaintext
    var ident = expr;
    ```
    */
    Var(Ident, Option<Type>, Expr),
    /**
    Substitution.

    ```plaintext
    ident = expr;
    ```

    Let statment is immutable, which cannot be changed by substitutions which is described below,
    while Var statement is mutable, and it is possible to reassign any value to the variable.

    ```plaintext
    let a = 3;
    var b = 4;

    b = 5; // okay
    a = 4; // error
    ```

    Unlike Rust, this is checked by runtime, not by parser nor compiler.
     */
    Subst(Ident, Expr),

    Return(Option<Expr>),
    Expr(Expr),

    Import(String),
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
