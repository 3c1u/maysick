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
use antlr_rust::tree::{ParseTree, ParseTreeListener};
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
    stack_stack_stmt: Vec<Vec<Stmt>>,
    stack_expr: Vec<Expr>,
    stack_stack_expr: Vec<Vec<Expr>>,
}

use antlr_rust::parser_rule_context::ParserRuleContext;

impl ASTBuilder {
    fn construct_infix(&mut self, infix: Infix) {
        let rhs = self.stack_expr.pop().unwrap();
        let lhs = self.stack_expr.pop().unwrap();

        self.stack_expr
            .push(Expr::Infix(infix, Box::new(lhs), Box::new(rhs)))
    }
}

impl MaysickListener for ASTBuilder {
    // expr_ident
    fn exit_ExprIdent_Ident(&mut self, ctx: &ExprIdent_IdentContext) {
        let ident = if let Some(i) = ctx.IDENT() {
            i
        } else {
            return; // WTF
        }
        .get_text();
        self.stack_expr.push(Expr::Ident(ident));
    }

    fn exit_ExprIdent_NumLiteral(&mut self, ctx: &ExprIdent_NumLiteralContext) {
        // todo!("parse number literal");
        self.stack_expr.push(Expr::Literal(Literal::Integer(
            i64::from_str_radix(&ctx.LIT_NUMBER().unwrap().get_text(), 10).unwrap(),
        )));
    }

    fn exit_ExprIdent_StrLiteral(&mut self, ctx: &ExprIdent_StrLiteralContext) {
        let lit = ctx.LIT_STRING().unwrap().get_text();

        // trim parenthesis
        let lit = &lit[..lit.len() - 1][1..];

        // TODO: unescape string

        self.stack_expr
            .push(Expr::Literal(Literal::String(lit.into())));
    }

    // expr_unary
    fn exit_ExprUnary_Minus(&mut self, _ctx: &ExprUnary_MinusContext) {
        let e = self.stack_expr.pop().unwrap();
        self.stack_expr
            .push(Expr::Prefix(Prefix::Negative, Box::new(e)));
    }

    // expr_mul
    fn exit_ExprMul_Mul(&mut self, _ctx: &ExprMul_MulContext) {
        self.construct_infix(Infix::MulOp)
    }

    fn exit_ExprMul_Div(&mut self, _ctx: &ExprMul_DivContext) {
        self.construct_infix(Infix::DivOp)
    }

    fn exit_ExprMul_Mod(&mut self, _ctx: &ExprMul_ModContext) {
        self.construct_infix(Infix::ModOp)
    }

    fn exit_ExprAdd_Add(&mut self, _ctx: &ExprAdd_AddContext) {
        self.construct_infix(Infix::AddOp)
    }

    fn exit_ExprAdd_Sub(&mut self, _ctx: &ExprAdd_SubContext) {
        self.construct_infix(Infix::SubOp)
    }

    fn exit_ExprAnd_And(&mut self, _ctx: &ExprAnd_AndContext) {
        self.construct_infix(Infix::AndOp)
    }

    fn exit_ExprOr_Or(&mut self, _ctx: &ExprOr_OrContext) {
        self.construct_infix(Infix::OrOp)
    }

    fn exit_ExprOr_SymEq(&mut self, _ctx: &ExprOr_SymEqContext) {
        self.construct_infix(Infix::EqualOp)
    }

    fn exit_IfExpr(&mut self, ctx: &IfExprContext) {
        let cond = self.stack_expr.pop().unwrap();
        let _else_stmt = if ctx.else_stmt().is_some() {
            self.stack_stmt.pop()
        } else {
            None
        };
        let block = if let Some(Stmt::Block(b)) = self.stack_stmt.pop() {
            b
        } else {
            panic!("expected block");
        };

        self.stack_expr.push(Expr::If(Box::new(cond), block));
    }

