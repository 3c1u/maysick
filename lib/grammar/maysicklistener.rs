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
     * Enter a parse tree produced by the {@code StmtExpr_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn enter_StmtExpr_(&mut self, _ctx: &StmtExpr_Context) {}
    /**
     * Exit a parse tree produced by the {@code StmtExpr_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn exit_StmtExpr_(&mut self, _ctx: &StmtExpr_Context) {}

    /**
     * Enter a parse tree produced by the {@code StmtVar_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn enter_StmtVar_(&mut self, _ctx: &StmtVar_Context) {}
    /**
     * Exit a parse tree produced by the {@code StmtVar_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn exit_StmtVar_(&mut self, _ctx: &StmtVar_Context) {}

    /**
     * Enter a parse tree produced by the {@code StmtLet_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn enter_StmtLet_(&mut self, _ctx: &StmtLet_Context) {}
    /**
     * Exit a parse tree produced by the {@code StmtLet_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn exit_StmtLet_(&mut self, _ctx: &StmtLet_Context) {}

    /**
     * Enter a parse tree produced by the {@code StmtRet_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn enter_StmtRet_(&mut self, _ctx: &StmtRet_Context) {}
    /**
     * Exit a parse tree produced by the {@code StmtRet_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn exit_StmtRet_(&mut self, _ctx: &StmtRet_Context) {}

    /**
     * Enter a parse tree produced by the {@code StmtImport_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn enter_StmtImport_(&mut self, _ctx: &StmtImport_Context) {}
    /**
     * Exit a parse tree produced by the {@code StmtImport_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn exit_StmtImport_(&mut self, _ctx: &StmtImport_Context) {}

    /**
     * Enter a parse tree produced by the {@code StmtAssgn_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn enter_StmtAssgn_(&mut self, _ctx: &StmtAssgn_Context) {}
    /**
     * Exit a parse tree produced by the {@code StmtAssgn_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn exit_StmtAssgn_(&mut self, _ctx: &StmtAssgn_Context) {}

    /**
     * Enter a parse tree produced by the {@code StmtFnDef_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn enter_StmtFnDef_(&mut self, _ctx: &StmtFnDef_Context) {}
    /**
     * Exit a parse tree produced by the {@code StmtFnDef_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn exit_StmtFnDef_(&mut self, _ctx: &StmtFnDef_Context) {}

    /**
     * Enter a parse tree produced by the {@code StmtBlock_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn enter_StmtBlock_(&mut self, _ctx: &StmtBlock_Context) {}
    /**
     * Exit a parse tree produced by the {@code StmtBlock_}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn exit_StmtBlock_(&mut self, _ctx: &StmtBlock_Context) {}

    /**
     * Enter a parse tree produced by the {@code StmtIf}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn enter_StmtIf(&mut self, _ctx: &StmtIfContext) {}
    /**
     * Exit a parse tree produced by the {@code StmtIf}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn exit_StmtIf(&mut self, _ctx: &StmtIfContext) {}

    /**
     * Enter a parse tree produced by the {@code StmtWhile}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn enter_StmtWhile(&mut self, _ctx: &StmtWhileContext) {}
    /**
     * Exit a parse tree produced by the {@code StmtWhile}
     * labeled alternative in {@link MaysickParser#stmt}.
     * @param ctx the parse tree
     */
    fn exit_StmtWhile(&mut self, _ctx: &StmtWhileContext) {}

    /**
     * Enter a parse tree produced by the {@code StmtExpr}
     * labeled alternative in {@link MaysickParser#stmt_expr}.
     * @param ctx the parse tree
     */
    fn enter_StmtExpr(&mut self, _ctx: &StmtExprContext) {}
    /**
     * Exit a parse tree produced by the {@code StmtExpr}
     * labeled alternative in {@link MaysickParser#stmt_expr}.
     * @param ctx the parse tree
     */
    fn exit_StmtExpr(&mut self, _ctx: &StmtExprContext) {}

    /**
     * Enter a parse tree produced by the {@code StmtLet}
     * labeled alternative in {@link MaysickParser#stmt_let}.
     * @param ctx the parse tree
     */
    fn enter_StmtLet(&mut self, _ctx: &StmtLetContext) {}
    /**
     * Exit a parse tree produced by the {@code StmtLet}
     * labeled alternative in {@link MaysickParser#stmt_let}.
     * @param ctx the parse tree
     */
    fn exit_StmtLet(&mut self, _ctx: &StmtLetContext) {}

    /**
     * Enter a parse tree produced by the {@code StmtVar}
     * labeled alternative in {@link MaysickParser#stmt_var}.
     * @param ctx the parse tree
     */
    fn enter_StmtVar(&mut self, _ctx: &StmtVarContext) {}
    /**
     * Exit a parse tree produced by the {@code StmtVar}
     * labeled alternative in {@link MaysickParser#stmt_var}.
     * @param ctx the parse tree
     */
    fn exit_StmtVar(&mut self, _ctx: &StmtVarContext) {}

    /**
     * Enter a parse tree produced by the {@code StmtRet}
     * labeled alternative in {@link MaysickParser#stmt_return}.
     * @param ctx the parse tree
     */
    fn enter_StmtRet(&mut self, _ctx: &StmtRetContext) {}
    /**
     * Exit a parse tree produced by the {@code StmtRet}
     * labeled alternative in {@link MaysickParser#stmt_return}.
     * @param ctx the parse tree
     */
    fn exit_StmtRet(&mut self, _ctx: &StmtRetContext) {}

    /**
     * Enter a parse tree produced by the {@code StmtImport}
     * labeled alternative in {@link MaysickParser#stmt_import}.
     * @param ctx the parse tree
     */
    fn enter_StmtImport(&mut self, _ctx: &StmtImportContext) {}
    /**
     * Exit a parse tree produced by the {@code StmtImport}
     * labeled alternative in {@link MaysickParser#stmt_import}.
     * @param ctx the parse tree
     */
    fn exit_StmtImport(&mut self, _ctx: &StmtImportContext) {}

    /**
     * Enter a parse tree produced by the {@code StmtAssgn}
     * labeled alternative in {@link MaysickParser#stmt_assign}.
     * @param ctx the parse tree
     */
    fn enter_StmtAssgn(&mut self, _ctx: &StmtAssgnContext) {}
    /**
     * Exit a parse tree produced by the {@code StmtAssgn}
     * labeled alternative in {@link MaysickParser#stmt_assign}.
     * @param ctx the parse tree
     */
    fn exit_StmtAssgn(&mut self, _ctx: &StmtAssgnContext) {}

    /**
     * Enter a parse tree produced by the {@code StmtFnDef}
     * labeled alternative in {@link MaysickParser#stmt_fn_def}.
     * @param ctx the parse tree
     */
    fn enter_StmtFnDef(&mut self, _ctx: &StmtFnDefContext) {}
    /**
     * Exit a parse tree produced by the {@code StmtFnDef}
     * labeled alternative in {@link MaysickParser#stmt_fn_def}.
     * @param ctx the parse tree
     */
    fn exit_StmtFnDef(&mut self, _ctx: &StmtFnDefContext) {}

    /**
     * Enter a parse tree produced by the {@code FnCall}
     * labeled alternative in {@link MaysickParser#fn_call}.
     * @param ctx the parse tree
     */
    fn enter_FnCall(&mut self, _ctx: &FnCallContext) {}
    /**
     * Exit a parse tree produced by the {@code FnCall}
     * labeled alternative in {@link MaysickParser#fn_call}.
     * @param ctx the parse tree
     */
    fn exit_FnCall(&mut self, _ctx: &FnCallContext) {}

    /**
     * Enter a parse tree produced by the {@code Expr_Or_}
     * labeled alternative in {@link MaysickParser#expr}.
     * @param ctx the parse tree
     */
    fn enter_Expr_Or_(&mut self, _ctx: &Expr_Or_Context) {}
    /**
     * Exit a parse tree produced by the {@code Expr_Or_}
     * labeled alternative in {@link MaysickParser#expr}.
     * @param ctx the parse tree
     */
    fn exit_Expr_Or_(&mut self, _ctx: &Expr_Or_Context) {}

    /**
     * Enter a parse tree produced by the {@code ExprIdent_StrLiteral}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn enter_ExprIdent_StrLiteral(&mut self, _ctx: &ExprIdent_StrLiteralContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprIdent_StrLiteral}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn exit_ExprIdent_StrLiteral(&mut self, _ctx: &ExprIdent_StrLiteralContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprIdent_NumLiteral}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn enter_ExprIdent_NumLiteral(&mut self, _ctx: &ExprIdent_NumLiteralContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprIdent_NumLiteral}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn exit_ExprIdent_NumLiteral(&mut self, _ctx: &ExprIdent_NumLiteralContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprIdent_FnCall}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn enter_ExprIdent_FnCall(&mut self, _ctx: &ExprIdent_FnCallContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprIdent_FnCall}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn exit_ExprIdent_FnCall(&mut self, _ctx: &ExprIdent_FnCallContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprIdent_Paren}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn enter_ExprIdent_Paren(&mut self, _ctx: &ExprIdent_ParenContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprIdent_Paren}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn exit_ExprIdent_Paren(&mut self, _ctx: &ExprIdent_ParenContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprIdent_IfExpr}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn enter_ExprIdent_IfExpr(&mut self, _ctx: &ExprIdent_IfExprContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprIdent_IfExpr}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn exit_ExprIdent_IfExpr(&mut self, _ctx: &ExprIdent_IfExprContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprIdent_WhileExpr}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn enter_ExprIdent_WhileExpr(&mut self, _ctx: &ExprIdent_WhileExprContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprIdent_WhileExpr}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn exit_ExprIdent_WhileExpr(&mut self, _ctx: &ExprIdent_WhileExprContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprIdent_Ident}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn enter_ExprIdent_Ident(&mut self, _ctx: &ExprIdent_IdentContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprIdent_Ident}
     * labeled alternative in {@link MaysickParser#expr_ident}.
     * @param ctx the parse tree
     */
    fn exit_ExprIdent_Ident(&mut self, _ctx: &ExprIdent_IdentContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprUnary_Minus}
     * labeled alternative in {@link MaysickParser#expr_unary}.
     * @param ctx the parse tree
     */
    fn enter_ExprUnary_Minus(&mut self, _ctx: &ExprUnary_MinusContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprUnary_Minus}
     * labeled alternative in {@link MaysickParser#expr_unary}.
     * @param ctx the parse tree
     */
    fn exit_ExprUnary_Minus(&mut self, _ctx: &ExprUnary_MinusContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprUnary_Ident_}
     * labeled alternative in {@link MaysickParser#expr_unary}.
     * @param ctx the parse tree
     */
    fn enter_ExprUnary_Ident_(&mut self, _ctx: &ExprUnary_Ident_Context) {}
    /**
     * Exit a parse tree produced by the {@code ExprUnary_Ident_}
     * labeled alternative in {@link MaysickParser#expr_unary}.
     * @param ctx the parse tree
     */
    fn exit_ExprUnary_Ident_(&mut self, _ctx: &ExprUnary_Ident_Context) {}

    /**
     * Enter a parse tree produced by the {@code ExprMul_Unary_}
     * labeled alternative in {@link MaysickParser#expr_mul}.
     * @param ctx the parse tree
     */
    fn enter_ExprMul_Unary_(&mut self, _ctx: &ExprMul_Unary_Context) {}
    /**
     * Exit a parse tree produced by the {@code ExprMul_Unary_}
     * labeled alternative in {@link MaysickParser#expr_mul}.
     * @param ctx the parse tree
     */
    fn exit_ExprMul_Unary_(&mut self, _ctx: &ExprMul_Unary_Context) {}

    /**
     * Enter a parse tree produced by the {@code ExprMul_Mul}
     * labeled alternative in {@link MaysickParser#expr_mul}.
     * @param ctx the parse tree
     */
    fn enter_ExprMul_Mul(&mut self, _ctx: &ExprMul_MulContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprMul_Mul}
     * labeled alternative in {@link MaysickParser#expr_mul}.
     * @param ctx the parse tree
     */
    fn exit_ExprMul_Mul(&mut self, _ctx: &ExprMul_MulContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprMul_Div}
     * labeled alternative in {@link MaysickParser#expr_mul}.
     * @param ctx the parse tree
     */
    fn enter_ExprMul_Div(&mut self, _ctx: &ExprMul_DivContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprMul_Div}
     * labeled alternative in {@link MaysickParser#expr_mul}.
     * @param ctx the parse tree
     */
    fn exit_ExprMul_Div(&mut self, _ctx: &ExprMul_DivContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprMul_Mod}
     * labeled alternative in {@link MaysickParser#expr_mul}.
     * @param ctx the parse tree
     */
    fn enter_ExprMul_Mod(&mut self, _ctx: &ExprMul_ModContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprMul_Mod}
     * labeled alternative in {@link MaysickParser#expr_mul}.
     * @param ctx the parse tree
     */
    fn exit_ExprMul_Mod(&mut self, _ctx: &ExprMul_ModContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprAdd_Add}
     * labeled alternative in {@link MaysickParser#expr_add}.
     * @param ctx the parse tree
     */
    fn enter_ExprAdd_Add(&mut self, _ctx: &ExprAdd_AddContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprAdd_Add}
     * labeled alternative in {@link MaysickParser#expr_add}.
     * @param ctx the parse tree
     */
    fn exit_ExprAdd_Add(&mut self, _ctx: &ExprAdd_AddContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprAdd_Mul_}
     * labeled alternative in {@link MaysickParser#expr_add}.
     * @param ctx the parse tree
     */
    fn enter_ExprAdd_Mul_(&mut self, _ctx: &ExprAdd_Mul_Context) {}
    /**
     * Exit a parse tree produced by the {@code ExprAdd_Mul_}
     * labeled alternative in {@link MaysickParser#expr_add}.
     * @param ctx the parse tree
     */
    fn exit_ExprAdd_Mul_(&mut self, _ctx: &ExprAdd_Mul_Context) {}

    /**
     * Enter a parse tree produced by the {@code ExprAdd_Sub}
     * labeled alternative in {@link MaysickParser#expr_add}.
     * @param ctx the parse tree
     */
    fn enter_ExprAdd_Sub(&mut self, _ctx: &ExprAdd_SubContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprAdd_Sub}
     * labeled alternative in {@link MaysickParser#expr_add}.
     * @param ctx the parse tree
     */
    fn exit_ExprAdd_Sub(&mut self, _ctx: &ExprAdd_SubContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprAnd_Add_}
     * labeled alternative in {@link MaysickParser#expr_and}.
     * @param ctx the parse tree
     */
    fn enter_ExprAnd_Add_(&mut self, _ctx: &ExprAnd_Add_Context) {}
    /**
     * Exit a parse tree produced by the {@code ExprAnd_Add_}
     * labeled alternative in {@link MaysickParser#expr_and}.
     * @param ctx the parse tree
     */
    fn exit_ExprAnd_Add_(&mut self, _ctx: &ExprAnd_Add_Context) {}

    /**
     * Enter a parse tree produced by the {@code ExprAnd_And}
     * labeled alternative in {@link MaysickParser#expr_and}.
     * @param ctx the parse tree
     */
    fn enter_ExprAnd_And(&mut self, _ctx: &ExprAnd_AndContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprAnd_And}
     * labeled alternative in {@link MaysickParser#expr_and}.
     * @param ctx the parse tree
     */
    fn exit_ExprAnd_And(&mut self, _ctx: &ExprAnd_AndContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprOr_Or}
     * labeled alternative in {@link MaysickParser#expr_or}.
     * @param ctx the parse tree
     */
    fn enter_ExprOr_Or(&mut self, _ctx: &ExprOr_OrContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprOr_Or}
     * labeled alternative in {@link MaysickParser#expr_or}.
     * @param ctx the parse tree
     */
    fn exit_ExprOr_Or(&mut self, _ctx: &ExprOr_OrContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprOr_SymEq}
     * labeled alternative in {@link MaysickParser#expr_or}.
     * @param ctx the parse tree
     */
    fn enter_ExprOr_SymEq(&mut self, _ctx: &ExprOr_SymEqContext) {}
    /**
     * Exit a parse tree produced by the {@code ExprOr_SymEq}
     * labeled alternative in {@link MaysickParser#expr_or}.
     * @param ctx the parse tree
     */
    fn exit_ExprOr_SymEq(&mut self, _ctx: &ExprOr_SymEqContext) {}

    /**
     * Enter a parse tree produced by the {@code ExprOr_And_}
     * labeled alternative in {@link MaysickParser#expr_or}.
     * @param ctx the parse tree
     */
    fn enter_ExprOr_And_(&mut self, _ctx: &ExprOr_And_Context) {}
    /**
     * Exit a parse tree produced by the {@code ExprOr_And_}
     * labeled alternative in {@link MaysickParser#expr_or}.
     * @param ctx the parse tree
     */
    fn exit_ExprOr_And_(&mut self, _ctx: &ExprOr_And_Context) {}

    /**
     * Enter a parse tree produced by the {@code Block_NonReturn}
     * labeled alternative in {@link MaysickParser#block}.
     * @param ctx the parse tree
     */
    fn enter_Block_NonReturn(&mut self, _ctx: &Block_NonReturnContext) {}
    /**
     * Exit a parse tree produced by the {@code Block_NonReturn}
     * labeled alternative in {@link MaysickParser#block}.
     * @param ctx the parse tree
     */
    fn exit_Block_NonReturn(&mut self, _ctx: &Block_NonReturnContext) {}

    /**
     * Enter a parse tree produced by the {@code Block_WithReturn}
     * labeled alternative in {@link MaysickParser#block}.
     * @param ctx the parse tree
     */
    fn enter_Block_WithReturn(&mut self, _ctx: &Block_WithReturnContext) {}
    /**
     * Exit a parse tree produced by the {@code Block_WithReturn}
     * labeled alternative in {@link MaysickParser#block}.
     * @param ctx the parse tree
     */
    fn exit_Block_WithReturn(&mut self, _ctx: &Block_WithReturnContext) {}

    /**
     * Enter a parse tree produced by the {@code IfExpr}
     * labeled alternative in {@link MaysickParser#if_expr}.
     * @param ctx the parse tree
     */
    fn enter_IfExpr(&mut self, _ctx: &IfExprContext) {}
    /**
     * Exit a parse tree produced by the {@code IfExpr}
     * labeled alternative in {@link MaysickParser#if_expr}.
     * @param ctx the parse tree
     */
    fn exit_IfExpr(&mut self, _ctx: &IfExprContext) {}

    /**
     * Enter a parse tree produced by the {@code ElseStmt}
     * labeled alternative in {@link MaysickParser#else_stmt}.
     * @param ctx the parse tree
     */
    fn enter_ElseStmt(&mut self, _ctx: &ElseStmtContext) {}
    /**
     * Exit a parse tree produced by the {@code ElseStmt}
     * labeled alternative in {@link MaysickParser#else_stmt}.
     * @param ctx the parse tree
     */
    fn exit_ElseStmt(&mut self, _ctx: &ElseStmtContext) {}

    /**
     * Enter a parse tree produced by the {@code WhileExpr}
     * labeled alternative in {@link MaysickParser#while_expr}.
     * @param ctx the parse tree
     */
    fn enter_WhileExpr(&mut self, _ctx: &WhileExprContext) {}
    /**
     * Exit a parse tree produced by the {@code WhileExpr}
     * labeled alternative in {@link MaysickParser#while_expr}.
     * @param ctx the parse tree
     */
    fn exit_WhileExpr(&mut self, _ctx: &WhileExprContext) {}
}
