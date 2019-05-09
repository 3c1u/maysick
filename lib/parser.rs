/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use crate::ast::*;
use crate::token::*;

// トークンを取り出すためのパーサー
named!(take_any<Tokens, Token>,
        map!(take!(1), |r: Tokens| {
            r.item_at(0)
        })
);
named_args!(take_token(token: Token)<Tokens, Token>,
            map_opt!(take!(1), |r: Tokens| {
                let t: Token = r.item_at(0);
                if token == t {
                    Some(t)
                } else {
                    None
                }
            })
);

// Expr
named!(parse_expr<Tokens, Expr>,
    apply!(parse_expr_left, 10)
);

named!(parse_expr_if<Tokens, Expr>,
       do_parse!(
               apply!(take_token, Token::KIf)    >>
               cond: parse_expr                  >>
               apply!(take_token, Token::LBlock) >>
               prog: parse_program               >>
               apply!(take_token, Token::RBlock) >>
               (Expr::If(Box::new(cond), prog))
       )
);

named!(parse_expr_while<Tokens, Expr>,
       do_parse!(
           apply!(take_token, Token::KWhile)     >>
               cond: parse_expr                  >>
               apply!(take_token, Token::LBlock) >>
               prog: parse_program               >>
               apply!(take_token, Token::RBlock) >>
               (Expr::While(Box::new(cond), prog))
       )
);

named_args!(parse_expr_left(n: i64)<Tokens, Expr>,
    alt!(
        apply!(parse_expr_infix, n) |
        parse_expr_fncall |
        do_parse!(
            apply!(take_token, Token::LParen) >>
            r: parse_expr >>
            apply!(take_token, Token::RParen) >>
            (r)
        ) |
        // parse_expr_prefix |
        parse_expr_if     |
        parse_expr_while  |
        parse_expr_ident   |
        parse_expr_literal
        )
);

fn get_op_precedence(op: &Infix) -> i32 {
    match op {
        Infix::EqualOp => 100,
        Infix::BinFnOp => 80,
        Infix::ModOp => 20,
        Infix::DivOp => 20,
        Infix::MulOp => 20,
        Infix::AndOp => 20,
        Infix::OrOp => 10,
        Infix::AddOp => 10,
        Infix::SubOp => 10,
    }
}

fn op_to_infix(t: Token) -> Option<Infix> {
    match t {
        Token::EqualOp => Some(Infix::EqualOp),
        Token::BinaryFnOp(_) => Some(Infix::BinFnOp),
        Token::ModOp => Some(Infix::ModOp),
        Token::DivOp => Some(Infix::DivOp),
        Token::MulOp => Some(Infix::MulOp),
        Token::AndOp => Some(Infix::AndOp),
        Token::OrOp => Some(Infix::OrOp),
        Token::AddOp => Some(Infix::AddOp),
        Token::SubOp => Some(Infix::SubOp),
        _ => None,
    }
}

fn get_infix_precedence(expr: &Expr) -> i32 {
    if let Expr::Infix(op, _, _) = expr {
        get_op_precedence(op)
    } else {
        1000
    }
}

fn swap_lop(expr: Expr) -> Expr {
    if let Expr::Infix(op, le, re) = expr {
        if get_infix_precedence(&le) < get_op_precedence(&op) {
            if let Expr::Infix(op_l, le_l, re_l) = le.as_ref().clone() {
                Expr::Infix(op_l, le_l, Box::new(swap_lop(Expr::Infix(op, re_l, re))))
            } else {
                Expr::Infix(op, le, re)
            }
        } else {
            Expr::Infix(op, le, re)
        }
    } else {
        expr
    }
}

named_args!(parse_expr_infix(n: i64)<Tokens, Expr>,
    do_parse!(
        // 0 means it has no infix operators
        verify!(take!(0), |_| n != 0 ) >>
        left: apply!(parse_expr_left, 0) >>
        op_r: map_opt!(fold_many1!( do_parse!(
            op: map_opt!(call!(take_any), op_to_infix) >>
            right: apply!(parse_expr_left, 0) >>
            (op, right)
        ), Err(Some(left.clone())), |acc: Result<Expr, Option<Expr>>, item: (Infix, Expr) | {
            let (op, rval) = item;

            if let Ok(lval) = acc {
                let tree = Expr::Infix(op, Box::new(lval), Box::new(rval));
                Ok(swap_lop(tree))
            } else {
                Ok(Expr::Infix(op.clone(), Box::new(left.clone()), Box::new(rval.clone())))
            }
        }), |v: Result<Expr, Option<Expr>>| v.ok()) >>
        (op_r)
    )
);