    fn exit_WhileExpr(&mut self, _ctx: &WhileExprContext) {
        let cond = self.stack_expr.pop().unwrap();
        let block = if let Some(Stmt::Block(b)) = self.stack_stmt.pop() {
            b
        } else {
            panic!("expected block");
        };

        self.stack_expr.push(Expr::While(Box::new(cond), block));
    }

    fn exit_StmtAssgn(&mut self, ctx: &StmtAssgnContext) {
        let name = ctx.IDENT().unwrap().get_text();
        let value = self.stack_expr.pop().unwrap();
        self.stack_stmt.push(Stmt::Subst(name, value));
    }

    fn exit_StmtLet(&mut self, ctx: &StmtLetContext) {
        let name = ctx.IDENT().unwrap().get_text();
        let value = self.stack_expr.pop().unwrap();
        self.stack_stmt.push(Stmt::Let(name, None, value));
    }

    fn exit_StmtVar(&mut self, ctx: &StmtVarContext) {
        let name = ctx.IDENT().unwrap().get_text();
        let value = self.stack_expr.pop().unwrap();
        self.stack_stmt.push(Stmt::Var(name, None, value));
    }

    fn exit_StmtRet(&mut self, ctx: &StmtRetContext) {
        let value = if ctx.expr().is_some() {
            self.stack_expr.pop()
        } else {
            None
        };

        self.stack_stmt.push(Stmt::Return(value));
    }

    fn exit_StmtFnDef(&mut self, ctx: &StmtFnDefContext) {
        let name = ctx.IDENT(0).unwrap();
        let block = if let Some(Stmt::Block(b)) = self.stack_stmt.pop() {
            b
        } else {
            panic!("expected block");
        };

        self.stack_stmt.push(Stmt::FnDef(
            name.get_text(),
            {
                let mut arr: Vec<_> = ctx
                    .IDENT_all()
                    .iter()
                    .skip(3)
                    .map(|v| v.get_text())
                    .collect();
                arr.pop();
                arr
            },
            block,
        ));
    }

    fn exit_StmtExpr(&mut self, _ctx: &StmtExprContext) {
        let e = self.stack_expr.pop().unwrap();
        self.stack_stmt.push(Stmt::Expr(e));
    }

    fn enter_Block_NonReturn(&mut self, _ctx: &Block_NonReturnContext) {
        // FIXME: not called
        // let s = std::mem::replace(&mut self.stack_stmt, vec![]);
        // self.stack_stack_stmt.push(s);
    }

    fn exit_Block_NonReturn(&mut self, _ctx: &Block_NonReturnContext) {
        let s = self.stack_stack_stmt.pop().unwrap();
        let b = std::mem::replace(&mut self.stack_stmt, s);

        self.stack_stmt.push(Stmt::Block(b));
    }

    fn enter_FnCall(&mut self, _ctx: &FnCallContext) {
        // FIXME: not called
        // let e = std::mem::replace(&mut self.stack_expr, vec![]);
        // self.stack_stack_expr.push(e);
    }

    fn exit_FnCall(&mut self, ctx: &FnCallContext) {
        let e = self.stack_stack_expr.pop().unwrap();
        let args = std::mem::replace(&mut self.stack_expr, e);

        self.stack_expr
            .push(Expr::FnCall(ctx.IDENT().unwrap().get_text(), args));
    }

    fn exit_prog(&mut self, _ctx: &ProgContext) {
        self.program.append(&mut self.stack_stmt);
    }
}

impl ParseTreeListener for ASTBuilder {
    fn enter_every_rule(&mut self, ctx: &dyn ParserRuleContext) {
        if ctx.get_rule_index() == par::RULE_fn_call {
            let e = std::mem::replace(&mut self.stack_expr, vec![]);
            self.stack_stack_expr.push(e);
        } else if ctx.get_rule_index() == par::RULE_block {
            let s = std::mem::replace(&mut self.stack_stmt, vec![]);
            self.stack_stack_stmt.push(s);
        }
    }
}
