/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use nom::*;
use std::iter::*;

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
    BinaryFnOp(String), // <tested>

    // ->
    Arrow, // <not-tested>

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

// traits for nom
impl InputLength for Token {
    fn input_len(&self) -> usize {
        1
    }
}

// tokens & traits for nom

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Tokens<'a> {
    pub tokens: &'a [Token],
}

impl<'a> Tokens<'a> {
    pub fn new(v: &'a Vec<Token>) -> Self {
        Tokens { tokens: v.as_slice() }
    }

    pub fn from_slice(s: &'a [Token]) -> Self {
        Tokens { tokens: s }
    }

    pub fn empty() -> Self {
        Tokens { tokens: &[] }
    }

    pub fn item_at(&self, i: usize) -> Token {
        self.tokens[i].clone()
    }
}

impl<'a> AtEof for Tokens<'a> {
    fn at_eof(&self) -> bool {
        true
    }
}

impl<'a> InputLength for Tokens<'a> {
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl<'a> InputTake for Tokens<'a> {
    fn take(&self, count: usize) -> Self {
        Tokens::from_slice(&self.tokens[0..count])
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (p, s) = self.tokens.split_at(count);
        (Tokens::from_slice(s), Tokens::from_slice(p))
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item     = &'a Token;
    type RawItem  = Token;
    type Iter     = Enumerate<std::slice::Iter<'a, Token>>;
    type IterElem = std::slice::Iter<'a, Token>;

    fn iter_indices(&self) -> Enumerate<std::slice::Iter<'a, Token>> {
        self.tokens.iter().enumerate()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.tokens.iter()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::RawItem) -> bool,
    {
        self.tokens.iter().position(|b| predicate(b.clone()))
    }

    fn slice_index(&self, count: usize) -> Option<usize> {
        if self.tokens.len() >= count {
            Some(count)
        } else {
            None
        }
    }
}
