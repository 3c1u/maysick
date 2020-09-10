/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use crate::ast::*;
use crate::token::*;

use crate::grammar::maysicklexer as lex;
use crate::grammar::maysickparser as par;

use lex::MaysickLexer;
use par::MaysickParser;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::token::{Token as AntlrToken, TOKEN_EOF};
use antlr_rust::token_source::TokenSource;

pub fn token_maysick_line(input: &str) -> Result<Vec<Token>, ()> {
    todo!()
}

pub fn token_line(input: &str) -> Result<Vec<Token>, ()> {
    todo!()
}

pub fn parse_expr(tokens: &[Token]) -> Result<Expr, ()> {
    todo!()
}

pub fn parse_program(tokens: &[Token]) -> Result<Program, ()> {
    todo!()
}

pub fn full_parse_program(input: &str) -> Result<Program, ()> {
    let lexer = MaysickLexer::new(Box::new(InputStream::new(input.into())));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = MaysickParser::new(Box::new(token_source));
    todo!()
}

#[cfg(test)]
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

            let tok = AttributedToken::new(&tok);
            println!("{:#?}", tok);
        }
    }
}

#[cfg(test)]
mod test_parser {
    use super::*;

    #[test]
    fn t_expr_fncall() {
        let tokens = vec![
            Token::Ident("senko".to_string()),
            Token::LParen,
            Token::RParen,
        ];
        assert_eq!(
            parse_expr(&tokens),
            Ok(Expr::FnCall("senko".to_string(), vec!()))
        );
    }

    #[test]
    fn t_expr_literal_str() {
        let tokens = vec![Token::String("senko".to_string())];
        assert_eq!(
            parse_expr(&tokens),
            Ok(Expr::Literal(Literal::String("senko".to_string())))
        );
    }

    #[test]
    fn t_expr_literal_integer() {
        let tokens = vec![Token::Integer(1123)];
        assert_eq!(
            parse_expr(&tokens),
            Ok(Expr::Literal(Literal::Integer(1123)))
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
            parse_expr(&tokens),
            Ok(Expr::Infix(
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
            parse_expr(&tokens),
            Ok(Expr::Infix(
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
            parse_expr(&tokens),
            Ok(Expr::Infix(
                Infix::SubOp,
                Box::new(Expr::Infix(
                    Infix::SubOp,
                    Box::new(Expr::Literal(Literal::Integer(11))),
                    Box::new(Expr::Literal(Literal::Integer(2)))
                )),
                Box::new(Expr::Literal(Literal::Integer(3)))
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
            parse_expr(&tokens),
            Ok(Expr::Infix(
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
            ))
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
        let tokens = vec![
            Token::KFn,
            Token::Ident("senko".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBlock,
            Token::RBlock,
        ];
        let res = vec![Stmt::FnDef("senko".to_string(), vec![], vec![])];

        if let Ok(pres) = parse_program(&tokens) {
            assert_eq!(pres, res);
        } else {
            panic!("Failed to parse.");
        }
    }

    #[test]
    fn t_stmt_let() {
        let tokens = vec![
            Token::KLet,
            Token::Ident("senko".to_string()),
            Token::EqualOp,
            Token::Integer(1123),
            Token::EndLine,
        ];
        let res = vec![Stmt::Let(
            "senko".to_string(),
            None,
            Expr::Literal(Literal::Integer(1123)),
        )];

        if let Ok(pres) = parse_program(&tokens) {
            assert_eq!(pres, res);
        } else {
            panic!("Failed to parse.");
        }
    }

    #[test]
    fn t_stmt_var() {
        let tokens = vec![
            Token::KVar,
            Token::Ident("senko".to_string()),
            Token::EqualOp,
            Token::Integer(1123),
            Token::EndLine,
        ];
        let res = vec![Stmt::Var(
            "senko".to_string(),
            None,
            Expr::Literal(Literal::Integer(1123)),
        )];

        if let Ok(pres) = parse_program(&tokens) {
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
        if let Ok(pres) = parse_program(&tokens) {
            assert_eq!(pres, res);
        } else {
            panic!("Failed to parse.");
        }
    }
}
