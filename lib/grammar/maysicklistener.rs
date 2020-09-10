#![allow(non_snake_case)]
// Generated from lib/grammar/Maysick.g4 by ANTLR 4.8
use super::maysickparser::*;
use antlr_rust::tree::ParseTreeListener;

use std::any::Any;

pub trait MaysickListener: ParseTreeListener {
    /**
     * Enter a parse tree produced by {@link MaysickParser#prog}.
     * @param ctx the parse tree
     */
    fn enter_prog(&mut self, _ctx: &ProgContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#prog}.
     * @param ctx the parse tree
     */
    fn exit_prog(&mut self, _ctx: &ProgContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn enter_stmt(&mut self, _ctx: &StmtContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn exit_stmt(&mut self, _ctx: &StmtContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#stmt_let}.
     * @param ctx the parse tree
     */
    fn enter_stmt_let(&mut self, _ctx: &Stmt_letContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#stmt_let}.
     * @param ctx the parse tree
     */
    fn exit_stmt_let(&mut self, _ctx: &Stmt_letContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#stmt_var}.
     * @param ctx the parse tree
     */
    fn enter_stmt_var(&mut self, _ctx: &Stmt_varContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#stmt_var}.
     * @param ctx the parse tree
     */
    fn exit_stmt_var(&mut self, _ctx: &Stmt_varContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#stmt_return}.
     * @param ctx the parse tree
     */
    fn enter_stmt_return(&mut self, _ctx: &Stmt_returnContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#stmt_return}.
     * @param ctx the parse tree
     */
    fn exit_stmt_return(&mut self, _ctx: &Stmt_returnContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#stmt_import}.
     * @param ctx the parse tree
     */
    fn enter_stmt_import(&mut self, _ctx: &Stmt_importContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#stmt_import}.
     * @param ctx the parse tree
     */
    fn exit_stmt_import(&mut self, _ctx: &Stmt_importContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#stmt_assign}.
     * @param ctx the parse tree
     */
    fn enter_stmt_assign(&mut self, _ctx: &Stmt_assignContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#stmt_assign}.
     * @param ctx the parse tree
     */
    fn exit_stmt_assign(&mut self, _ctx: &Stmt_assignContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#stmt_fn_def}.
     * @param ctx the parse tree
     */
    fn enter_stmt_fn_def(&mut self, _ctx: &Stmt_fn_defContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#stmt_fn_def}.
     * @param ctx the parse tree
     */
    fn exit_stmt_fn_def(&mut self, _ctx: &Stmt_fn_defContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#fn_call}.
     * @param ctx the parse tree
     */
    fn enter_fn_call(&mut self, _ctx: &Fn_callContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#fn_call}.
     * @param ctx the parse tree
     */
    fn exit_fn_call(&mut self, _ctx: &Fn_callContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#expr}.
     * @param ctx the parse tree
     */
    fn enter_expr(&mut self, _ctx: &ExprContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#expr}.
     * @param ctx the parse tree
     */
    fn exit_expr(&mut self, _ctx: &ExprContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn enter_expr_ident(&mut self, _ctx: &Expr_identContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn exit_expr_ident(&mut self, _ctx: &Expr_identContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#expr_unary}.
     * @param ctx the parse tree
     */
    fn enter_expr_unary(&mut self, _ctx: &Expr_unaryContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#expr_unary}.
     * @param ctx the parse tree
     */
    fn exit_expr_unary(&mut self, _ctx: &Expr_unaryContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#expr_mul}.
     * @param ctx the parse tree
     */
    fn enter_expr_mul(&mut self, _ctx: &Expr_mulContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#expr_mul}.
     * @param ctx the parse tree
     */
    fn exit_expr_mul(&mut self, _ctx: &Expr_mulContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#expr_add}.
     * @param ctx the parse tree
     */
    fn enter_expr_add(&mut self, _ctx: &Expr_addContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#expr_add}.
     * @param ctx the parse tree
     */
    fn exit_expr_add(&mut self, _ctx: &Expr_addContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#expr_and}.
     * @param ctx the parse tree
     */
    fn enter_expr_and(&mut self, _ctx: &Expr_andContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#expr_and}.
     * @param ctx the parse tree
     */
    fn exit_expr_and(&mut self, _ctx: &Expr_andContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#expr_or}.
     * @param ctx the parse tree
     */
    fn enter_expr_or(&mut self, _ctx: &Expr_orContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#expr_or}.
     * @param ctx the parse tree
     */
    fn exit_expr_or(&mut self, _ctx: &Expr_orContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#block}.
     * @param ctx the parse tree
     */
    fn enter_block(&mut self, _ctx: &BlockContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#block}.
     * @param ctx the parse tree
     */
    fn exit_block(&mut self, _ctx: &BlockContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#if_expr}.
     * @param ctx the parse tree
     */
    fn enter_if_expr(&mut self, _ctx: &If_exprContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#if_expr}.
     * @param ctx the parse tree
     */
    fn exit_if_expr(&mut self, _ctx: &If_exprContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#else_stmt}.
     * @param ctx the parse tree
     */
    fn enter_else_stmt(&mut self, _ctx: &Else_stmtContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#else_stmt}.
     * @param ctx the parse tree
     */
    fn exit_else_stmt(&mut self, _ctx: &Else_stmtContext) {}

    /**
     * Enter a parse tree produced by {@link MaysickParser#while_expr}.
     * @param ctx the parse tree
     */
    fn enter_while_expr(&mut self, _ctx: &While_exprContext) {}
    /**
     * Exit a parse tree produced by {@link MaysickParser#while_expr}.
     * @param ctx the parse tree
     */
    fn exit_while_expr(&mut self, _ctx: &While_exprContext) {}
}