named!(parse_ident<Tokens, Ident>,
        map_opt!(take_any, |r| {
                match r {
                    Token::Ident(v) => Some(v),
                    _               => None,
                }
        })
);

named!(parse_expr_ident<Tokens, Expr>,
    map!(parse_ident, |idt: Ident| {
            Expr::Ident(idt)
        })
);

named!(parse_expr_literal_int<Tokens, Expr>,
    map_opt!(take_any, |t: Token| {
            if let Token::Integer(i) = t {
                Some(Expr::Literal(Literal::Integer(i)))
            } else {
                None
            }
        })
);

named!(parse_expr_literal_str<Tokens, Expr>,
    map_opt!(take_any, |t: Token| {
            if let Token::String(s) = t {
                Some(Expr::Literal(Literal::String(s)))
            } else {
                None
            }
        })
);

named!(parse_expr_literal<Tokens, Expr>,
    alt!(
        parse_expr_literal_int | parse_expr_literal_str
    )
);

named!(pub parse_expr_fncall<Tokens, Expr>,
    do_parse!(
            i: parse_ident >>
            apply!(take_token, Token::LParen) >>
            v: many0!(parse_expr) >>
            apply!(take_token, Token::RParen) >>
            (Expr::FnCall(i, v))
    )
);

// Stmt
named!(pub parse_stmt<Tokens, Stmt>,
    alt!(
        parse_stmt_return |
        parse_stmt_fndef  |
        parse_stmt_let    |
        parse_stmt_var    |
        parse_stmt_expr   |
        parse_stmt_blocked_expr |
        parse_stmt_subst
    )
);

named!(pub parse_stmt_subst<Tokens, Stmt>,
    do_parse!(
            idt: parse_ident                   >>
            apply!(take_token, Token::EqualOp) >>
            val: parse_expr                    >>
            apply!(take_token, Token::EndLine) >>
            (
                Stmt::Subst(idt, val)
            )
    )
);

named!(pub parse_stmt_return<Tokens, Stmt>,
    do_parse!(
            apply!(take_token, Token::KReturn) >>
            retval: opt!(parse_expr)           >>
            apply!(take_token, Token::EndLine) >>
            (
                Stmt::Return(retval)
            )
    )
);

named!(pub parse_stmt_fndef<Tokens, Stmt>,
    do_parse!(
            apply!(take_token, Token::KFn)    >>
            idt: parse_ident                  >>
            apply!(take_token, Token::LParen) >>
            args: many0!(parse_ident)         >>
            apply!(take_token, Token::RParen) >>
            apply!(take_token, Token::LBlock) >>
            body: many0!(parse_stmt)          >>
            apply!(take_token, Token::RBlock) >>
            (
                Stmt::FnDef(idt, args, body)
            )
    )
);

named!(pub parse_stmt_let<Tokens, Stmt>,
    do_parse!(
            apply!(take_token, Token::KLet)    >>
            idt: parse_ident                   >>
            apply!(take_token, Token::EqualOp) >>
            val: parse_expr                    >>
            apply!(take_token, Token::EndLine) >>
            (
                Stmt::Let(idt, None, val)
            )
    )
);

named!(pub parse_stmt_var<Tokens, Stmt>,
    do_parse!(
            apply!(take_token, Token::KVar)    >>
            idt: parse_ident                   >>
            apply!(take_token, Token::EqualOp) >>
            val: parse_expr                    >>
            apply!(take_token, Token::EndLine) >>
            (
                Stmt::Var(idt, None, val)
            )
    )
);

named!(pub parse_stmt_blocked_expr<Tokens, Stmt>,
       do_parse!(
           v: alt!(
               parse_expr_if | parse_expr_while
           ) >>
           (
               Stmt::Expr(v)
           )
       )
);

