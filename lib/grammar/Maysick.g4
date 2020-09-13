grammar Maysick;

prog
    : (stmt)* EOF
    ;

stmt
    : stmt_expr                                   # StmtExpr_
    | stmt_var                                    # StmtVar_
    | stmt_let                                    # StmtLet_
    | stmt_return                                 # StmtRet_
    | stmt_import                                 # StmtImport_
    | stmt_assign                                 # StmtAssgn_
    | stmt_fn_def                                 # StmtFnDef_
    | block                                       # StmtBlock_
    | if_expr                                     # StmtIf
    | while_expr                                  # StmtWhile
    ;

stmt_expr
    : expr SYM_SEMICOLON                          # StmtExpr
    ;

stmt_let
    : LET IDENT SYM_EQ expr SYM_SEMICOLON         # StmtLet
    ;

stmt_var
    : VAR IDENT SYM_EQ expr SYM_SEMICOLON         # StmtVar
    ;

stmt_return
    : RETURN expr? SYM_SEMICOLON                  # StmtRet
    ;

stmt_import
    : IMPORT LIT_STRING SYM_SEMICOLON             # StmtImport
    ;

stmt_assign
    : IDENT '=' expr SYM_SEMICOLON                # StmtAssgn
    ;

stmt_fn_def /* fn ident(a b c ..) { } */
    : FN IDENT SYM_LPAREN IDENT* SYM_RPAREN block # StmtFnDef
    ;

fn_call
    : IDENT SYM_LPAREN expr* SYM_RPAREN           # FnCall
    ;

expr
    : expr_or                                     # Expr_Or_
    ;

expr_ident
    : LIT_STRING                                  # ExprIdent_StrLiteral
    | LIT_NUMBER                                  # ExprIdent_NumLiteral
    | fn_call                                     # ExprIdent_FnCall
    | SYM_LPAREN expr SYM_RPAREN                  # ExprIdent_Paren
    | if_expr                                     # ExprIdent_IfExpr
    | while_expr                                  # ExprIdent_WhileExpr
    | IDENT                                       # ExprIdent_Ident
    ;

expr_unary
    : SYM_SUB expr_ident                          # ExprUnary_Minus
    | expr_ident                                  # ExprUnary_Ident_
    ;

expr_mul
    : expr_unary                                  # ExprMul_Unary_
    | expr_mul SYM_MUL expr_unary                 # ExprMul_Mul
    | expr_mul SYM_DIV expr_unary                 # ExprMul_Div
    | expr_mul SYM_MOD expr_unary                 # ExprMul_Mod
    ;

expr_add
    : expr_mul                                    # ExprAdd_Mul_
    | expr_add SYM_ADD expr_mul                   # ExprAdd_Add
    | expr_add SYM_SUB expr_mul                   # ExprAdd_Sub
    ;

expr_and
    : expr_add                                    # ExprAnd_Add_
    | expr_and SYM_AND expr_add                   # ExprAnd_And
    ;

expr_or
    : expr_and                                    # ExprOr_And_
    | expr_or SYM_OR expr_and                     # ExprOr_Or
    | expr_and SYM_EQ expr_or                     # ExprOr_Eq
    ;

block
    : SYM_LCURLY (stmt)* SYM_RCURLY               # Block_NonReturn
    | SYM_LCURLY (stmt)* expr SYM_RCURLY          # Block_WithReturn
    ;

if_expr
    : IF SYM_LPAREN expr SYM_RPAREN
        block else_stmt?                          # IfExpr
    ;

else_stmt
    : ELSE block                                  # ElseStmt
    ;

while_expr
    : WHILE SYM_LPAREN expr SYM_RPAREN block      # WhileExpr
    ;

/* lexer */
IF
    : 'if'
    ;

ELSE
    : 'else'
    ;

WHILE
    : 'while'
    ;

FN
    : 'fn'
    ;

LET
    : 'let'
    ;

VAR
    : 'var'
    ;

RETURN
    : 'return'
    ;

IMPORT
    : 'import'
    ;

SYM_LPAREN
    : '('
    ;

SYM_RPAREN
    : ')'
    ;

SYM_LBOX
    : '['
    ;

SYM_RBOX
    : ']'
    ;

SYM_LCURLY
    : '{'
    ;

SYM_RCURLY
    : '}'
    ;

SYM_EQ
    : '='
    ;

SYM_CMPEQ
    : '=='
    ;

SYM_MOD
    : '`mod`'
    | '%'
    ;

SYM_AND
    : '`and`'
    | '&'
    ;

SYM_OR
    : '`or`'
    | '|'
    ;

SYM_DIV
    : '`div`'
    | '/'
    ;

SYM_MUL
    : '`mul`'
    | '*'
    ;

SYM_ADD
    : '+'
    ;

SYM_SUB
    : '-'
    ;

SYM_DOUBLECOLON
    : '::'
    ;

SYM_SEMICOLON
    : ';'
    ;

IDENT
    : [\p{Alpha}\p{General_Category=Other_Letter}_] [\p{Alnum}\p{General_Category=Other_Letter}_]*
    ;

LIT_STRING
    : '\'' ('\\\\' | '\\\'' | ~[\\'\r\n] )* '\''
    | '"' ('\\\\' | '\\"' | ~[\\"\r\n] )* '"'
    ;

LIT_NUMBER
    : '0' [0-7]*
    | '0x' [0-9A-Fa-f]+
    | '0b' [01]+
    | [1-9] [0-9]*
    ;

WS
    : (' ' | '\t' | '\n' | '\r')+ -> channel(HIDDEN)
    ;
