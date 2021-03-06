use super::*;
use antlr_rust::token::TOKEN_EOF;
use antlr_rust::token_source::TokenSource;

mod test_lexer {
    use super::*;

    #[test]
    fn t_lexer() {
        let input = r#"fn kani(s) {
            let s2 = s + 'かに';
            return s2;
        }
        println(kani('とー'));"#;

        let mut lexer = MaysickLexer::new(Box::new(InputStream::new(input.into())));

        loop {
            let tok = lexer.next_token();
            if tok.get_token_type() == TOKEN_EOF {
                break;
            }

            if tok.get_token_type() == lex::WS {
                continue;
            }

            let _ = AttributedToken::new(&tok);
        }
    }
}

mod test_parser {
    use super::*;

    #[test]
    fn t_full_parse() {
        let input = r#"'とーかちゃんの' + 'かに';"#;
        let _ = full_parse_program(input).unwrap();
    }

    #[test]
    fn t_block() {
        full_parse_program("{ 'かにー'; }").unwrap();
    }

    #[test]
    fn t_expr_fncall() {
        assert_eq!(
            full_parse_program("senko();").unwrap(),
            vec![Stmt::Expr(Expr::FnCall("senko".to_string(), vec!())),]
        );
    }

    #[test]
    fn t_expr_literal_str() {
        assert_eq!(
            full_parse_program("'senko';").unwrap(),
            vec![Stmt::Expr(Expr::Literal(Literal::String(
                "senko".to_string()
            ))),]
        );
    }

    #[test]
    fn t_expr_literal_integer() {
        assert_eq!(
            full_parse_program("1123;").unwrap(),
            vec![Stmt::Expr(Expr::Literal(Literal::Integer(1123)))]
        );
    }

    #[test]
    fn t_expr_infix_01() {
        assert_eq!(
            full_parse_program("11+2+3-9+8;").unwrap(),
            vec![Stmt::Expr(Expr::Infix(
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
            ))]
        );
    }

    #[test]
    fn t_expr_infix_02() {
        assert_eq!(
            full_parse_program("11+2%3-9+8;").unwrap(),
            vec![Stmt::Expr(Expr::Infix(
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
            )),]
        );
    }

    #[test]
    fn t_expr_infix_03() {
        assert_eq!(
            full_parse_program("11-2-3;").unwrap(),
            vec![Stmt::Expr(Expr::Infix(
                Infix::SubOp,
                Box::new(Expr::Infix(
                    Infix::SubOp,
                    Box::new(Expr::Literal(Literal::Integer(11))),
                    Box::new(Expr::Literal(Literal::Integer(2)))
                )),
                Box::new(Expr::Literal(Literal::Integer(3)))
            ))]
        );
    }

    #[test]
    fn t_expr_infix_04() {
        assert_eq!(
            full_parse_program("11 - 2 * 3 = 4;").unwrap(),
            vec![Stmt::Expr(Expr::Infix(
                Infix::EqualOp,
                Box::new(Expr::Infix(
                    Infix::SubOp,
                    Box::new(Expr::Literal(Literal::Integer(11))),
                    Box::new(Expr::Infix(
                        Infix::MulOp,
                        Box::new(Expr::Literal(Literal::Integer(2))),
                        Box::new(Expr::Literal(Literal::Integer(3))),
                    )),
                )),
                Box::new(Expr::Literal(Literal::Integer(4))),
            )),]
        );
    }

    /* #[test]
    fn t_stmt_return() {
        let tokens = vec![
            Token::KReturn,
            Token::Ident("retval".to_string()),
            Token::EndLine,
        ];
        assert_eq!(
            parse_stmt_return(&tokens),
            Ok((
                Tokens::empty(),
                Stmt::Return(Some(Expr::Ident("retval".to_string())))
            ))
        );
    } */

    #[test]
    fn t_stmt_fndef() {
        let program = r#"fn senko() {}"#;
        let res = vec![Stmt::FnDef("senko".to_string(), vec![], vec![])];

        if let Ok(pres) = full_parse_program(program) {
            assert_eq!(pres, res);
        } else {
            panic!("Failed to parse.");
        }
    }

    #[test]
    fn t_stmt_let() {
        let res = vec![Stmt::Let(
            "senko".to_string(),
            None,
            Expr::Literal(Literal::Integer(1123)),
        )];

        if let Ok(pres) = full_parse_program("let senko = 1123;") {
            assert_eq!(pres, res);
        } else {
            panic!("Failed to parse.");
        }
    }

    #[test]
    fn t_stmt_var() {
        let res = vec![Stmt::Var(
            "senko".to_string(),
            None,
            Expr::Literal(Literal::Integer(1123)),
        )];

        if let Ok(pres) = full_parse_program("var senko = 1123;") {
            assert_eq!(pres, res);
        } else {
            panic!("Failed to parse.");
        }
    }

    #[test]
    fn t_helloworld() {
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
        if let Ok(pres) = full_parse_program(
            r#"
        fn main() {
            let retval = 0;
            println('Hello, world!');
            return retval;
        }
        "#,
        ) {
            assert_eq!(pres, res);
        } else {
            panic!("Failed to parse.");
        }
    }
}
