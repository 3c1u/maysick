/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

#[cfg(test)]
pub(crate) mod test;

use crate::ast::*;
use crate::token::*;

use crate::grammar::maysicklexer as lex;
use crate::grammar::maysicklistener as lis;
use crate::grammar::maysickparser as par;

use lex::MaysickLexer;
use lis::MaysickListener;
use par::*;

use antlr_rust::input_stream::InputStream;
use antlr_rust::token_source::TokenSource;
use antlr_rust::tree::{ErrorNode, ParseTree, ParseTreeListener, TerminalNode, TerminalNodeCtx};
use antlr_rust::{common_token_stream::CommonTokenStream, errors::ANTLRError};

pub fn token_maysick_line(_input: &str) -> Result<Vec<Token>, ()> {
    todo!()
}

pub fn token_line(_input: &str) -> Result<Vec<Token>, ()> {
    todo!()
}

pub fn parse_expr(_tokens: &[Token]) -> Result<Expr, ()> {
    todo!()
}

pub fn parse_program(_tokens: &[Token]) -> Result<Program, ()> {
    todo!()
}

pub fn full_parse_program(input: &str) -> Result<Program, ANTLRError> {
    let lexer = MaysickLexer::new(Box::new(InputStream::new(input.into())));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = MaysickParser::new(Box::new(token_source));
    let builder = ASTBuilder::default();

    let id = parser.add_parse_listener(Box::new(builder));
    parser.prog().expect("failed to parse");

    let m = parser.remove_parse_listener(id);

    Ok(m.program)
}

#[derive(Debug, Default)]
struct ASTBuilder {
    program: Program,
    stack_stmt: Vec<Stmt>,
    stack_expr: Vec<Expr>,
}

use antlr_rust::parser_rule_context::ParserRuleContext;

impl MaysickListener for ASTBuilder {}

impl ParseTreeListener for ASTBuilder {}

// statement
