/*
 * maysick
 *
 * 2018 - murueka
 */

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // EOF
    Eof, // <tested>

    // fn, let, var, if, else, while, return
    KFn,     // <tested>
    KLet,    // <tested>
    KVar,    // <tested>
    KIf,     // <tested>
    KElse,   // <tested>
    KWhile,  // <tested>
    KReturn, // <tested>
    KImport, // <not-tested>

    Ident(String), // <tested>

    // integer.
    Integer(i64), // <tested>
    // 'yah string'
    String(String), // <tested>

    // =
    EqualOp,
    // +
    AddOp, // <not-tested>
    // -
    SubOp, // <not-tested>
    // %
    ModOp, // <not-tested>
    // `mul`, `div`, etc...
    MulOp,              // <not-tested>
    DivOp,              // <not-tested>
    AndOp,              // <not-tested>
    OrOp,               // <not-tested>
    NotOp,              // <not-tested>
    BinaryFnOp(String), // <tested>

    // ->
    Arrow, // <not-tested>

    // ::
    DoubleColon,

    // .
    Dot,

    // (
    LParen, // <not-tested>
    // )
    RParen, // <not-tested>

    // ,
    Comma, // <not-tested>

    // :
    Colon, // <not-tested>

    // Line
    EndLine, //equivalent to ;

    // Block
    LBlock, //equivalent to {
    RBlock, //equivalent to }
}