named!(pub parse_stmt_expr<Tokens, Stmt>,
    do_parse!(
            val: parse_expr >>
            apply!(take_token, Token::EndLine) >>
            (
                Stmt::Expr(val)
            )
    )
);

named!(pub parse_program<Tokens, Program>, many0!(parse_stmt));

// テストセクション

#[cfg(test)]
mod test {
    use crate::parser::*;

    #[test]
    fn t_expr_fncall() {
        let tokens = vec![
            Token::Ident("nomay".to_string()),
            Token::LParen,
            Token::RParen,
        ];
        assert_eq!(
            parse_expr(Tokens::new(&tokens)),
            Ok((Tokens::empty(), Expr::FnCall("nomay".to_string(), vec!())))
        );
    }

    #[test]
    fn t_expr_literal_str() {
        let tokens = vec![Token::String("nomay".to_string())];
        assert_eq!(
            parse_expr(Tokens::new(&tokens)),
            Ok((
                Tokens::empty(),
                Expr::Literal(Literal::String("nomay".to_string()))
            ))
        );
    }

    #[test]
    fn t_expr_literal_integer() {
        let tokens = vec![Token::Integer(1123)];
        assert_eq!(
            parse_expr(Tokens::new(&tokens)),
            Ok((Tokens::empty(), Expr::Literal(Literal::Integer(1123))))
        );
    }

    #[test]
    fn t_expr_infix_01() {
        let tokens = vec![
            Token::Integer(11),
            Token::AddOp,
            Token::Integer(2),
            Token::AddOp,
            Token::Integer(3),
            Token::SubOp,
            Token::Integer(9),
            Token::AddOp,
            Token::Integer(8),
        ];
        assert_eq!(
            parse_expr(Tokens::new(&tokens)),
            Ok((
                Tokens::empty(),
                Expr::Infix(
                    Infix::AddOp,
                    Box::new(Expr::Infix(
                        Infix::SubOp,
                        Box::new(Expr::Infix(
                            Infix::AddOp,
                            Box::new(Expr::Infix(
                                Infix::AddOp,
                                Box::new(Expr::Literal(Literal::Integer(11))),
                                Box::new(Expr::Literal(Literal::Integer(2)))
                            )),
                            Box::new(Expr::Literal(Literal::Integer(3)))
                        )),
                        Box::new(Expr::Literal(Literal::Integer(9)))
                    )),
                    Box::new(Expr::Literal(Literal::Integer(8)))
                )
            ))
        );
    }

    #[test]
    fn t_expr_infix_02() {
        let tokens = vec![
            Token::Integer(11),
            Token::AddOp,
            Token::Integer(2),
            Token::ModOp,
            Token::Integer(3),
            Token::SubOp,
            Token::Integer(9),
            Token::AddOp,
            Token::Integer(8),
        ];
        assert_eq!(
            parse_expr(Tokens::new(&tokens)),
            Ok((
                Tokens::empty(),
                Expr::Infix(
                    Infix::AddOp,
                    Box::new(Expr::Infix(
                        Infix::SubOp,
                        Box::new(Expr::Infix(
                            Infix::AddOp,
                            Box::new(Expr::Literal(Literal::Integer(11))),
                            Box::new(Expr::Infix(
                                Infix::ModOp,
                                Box::new(Expr::Literal(Literal::Integer(2))),
                                Box::new(Expr::Literal(Literal::Integer(3)))
                            )),
                        )),
                        Box::new(Expr::Literal(Literal::Integer(9)))
                    )),
                    Box::new(Expr::Literal(Literal::Integer(8)))
                )
            ))
        );
    }

    #[test]
    fn t_expr_infix_03() {
        let tokens = vec![
            Token::Integer(11),
            Token::SubOp,
            Token::Integer(2),
            Token::SubOp,
            Token::Integer(3),
        ];
        assert_eq!(
            parse_expr(Tokens::new(&tokens)),
            Ok((
                Tokens::empty(),
                Expr::Infix(
                    Infix::SubOp,
                    Box::new(Expr::Infix(
                        Infix::SubOp,
                        Box::new(Expr::Literal(Literal::Integer(11))),
                        Box::new(Expr::Literal(Literal::Integer(2)))
                    )),
                    Box::new(Expr::Literal(Literal::Integer(3)))
                )
            ))
        );
    }

