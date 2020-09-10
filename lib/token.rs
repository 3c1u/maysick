/*
 * maysick
 *
 * 2018 - murueka
 */

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // EOF
    Eof,

    // fn, let, var, if, else, while, return
    KFn,
    KLet,
    KVar,
    KIf,
    KElse,
    KWhile,
    KReturn,
    KImport,

    Ident(String),

    // integer.
    Integer(i64),
    // 'yah string'
    String(String),

    // =
    EqualOp,
    // +
    AddOp,
    // -
    SubOp,
    // %
    ModOp,
    // `mul`, `div`, etc...
    MulOp,
    DivOp,
    AndOp,
    OrOp,
    NotOp,
    BinaryFnOp(String),

    // ->
    Arrow,

    // ::
    DoubleColon,

    // .
    Dot,

    // (
    LParen,
    // )
    RParen,

    // [
    LBox,
    // ]
    RBox,

    // ,
    Comma,

    // :
    Colon,

    // Line
    EndLine,

    // Block
    LBlock, //equivalent to {
    RBlock, //equivalent to }
}

use antlr_rust::char_stream::CharStream;
use antlr_rust::token::{OwningToken, TOKEN_EOF};
use antlr_rust::token_source::TokenSource;

#[derive(Debug)]
pub struct AttributedToken {
    pub token: Token,
    pub owning_token: OwningToken,
}

impl Token {
    fn token_type(&self) -> isize {
        use crate::grammar::maysicklexer as t;

        match self {
            Token::Eof => TOKEN_EOF,
            Token::KFn => t::FN,
            Token::KLet => t::LET,
            Token::KVar => t::VAR,
            Token::KIf => t::IF,
            Token::KElse => t::ELSE,
            Token::KWhile => t::WHILE,
            Token::KReturn => t::RETURN,
            Token::KImport => t::IMPORT,
            Token::Ident(_) => t::IDENT,
            Token::Integer(_) => t::LIT_NUMBER,
            Token::String(_) => t::LIT_STRING,
            Token::EqualOp => t::SYM_EQ,
            Token::AddOp => t::SYM_AND,
            Token::SubOp => t::SYM_SUB,
            Token::ModOp => t::SYM_MOD,
            Token::MulOp => t::SYM_MUL,
            Token::DivOp => t::SYM_DIV,
            Token::AndOp => t::SYM_AND,
            Token::OrOp => t::SYM_OR,
            Token::DoubleColon => t::SYM_DOUBLECOLON,
            Token::LParen => t::SYM_LPAREN,
            Token::RParen => t::SYM_RPAREN,
            Token::LBox => t::SYM_LBOX,
            Token::RBox => t::SYM_RBOX,
            Token::EndLine => t::SYM_SEMICOLON,
            Token::LBlock => t::SYM_LCURLY,
            Token::RBlock => t::SYM_RCURLY,
            Token::Colon => unimplemented!(),
            Token::Comma => unimplemented!(),
            Token::Dot => unimplemented!(),
            Token::Arrow => unimplemented!(),
            Token::BinaryFnOp(_) => unimplemented!(),
            Token::NotOp => unimplemented!(),
        }
    }
}

impl AttributedToken {
    pub fn new<T>(orig: impl AsRef<T>) -> Self
    where
        T: antlr_rust::token::Token + ?Sized,
    {
        use crate::grammar::maysicklexer as t;
        let orig = orig.as_ref();

        let owning_token = orig.to_owned();

        let token = match orig.get_token_type() {
            TOKEN_EOF => Token::Eof,
            t::IF => Token::KIf,
            t::ELSE => Token::KElse,
            t::WHILE => Token::KWhile,
            t::FN => Token::KFn,
            t::LET => Token::KLet,
            t::VAR => Token::KVar,
            t::RETURN => Token::KReturn,
            t::IMPORT => Token::KImport,
            t::SYM_LPAREN => Token::LParen,
            t::SYM_RPAREN => Token::RParen,
            t::SYM_LBOX => Token::LBox,
            t::SYM_RBOX => Token::RBox,
            t::SYM_LCURLY => Token::LBlock,
            t::SYM_RCURLY => Token::RBlock,
            t::SYM_EQ => Token::EqualOp,
            t::SYM_CMPEQ => unimplemented!(),
            t::SYM_MOD => Token::ModOp,
            t::SYM_AND => Token::AndOp,
            t::SYM_OR => Token::OrOp,
            t::SYM_DIV => Token::DivOp,
            t::SYM_MUL => Token::MulOp,
            t::SYM_ADD => Token::AddOp,
            t::SYM_SUB => Token::SubOp,
            t::SYM_DOUBLECOLON => Token::DoubleColon,
            t::SYM_SEMICOLON => Token::EndLine,
            t::IDENT => Token::Ident((&owning_token.text).into()),
            t::LIT_STRING => Token::String({
                let s = &owning_token.text;
                s[..s.len() - 1][1..].into()
            }),
            // TODO:
            t::LIT_NUMBER => {
                Token::Integer(i64::from_str_radix(&owning_token.text, 10).unwrap_or(0))
            }
            t => unreachable!("unknown token type: {}", t),
        };

        Self {
            token,
            owning_token,
        }
    }
}

impl antlr_rust::token::Token for AttributedToken {
    fn get_token_type(&self) -> isize {
        self.owning_token.token_type
    }

    fn get_channel(&self) -> isize {
        self.owning_token.channel
    }

    fn get_start(&self) -> isize {
        self.owning_token.start
    }

    fn get_stop(&self) -> isize {
        self.owning_token.stop
    }

    fn get_line(&self) -> isize {
        self.owning_token.line
    }
    fn get_column(&self) -> isize {
        self.owning_token.column
    }

    fn get_text(&self) -> &str {
        &self.owning_token.text
    }

    fn set_text(&self, _: String) {
        unimplemented!()
    }

    fn get_token_index(&self) -> isize {
        self.owning_token.token_index
    }

    fn set_token_index(&mut self, _: isize) {
        unimplemented!()
    }

    fn get_source(&self) -> Option<(Box<dyn TokenSource>, Box<dyn CharStream>)> {
        unimplemented!()
    }

    fn get_token_source(&self) -> &dyn TokenSource {
        unimplemented!()
    }

    fn get_input_stream(&self) -> &dyn CharStream {
        unimplemented!()
    }

    fn to_owned(&self) -> OwningToken {
        self.owning_token.clone()
    }
}
