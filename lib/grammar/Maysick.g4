grammar Maysick;

prog
    : (stmt)* EOF
    ;

stmt
    : expr SYM_SEMICOLON
    | stmt_var
    | stmt_let
    | stmt_return
    | stmt_import
    | stmt_assign
    | stmt_fn_def
    | block
    | if_expr
    | while_expr
    ;

stmt_let
    : LET IDENT SYM_EQ expr SYM_SEMICOLON
    ;

stmt_var
    : VAR IDENT SYM_EQ expr SYM_SEMICOLON
    ;

stmt_return
    : RETURN expr? SYM_SEMICOLON
    ;

stmt_import
    : IMPORT LIT_STRING SYM_SEMICOLON
    ;

stmt_assign
    : IDENT '=' expr SYM_SEMICOLON
    ;

stmt_fn_def /* fn ident(a b c ..) { } */
    : FN IDENT SYM_LPAREN IDENT* SYM_RPAREN block
    ;

fn_call
    : IDENT SYM_LPAREN expr* SYM_RPAREN
    ;

expr
    : expr_or
    ;

expr_ident
    : IDENT
    | LIT_STRING
    | LIT_NUMBER
    | SYM_LPAREN expr SYM_RPAREN
    | if_expr
    | while_expr
    | fn_call
    ;

expr_unary
    : SYM_SUB expr_ident
    | expr_ident
    ;

expr_mul
    : expr_unary
    | expr_mul SYM_MUL expr_unary
    | expr_mul SYM_DIV expr_unary
    | expr_mul SYM_MOD expr_unary
    ;

expr_add
    : expr_mul
    | expr_add SYM_ADD expr_mul
    | expr_add SYM_SUB expr_mul
    ;

expr_and
    : expr_add
    | expr_and SYM_AND expr_add
    ;

expr_or
    : expr_and
    | expr_or SYM_OR expr_and
    ;

block
    : SYM_LCURLY (stmt)* SYM_RCURLY
    | SYM_LCURLY (stmt)* expr SYM_RCURLY
    ;

if_expr
    : IF SYM_LPAREN expr SYM_RPAREN block else_stmt?
    ;

else_stmt
    : ELSE stmt
    ;

while_expr
    : WHILE SYM_LPAREN expr SYM_RPAREN block
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
    : '\'' ('\\\\' | '\\\'' | ~[\\"\r\n] )* '\''
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
