/*
 * Maysick -- The Programming Language
 *
 * 2018 - murueka
 */

use nom::types::CompleteStr;
use token::*;

// Eof parser
named!(token_eof<CompleteStr, Token>,
       map!(eof!(), |_| Token::Eof)
);

// Identifier parser
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
fn gen_token(c: CompleteStr, l: CompleteStr) -> String {
    let mut s = c.0.to_owned();
    s.push_str(l.0);
    s
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
        _ => None,
    }
}
named!(token_ident_str<CompleteStr, String>,
       do_parse!(
           c: verify!(take!(1), |r: CompleteStr| check_ident(r.chars().nth(0), false)) >>
           l: take_while!(|c: char| check_ident(Some(c), true)) >>
           (gen_token(c, l))
       )
);
named!(token_ident<CompleteStr, Token>,
       map!(token_ident_str, |s| {
           if let Some(k) = parse_reserved_keyword(s.as_str()) {
               k
           }else {
               Token::Ident(s)
           }
       })
);

// Binary function operator parser
named!(token_binfop<CompleteStr, Token>,
       do_parse!(
           tag!("`") >>
           i: token_ident_str >>
           tag!("`") >>
           (Token::BinaryFnOp(i))
       )
);

// Symbol parser
named!(token_symbol<CompleteStr, Token>,
       alt!(
           tag!("->") => { |_| Token::Arrow }   |
           tag!("=")  => { |_| Token::EqualOp } |
           tag!("+")  => { |_| Token::AddOp }   |
           tag!("-")  => { |_| Token::SubOp }   |
           tag!("%")  => { |_| Token::ModOp }   |
           tag!("(")  => { |_| Token::LParen }  |
           tag!(")")  => { |_| Token::RParen }  |
           tag!(",")  => { |_| Token::Comma }   |
           tag!(":")  => { |_| Token::Colon }
       )
);

// String parser
named!(
    token_string_escape<CompleteStr, String>,
        escaped_transform!(
            is_not!("\n\\'"),
            '\\',
            alt!(
                         tag!("\\")   => { |_| "\\" }
                         | tag!("'")  => { |_| "'"  }
                         | tag!("n")  => { |_| "\n" }
                     )
        )
);
named!(token_string<CompleteStr, Token>,
       do_parse!(
           tag!("'")              >>
           s: token_string_escape >>
           tag!("'")              >>
           (Token::String(s))
       )
);

// Integer parser
named!(token_integer<CompleteStr, Token>,
       map!(
           alt!(
               do_parse!(
                   tag!("0x") >>
                   num: map_res!(nom::hex_digit, |res: CompleteStr| i64::from_str_radix(res.0, 16)) >>
                   (num)
               ) |
               map_res!(nom::digit, |res: CompleteStr| {
                   if &res.0[0..1] == "0" {
                       i64::from_str_radix(res.0, 8)
                   } else {
                       i64::from_str_radix(res.0, 10)
                   }
               })
           ),
           |res| Token::Integer(res)
       )
);

named!(token_lex<CompleteStr, Token>,
       alt!(
           token_ident    |
           token_integer  |
           token_string   |
           token_symbol   |
           token_binfop
       )
);

named!(token_line<CompleteStr, Vec<Token>>,
       ws!(many0!(token_lex))
);

named!(pub token_maysick_line<CompleteStr, Vec<Token>>,
       do_parse!(
            map!(nom::digit, |_| ()) >>
            r: token_line            >>
            (r)
          )
);

/* tests */

#[cfg(test)]
mod test {
    use lexer::*;

    #[test]
    fn t_eof() {
        let empty = CompleteStr::from("");
        assert_eq!(token_eof(empty), Ok((empty, Token::Eof)));
    }

    #[test]
    fn t_keyword() {
        let empty = CompleteStr::from("");
        let pat = vec!["fn", "let", "var", "if", "else", "while", "return"];
        let res = vec![
            Token::KFn,
            Token::KLet,
            Token::KVar,
            Token::KIf,
            Token::KElse,
            Token::KWhile,
            Token::KReturn,
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
        let pat = vec!["`div`", "`mul`", "`nomay`"];
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
