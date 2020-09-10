grammar maysick;

prog
    : (stmt)* EOF
    ;

stmt
    : expr SYM_SEMICOLON
    | VAR IDENT SYM_EQ expr SYM_SEMICOLON
    | VAR IDENT SYM_EQ expr SYM_SEMICOLON
    | RETURN expr? SYM_SEMICOLON
    | IMPORT SYM_SEMICOLON
    | IDENT '=' expr SYM_SEMICOLON
    | fn_def
    | block
    | if_expr
    | while_expr
    ;

fn_def /* fn ident(a b c ..) { } */
    : FN IDENT SYM_LPAREN IDENT* SYM_RPAREN block
    ;

fn_call
    : IDENT SYM_LPAREN expr* SYM_RPAREN
    ;

expr
    : IDENT
    | LIT
    | LIT_STRING
    | LIT_NUMBER
    | if_expr
    | while_expr
    | fn_call
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

SYM_SEMICOLON
    : ';'
    ;

IDENT
    : [\p{Alpha}\p{General_Category=Other_Letter}] [\p{Alnum}\p{General_Category=Other_Letter}]*
    ;

LIT
    : LIT_NUMBER
    | LIT_STRING
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
