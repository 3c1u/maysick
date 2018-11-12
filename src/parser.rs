/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use ast::*;
use token::*;

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


named!(pub parse_expr_ident<Tokens, Expr>,
    map_opt!(take!(1), |r: Tokens| {
                let t: Token = r.item_at(0);
                if let Token::Ident(idt) = t {
                    Some(Expr::Ident(idt))
                } else {
                    None
                }
            })
);

named!(pub parse_expr<Tokens, Expr>,
    alt!(parse_expr_ident)
);

named!(pub parse_stmt_return<Tokens, Stmt>,
    do_parse!(
            apply!(take_token, Token::KReturn) >>
            retval: parse_expr                 >>
            (
                Stmt::Return(retval)
            )  
    )
);

#[cfg(test)]
mod test {
    use ast::*;
    use lexer::*;
    use parser::*;
    use token::*;

    #[test]
    fn test_stmt_return() {
        let tokens = vec![
            Token::KReturn,
            Token::Ident("retval".to_string()),
        ];
        assert_eq!(
            parse_stmt_return(Tokens::new(&tokens)),
            Ok(
                (Tokens::empty(),
                 Stmt::Return(
                     Expr::Ident("retval".to_string())
                     )
                )
            )
        );
    }
    /*
    #[test]
    fn test_parser() {
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

        println!("{:#?}", parse_program(Tokens::new(&tokens)));
    }
    */
}
