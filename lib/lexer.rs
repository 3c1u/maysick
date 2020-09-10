/*
 * maysick
 *
 * 2018 - murueka
 */

use crate::token::*;

fn check_ident(oc: Option<char>, contains_number: bool) -> bool {
    match oc {
        None => false,
        Some(c) => {
            ('A' <= c && c <= 'Z')
                || ('a' <= c && c <= 'z')
                || (contains_number && '0' <= c && c <= '9')
                || c == '_'
                || c == '$'
        }
    }
}

fn parse_reserved_keyword(s: &str) -> Option<Token> {
    match s {
        "fn" => Some(Token::KFn),
        "let" => Some(Token::KLet),
        "var" => Some(Token::KVar),
        "if" => Some(Token::KIf),
        "else" => Some(Token::KElse),
        "while" => Some(Token::KWhile),
        "return" => Some(Token::KReturn),
        "import" => Some(Token::KImport),
        _ => None,
    }
}

/* tests */

#[cfg(test)]
mod test {
    use crate::lexer::*;

    #[test]
    fn t_eof() {
        let empty = CompleteStr::from("");
        assert_eq!(token_eof(empty), Ok((empty, Token::Eof)));
    }

    #[test]
    fn t_keyword() {
        let empty = CompleteStr::from("");
        let pat = vec![
            "fn", "let", "var", "if", "else", "while", "return", "import",
        ];
        let res = vec![
            Token::KFn,
            Token::KLet,
            Token::KVar,
            Token::KIf,
            Token::KElse,
            Token::KWhile,
            Token::KReturn,
            Token::KImport,
        ];
        let cnt = pat.len();
        for i in 0..cnt {
            assert_eq!(
                token_ident(CompleteStr::from(pat[i])),
                Ok((empty, res[i].clone()))
            );
        }
    }

    #[test]
    fn t_ident() {
        let empty = CompleteStr::from("");
        let pat = vec!["nomay", "nomay1123", "ManoChan", "$php_is_shit"];
        for s in pat {
            assert_eq!(
                token_ident(CompleteStr::from(s)),
                Ok((empty, Token::Ident(s.to_string())))
            );
        }
    }

    #[test]
    #[should_panic]
    fn t_ident_should_fail() {
        let empty = CompleteStr::from("");
        let s = "1123nomay";

        assert_eq!(
            token_ident(CompleteStr::from(s)),
            Ok((empty, Token::Ident(s.to_string())))
        );
    }

    #[test]
    fn t_binfop() {
        let empty = CompleteStr::from("");
        let pat = vec![/*"`div`", "`mul`",*/ "`nomay`"];
        for s in pat {
            assert_eq!(
                token_binfop(CompleteStr::from(s)),
                Ok((empty, Token::BinaryFnOp(s[1..s.len() - 1].to_string())))
            );
        }
    }

    #[test]
    #[should_panic]
    fn t_binfop_should_fail() {
        let empty = CompleteStr::from("");
        let s = "``";

        assert_eq!(
            token_binfop(CompleteStr::from(s)),
            Ok((empty, Token::BinaryFnOp("".to_string())))
        );
    }

    #[test]
    fn t_numbers() {
        let empty = CompleteStr::from("");
        let pat = vec!["0123", "0xCafeBaBE", "1123"];
        let res: Vec<i64> = vec![83, 0xCAFEBABE, 1123];
        let cnt = pat.len();
        for i in 0..cnt {
            assert_eq!(
                token_integer(CompleteStr::from(pat[i])),
                Ok((empty, Token::Integer(res[i])))
            );
        }
    }

    #[test]
    fn t_numbers_with_residue() {
        let empty = CompleteStr::from("nomay");
        let pat = vec!["01123nomay", "0xCAFEBABEnomay", "1123nomay"];
        let res: Vec<i64> = vec![595, 0xCAFEBABE, 1123];
        let cnt = pat.len();
        for i in 0..cnt {
            assert_eq!(
                token_integer(CompleteStr::from(pat[i])),
                Ok((empty, Token::Integer(res[i])))
            );
        }
    }

    #[test]
    #[should_panic]
    fn t_numbers_should_panic() {
        let empty = CompleteStr::from("98nomay");
        let pat = vec!["0112398nomay", "nomay"];
        let res: Vec<i64> = vec![0112398, 0];
        let cnt = pat.len();
        for i in 0..cnt {
            assert_eq!(
                token_integer(CompleteStr::from(pat[i])),
                Ok((empty, Token::Integer(res[i])))
            );
        }
    }

    #[test]
    fn t_string() {
        assert_eq!(
            token_string(CompleteStr::from("'nomay\\' のーめい'")),
            Ok((
                CompleteStr::from(""),
                Token::String("nomay' のーめい".to_string())
            ))
        )
    }
}