    #[test]
    fn t_expr_infix_04() {
        let tokens = vec![
            Token::Integer(11),
            Token::SubOp,
            Token::Integer(2),
            Token::MulOp,
            Token::Integer(3),
            Token::EqualOp,
            Token::Integer(4),
        ];
        assert_eq!(
            parse_expr(Tokens::new(&tokens)),
            Ok((
                Tokens::empty(),
                Expr::Infix(
                    Infix::SubOp,
                    Box::new(Expr::Literal(Literal::Integer(11))),
                    Box::new(Expr::Infix(
                        Infix::MulOp,
                        Box::new(Expr::Literal(Literal::Integer(2))),
                        Box::new(Expr::Infix(
                            Infix::EqualOp,
                            Box::new(Expr::Literal(Literal::Integer(3))),
                            Box::new(Expr::Literal(Literal::Integer(4)))
                        )),
                    )),
                )
            ))
        );
    }

    #[test]
    fn t_stmt_return() {
        let tokens = vec![
            Token::KReturn,
            Token::Ident("retval".to_string()),
            Token::EndLine,
        ];
        assert_eq!(
            parse_stmt_return(Tokens::new(&tokens)),
            Ok((
                Tokens::empty(),
                Stmt::Return(Some(Expr::Ident("retval".to_string())))
            ))
        );
    }

    #[test]
    fn t_stmt_fndef() {
        let tokens = vec![
            Token::KFn,
            Token::Ident("nomay".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBlock,
            Token::RBlock,
        ];
        let res = vec![Stmt::FnDef("nomay".to_string(), vec![], vec![])];

        if let Ok((_, pres)) = parse_program(Tokens::new(&tokens)) {
            assert_eq!(pres, res);
        } else {
            panic!("Failed to parse.");
        }
    }

    #[test]
    fn t_stmt_let() {
        let tokens = vec![
            Token::KLet,
            Token::Ident("nomay".to_string()),
            Token::EqualOp,
            Token::Integer(1123),
            Token::EndLine,
        ];
        let res = vec![Stmt::Let(
            "nomay".to_string(),
            None,
            Expr::Literal(Literal::Integer(1123)),
        )];

        if let Ok((_, pres)) = parse_program(Tokens::new(&tokens)) {
            assert_eq!(pres, res);
        } else {
            panic!("Failed to parse.");
        }
    }

    #[test]
    fn t_stmt_var() {
        let tokens = vec![
            Token::KVar,
            Token::Ident("nomay".to_string()),
            Token::EqualOp,
            Token::Integer(1123),
            Token::EndLine,
        ];
        let res = vec![Stmt::Var(
            "nomay".to_string(),
            None,
            Expr::Literal(Literal::Integer(1123)),
        )];

        if let Ok((_, pres)) = parse_program(Tokens::new(&tokens)) {
            assert_eq!(pres, res);
        } else {
            panic!("Failed to parse.");
        }
    }

    #[test]
    fn t_helloworld() {
        let tokens = vec![
            Token::KFn,
            Token::Ident("main".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBlock,
            Token::KLet,
            Token::Ident("retval".to_string()),
            Token::EqualOp,
            Token::Integer(0),
            Token::EndLine,
            Token::Ident("println".to_string()),
            Token::LParen,
            Token::String("Hello, world!".to_string()),
            Token::RParen,
            Token::EndLine,
            Token::KReturn,
            Token::Ident("retval".to_string()),
            Token::EndLine,
            Token::RBlock,
        ];
        let res = vec![Stmt::FnDef(
            "main".to_string(),
            vec![],
            vec![
                Stmt::Let(
                    "retval".to_string(),
                    None,
                    Expr::Literal(Literal::Integer(0)),
                ),
                Stmt::Expr(Expr::FnCall(
                    "println".to_string(),
                    vec![Expr::Literal(Literal::String("Hello, world!".to_string()))],
                )),
                Stmt::Return(Some(Expr::Ident("retval".to_string()))),
            ],
        )];
        if let Ok((_, pres)) = parse_program(Tokens::new(&tokens)) {
            assert_eq!(pres, res);
        } else {
            panic!("Failed to parse.");
        }
    }
}
