/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use ast::*;
use token::*;

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
    alt!(
        parse_expr_fncall |
        do_parse!(
            apply!(take_token, Token::LParen) >>
            r: parse_expr >>
            apply!(take_token, Token::RParen) >>
            (r)
        ) |
        // parse_expr_prefix |
        apply!(parse_expr_infix, 100) | // TODO `f`の実装
        // parse_expr_if     |
        // parse_expr_while  |
        parse_expr_ident   |
        parse_expr_literal
        )
);

named_args!(parse_expr_left(n: i64)<Tokens, Expr>,
    alt!(
        parse_expr_fncall |
        do_parse!(
            apply!(take_token, Token::LParen) >>
            r: parse_expr >>
            apply!(take_token, Token::RParen) >>
            (r)
        ) |
        apply!(parse_expr_infix, n) |
        // parse_expr_prefix |
        // parse_expr_if     |
        // parse_expr_while  |
        parse_expr_ident   |
        parse_expr_literal
        )
);

named_args!(parse_infix_op(op: Token, itype: Infix, priority: i64, n: i64)<Tokens, Expr>,
            do_parse!(
              verify!(take!(0), |_| { priority <= n })    >>
              left: apply!(parse_expr_left, priority - 1) >>
              res : apply!(parse_infix_op_t,
                           op,
                           itype,
                           priority,
                           priority,
                           left) >>
              (res)
            )
);

named_args!(parse_infix_op_t(op: Token, itype: Infix, priority: i64, n: i64, left: Expr)<Tokens, Expr>,
            alt!(
                do_parse!(
                    verify!(take!(0), |_| { priority == n } ) >>
                    apply!(take_token, op.clone()) >>
                    comb: map!(apply!(parse_expr_left, priority - 1),
                               |right| { Expr::Infix(itype, Box::new(left.clone()), Box::new(right)) })
                    >>
                    res : opt!(apply!(parse_infix_op_t_w, n, comb.clone())) >>
                    (match res {
                        Some(r) => r,
                        None    => comb,
                    })
                )
            )
);

named_args!(parse_infix_op_t_w(n: i64, left: Expr)<Tokens, Expr>,
    alt!(
        apply!(parse_infix_op_t,
               Token::AddOp, Infix::AddOp, 2, n, left.clone()) |
        apply!(parse_infix_op_t,
               Token::SubOp, Infix::SubOp, 2, n, left.clone()) |
        apply!(parse_infix_op_t,
               Token::ModOp, Infix::ModOp, 1, n, left.clone())
    )
);

named_args!(parse_expr_infix(n: i64)<Tokens, Expr>,
    alt!(
        apply!(parse_infix_op,
               Token::AddOp, Infix::AddOp, 2, n) |
        apply!(parse_infix_op,
               Token::SubOp, Infix::SubOp, 2, n) |
        apply!(parse_infix_op,
               Token::ModOp, Infix::ModOp, 1, n)
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
        parse_stmt_expr
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
    use ast::*;
    use parser::*;
    use token::*;

    #[test]
    fn t_expr_fncall() {
        let tokens = vec![
            Token::Ident("nomay".to_string()),
            Token::LParen,
            Token::RParen,
        ];
        assert_eq!(
            parse_expr(Tokens::new(&tokens)),
            Ok((
                Tokens::empty(),
                Expr::FnCall("nomay".to_string(), vec!())
            ))
        );
    }

    #[test]
    fn t_expr_literal_str() {
        let tokens = vec![
            Token::String("nomay".to_string()),
        ];
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
        let tokens = vec![
            Token::Integer(1123),
        ];
        assert_eq!(
            parse_expr(Tokens::new(&tokens)),
            Ok((
                Tokens::empty(),
                Expr::Literal(Literal::Integer(1123))
            ))
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
                Expr::Infix(Infix::AddOp,
                    Box::new(Expr::Infix(Infix::SubOp,
                        Box::new(Expr::Infix(Infix::AddOp,
                            Box::new(Expr::Infix(Infix::AddOp,
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
                Expr::Infix(Infix::AddOp,
                    Box::new(Expr::Infix(Infix::SubOp,
                        Box::new(Expr::Infix(Infix::AddOp,
                            Box::new(Expr::Literal(Literal::Integer(11))),
                            Box::new(Expr::Infix(Infix::ModOp,
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
                Expr::Infix(Infix::SubOp,
                    Box::new(Expr::Infix(Infix::SubOp,
                        Box::new(Expr::Literal(Literal::Integer(11))),
                        Box::new(Expr::Literal(Literal::Integer(2)))
                    )),
                    Box::new(Expr::Literal(Literal::Integer(3)))
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
        let res = vec![Stmt::FnDef(
            "nomay".to_string(),
            vec![],
            vec![],
        )];

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
