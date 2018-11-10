/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use ast::*;
use token::*;

/*
named!(pub parse_program<Tokens, Program>,
    
);
*/

#[cfg(test)]
mod test {
    use ast::*;
    use lexer::*;
    use parser::*;
    use token::*;

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
}
