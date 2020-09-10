// Generated from lib/grammar/Maysick.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
use antlr_rust::atn::ATN;
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::char_stream::CharStream;
use antlr_rust::common_token_factory::TokenFactory;
use antlr_rust::dfa::DFA;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::lexer_atn_simulator::{ILexerATNSimulator, LexerATNSimulator};
use antlr_rust::parser_rule_context::{cast, LexerContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::BaseRuleContext;
use antlr_rust::token::*;
use antlr_rust::token_source::TokenSource;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const IF: isize = 1;
pub const ELSE: isize = 2;
pub const WHILE: isize = 3;
pub const FN: isize = 4;
pub const LET: isize = 5;
pub const VAR: isize = 6;
pub const RETURN: isize = 7;
pub const IMPORT: isize = 8;
pub const SYM_LPAREN: isize = 9;
pub const SYM_RPAREN: isize = 10;
pub const SYM_LBOX: isize = 11;
pub const SYM_RBOX: isize = 12;
pub const SYM_LCURLY: isize = 13;
pub const SYM_RCURLY: isize = 14;
pub const SYM_EQ: isize = 15;
pub const SYM_CMPEQ: isize = 16;
pub const SYM_MOD: isize = 17;
pub const SYM_AND: isize = 18;
pub const SYM_OR: isize = 19;
pub const SYM_DIV: isize = 20;
pub const SYM_MUL: isize = 21;
pub const SYM_ADD: isize = 22;
pub const SYM_SUB: isize = 23;
pub const SYM_DOUBLECOLON: isize = 24;
pub const SYM_SEMICOLON: isize = 25;
pub const IDENT: isize = 26;
pub const LIT_STRING: isize = 27;
pub const LIT_NUMBER: isize = 28;
pub const WS: isize = 29;
pub const channelNames: [&'static str; 0 + 2] = ["DEFAULT_TOKEN_CHANNEL", "HIDDEN"];

pub const modeNames: [&'static str; 1] = ["DEFAULT_MODE"];

pub const ruleNames: [&'static str; 29] = [
    "IF",
    "ELSE",
    "WHILE",
    "FN",
    "LET",
    "VAR",
    "RETURN",
    "IMPORT",
    "SYM_LPAREN",
    "SYM_RPAREN",
    "SYM_LBOX",
    "SYM_RBOX",
    "SYM_LCURLY",
    "SYM_RCURLY",
    "SYM_EQ",
    "SYM_CMPEQ",
    "SYM_MOD",
    "SYM_AND",
    "SYM_OR",
    "SYM_DIV",
    "SYM_MUL",
    "SYM_ADD",
    "SYM_SUB",
    "SYM_DOUBLECOLON",
    "SYM_SEMICOLON",
    "IDENT",
    "LIT_STRING",
    "LIT_NUMBER",
    "WS",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 26] = [
    None,
    Some("'if'"),
    Some("'else'"),
    Some("'while'"),
    Some("'fn'"),
    Some("'let'"),
    Some("'var'"),
    Some("'return'"),
    Some("'import'"),
    Some("'('"),
    Some("')'"),
    Some("'['"),
    Some("']'"),
    Some("'{'"),
    Some("'}'"),
    Some("'='"),
    Some("'=='"),
    None,
    None,
    None,
    None,
    None,
    Some("'+'"),
    Some("'-'"),
    Some("'::'"),
    Some("';'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 30] = [
    None,
    Some("IF"),
    Some("ELSE"),
    Some("WHILE"),
    Some("FN"),
    Some("LET"),
    Some("VAR"),
    Some("RETURN"),
    Some("IMPORT"),
    Some("SYM_LPAREN"),
    Some("SYM_RPAREN"),
    Some("SYM_LBOX"),
    Some("SYM_RBOX"),
    Some("SYM_LCURLY"),
    Some("SYM_RCURLY"),
    Some("SYM_EQ"),
    Some("SYM_CMPEQ"),
    Some("SYM_MOD"),
    Some("SYM_AND"),
    Some("SYM_OR"),
    Some("SYM_DIV"),
    Some("SYM_MUL"),
    Some("SYM_ADD"),
    Some("SYM_SUB"),
    Some("SYM_DOUBLECOLON"),
    Some("SYM_SEMICOLON"),
    Some("IDENT"),
    Some("LIT_STRING"),
    Some("LIT_NUMBER"),
    Some("WS"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

pub struct MaysickLexer {
    base: BaseLexer<MaysickLexerActions>,
    //	static { RuntimeMetaData.checkVersion("4.8", RuntimeMetaData.VERSION); }
}

impl Deref for MaysickLexer {
    type Target = BaseLexer<MaysickLexerActions>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for MaysickLexer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl MaysickLexer {
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn add_error_listener(&mut self, _listener: Box<dyn ErrorListener>) {
        self.base.add_error_listener(_listener);
    }

    fn remove_error_listeners(&mut self) {
        self.base.remove_error_listeners()
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "MaysickLexer.g4"
    }

    pub fn new(input: Box<dyn CharStream>) -> Self {
        antlr_rust::recognizer::check_version("0", "1");
        Self {
            base: BaseLexer::new_base_lexer(
                input,
                LexerATNSimulator::new_lexer_atnsimulator(
                    _ATN.clone(),
                    _decision_to_DFA.clone(),
                    _shared_context_cache.clone(),
                ),
                Box::new(MaysickLexerActions {}),
            ),
        }
    }
}

pub struct MaysickLexerActions {}

impl MaysickLexerActions {}

impl LexerRecog for MaysickLexerActions {}

impl Recognizer for MaysickLexerActions {}

impl Actions for MaysickLexerActions {
    type Recog = BaseLexer<MaysickLexerActions>;
}

impl MaysickLexerActions {}

impl TokenSource for MaysickLexer {
    fn next_token(&mut self) -> Box<dyn Token> {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> &mut dyn CharStream {
        self.base.get_input_stream()
    }

    fn get_source_name(&self) -> String {
        self.base.get_source_name()
    }

    fn get_token_factory(&self) -> &dyn TokenFactory {
        self.base.get_token_factory()
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<DFA>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ))
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x1f\u{ed}\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\
		\x05\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\
		\x09\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\
		\x0e\x09\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\
		\x12\x04\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\
		\x17\x09\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\
		\x1b\x04\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x03\x02\x03\x02\x03\
		\x02\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\
		\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\
		\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\
		\x08\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\
		\x09\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\
		\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\x11\x03\x11\x03\x11\x03\
		\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x05\x12\x7c\x0a\x12\x03\x13\
		\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\x05\x13\u{84}\x0a\x13\x03\x14\
		\x03\x14\x03\x14\x03\x14\x03\x14\x05\x14\u{8b}\x0a\x14\x03\x15\x03\x15\
		\x03\x15\x03\x15\x03\x15\x03\x15\x05\x15\u{93}\x0a\x15\x03\x16\x03\x16\
		\x03\x16\x03\x16\x03\x16\x03\x16\x05\x16\u{9b}\x0a\x16\x03\x17\x03\x17\
		\x03\x18\x03\x18\x03\x19\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1b\x03\x1b\
		\x07\x1b\u{a8}\x0a\x1b\x0c\x1b\x0e\x1b\u{ab}\x0b\x1b\x03\x1c\x03\x1c\x03\
		\x1c\x03\x1c\x03\x1c\x03\x1c\x07\x1c\u{b3}\x0a\x1c\x0c\x1c\x0e\x1c\u{b6}\
		\x0b\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x07\x1c\
		\u{bf}\x0a\x1c\x0c\x1c\x0e\x1c\u{c2}\x0b\x1c\x03\x1c\x05\x1c\u{c5}\x0a\
		\x1c\x03\x1d\x03\x1d\x07\x1d\u{c9}\x0a\x1d\x0c\x1d\x0e\x1d\u{cc}\x0b\x1d\
		\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x06\x1d\u{d2}\x0a\x1d\x0d\x1d\x0e\x1d\
		\u{d3}\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x06\x1d\u{da}\x0a\x1d\x0d\x1d\x0e\
		\x1d\u{db}\x03\x1d\x03\x1d\x07\x1d\u{e0}\x0a\x1d\x0c\x1d\x0e\x1d\u{e3}\
		\x0b\x1d\x05\x1d\u{e5}\x0a\x1d\x03\x1e\x06\x1e\u{e8}\x0a\x1e\x0d\x1e\x0e\
		\x1e\u{e9}\x03\x1e\x03\x1e\x02\x02\x1f\x03\x03\x05\x04\x07\x05\x09\x06\
		\x0b\x07\x0d\x08\x0f\x09\x11\x0a\x13\x0b\x15\x0c\x17\x0d\x19\x0e\x1b\x0f\
		\x1d\x10\x1f\x11\x21\x12\x23\x13\x25\x14\x27\x15\x29\x16\x2b\x17\x2d\x18\
		\x2f\x19\x31\x1a\x33\x1b\x35\x1c\x37\x1d\x39\x1e\x3b\x1f\x03\x02\x09\x06\
		\x02\x0c\x0c\x0f\x0f\x24\x24\x5e\x5e\x03\x02\x32\x39\x05\x02\x32\x3b\x43\
		\x48\x63\x68\x03\x02\x32\x33\x03\x02\x33\x3b\x03\x02\x32\x3b\x05\x02\x0b\
		\x0c\x0f\x0f\x22\x22\x04\u{297}\x02\x43\x02\x5c\x02\x61\x02\x61\x02\x63\
		\x02\x7c\x02\u{ac}\x02\u{ac}\x02\u{b7}\x02\u{b7}\x02\u{bc}\x02\u{bc}\x02\
		\u{c2}\x02\u{d8}\x02\u{da}\x02\u{f8}\x02\u{fa}\x02\u{2c3}\x02\u{2c8}\x02\
		\u{2d3}\x02\u{2e2}\x02\u{2e6}\x02\u{2ee}\x02\u{2ee}\x02\u{2f0}\x02\u{2f0}\
		\x02\u{347}\x02\u{347}\x02\u{372}\x02\u{376}\x02\u{378}\x02\u{379}\x02\
		\u{37c}\x02\u{37f}\x02\u{381}\x02\u{381}\x02\u{388}\x02\u{388}\x02\u{38a}\
		\x02\u{38c}\x02\u{38e}\x02\u{38e}\x02\u{390}\x02\u{3a3}\x02\u{3a5}\x02\
		\u{3f7}\x02\u{3f9}\x02\u{483}\x02\u{48c}\x02\u{531}\x02\u{533}\x02\u{558}\
		\x02\u{55b}\x02\u{55b}\x02\u{563}\x02\u{589}\x02\u{5b2}\x02\u{5bf}\x02\
		\u{5c1}\x02\u{5c1}\x02\u{5c3}\x02\u{5c4}\x02\u{5c6}\x02\u{5c7}\x02\u{5c9}\
		\x02\u{5c9}\x02\u{5d2}\x02\u{5ec}\x02\u{5f2}\x02\u{5f4}\x02\u{612}\x02\
		\u{61c}\x02\u{622}\x02\u{659}\x02\u{65b}\x02\u{661}\x02\u{670}\x02\u{6d5}\
		\x02\u{6d7}\x02\u{6de}\x02\u{6e3}\x02\u{6ea}\x02\u{6ef}\x02\u{6f1}\x02\
		\u{6fc}\x02\u{6fe}\x02\u{701}\x02\u{701}\x02\u{712}\x02\u{741}\x02\u{74f}\
		\x02\u{7b3}\x02\u{7cc}\x02\u{7ec}\x02\u{7f6}\x02\u{7f7}\x02\u{7fc}\x02\
		\u{7fc}\x02\u{802}\x02\u{819}\x02\u{81c}\x02\u{82e}\x02\u{842}\x02\u{85a}\
		\x02\u{862}\x02\u{86c}\x02\u{8a2}\x02\u{8b6}\x02\u{8b8}\x02\u{8bf}\x02\
		\u{8d6}\x02\u{8e1}\x02\u{8e5}\x02\u{8eb}\x02\u{8f2}\x02\u{93d}\x02\u{93f}\
		\x02\u{94e}\x02\u{950}\x02\u{952}\x02\u{957}\x02\u{965}\x02\u{973}\x02\
		\u{985}\x02\u{987}\x02\u{98e}\x02\u{991}\x02\u{992}\x02\u{995}\x02\u{9aa}\
		\x02\u{9ac}\x02\u{9b2}\x02\u{9b4}\x02\u{9b4}\x02\u{9b8}\x02\u{9bb}\x02\
		\u{9bf}\x02\u{9c6}\x02\u{9c9}\x02\u{9ca}\x02\u{9cd}\x02\u{9ce}\x02\u{9d0}\
		\x02\u{9d0}\x02\u{9d9}\x02\u{9d9}\x02\u{9de}\x02\u{9df}\x02\u{9e1}\x02\
		\u{9e5}\x02\u{9f2}\x02\u{9f3}\x02\u{9fe}\x02\u{9fe}\x02\u{a03}\x02\u{a05}\
		\x02\u{a07}\x02\u{a0c}\x02\u{a11}\x02\u{a12}\x02\u{a15}\x02\u{a2a}\x02\
		\u{a2c}\x02\u{a32}\x02\u{a34}\x02\u{a35}\x02\u{a37}\x02\u{a38}\x02\u{a3a}\
		\x02\u{a3b}\x02\u{a40}\x02\u{a44}\x02\u{a49}\x02\u{a4a}\x02\u{a4d}\x02\
		\u{a4e}\x02\u{a53}\x02\u{a53}\x02\u{a5b}\x02\u{a5e}\x02\u{a60}\x02\u{a60}\
		\x02\u{a72}\x02\u{a77}\x02\u{a83}\x02\u{a85}\x02\u{a87}\x02\u{a8f}\x02\
		\u{a91}\x02\u{a93}\x02\u{a95}\x02\u{aaa}\x02\u{aac}\x02\u{ab2}\x02\u{ab4}\
		\x02\u{ab5}\x02\u{ab7}\x02\u{abb}\x02\u{abf}\x02\u{ac7}\x02\u{ac9}\x02\
		\u{acb}\x02\u{acd}\x02\u{ace}\x02\u{ad2}\x02\u{ad2}\x02\u{ae2}\x02\u{ae5}\
		\x02\u{afb}\x02\u{afe}\x02\u{b03}\x02\u{b05}\x02\u{b07}\x02\u{b0e}\x02\
		\u{b11}\x02\u{b12}\x02\u{b15}\x02\u{b2a}\x02\u{b2c}\x02\u{b32}\x02\u{b34}\
		\x02\u{b35}\x02\u{b37}\x02\u{b3b}\x02\u{b3f}\x02\u{b46}\x02\u{b49}\x02\
		\u{b4a}\x02\u{b4d}\x02\u{b4e}\x02\u{b58}\x02\u{b59}\x02\u{b5e}\x02\u{b5f}\
		\x02\u{b61}\x02\u{b65}\x02\u{b73}\x02\u{b73}\x02\u{b84}\x02\u{b85}\x02\
		\u{b87}\x02\u{b8c}\x02\u{b90}\x02\u{b92}\x02\u{b94}\x02\u{b97}\x02\u{b9b}\
		\x02\u{b9c}\x02\u{b9e}\x02\u{b9e}\x02\u{ba0}\x02\u{ba1}\x02\u{ba5}\x02\
		\u{ba6}\x02\u{baa}\x02\u{bac}\x02\u{bb0}\x02\u{bbb}\x02\u{bc0}\x02\u{bc4}\
		\x02\u{bc8}\x02\u{bca}\x02\u{bcc}\x02\u{bce}\x02\u{bd2}\x02\u{bd2}\x02\
		\u{bd9}\x02\u{bd9}\x02\u{c02}\x02\u{c05}\x02\u{c07}\x02\u{c0e}\x02\u{c10}\
		\x02\u{c12}\x02\u{c14}\x02\u{c2a}\x02\u{c2c}\x02\u{c3b}\x02\u{c3f}\x02\
		\u{c46}\x02\u{c48}\x02\u{c4a}\x02\u{c4c}\x02\u{c4e}\x02\u{c57}\x02\u{c58}\
		\x02\u{c5a}\x02\u{c5c}\x02\u{c62}\x02\u{c65}\x02\u{c82}\x02\u{c85}\x02\
		\u{c87}\x02\u{c8e}\x02\u{c90}\x02\u{c92}\x02\u{c94}\x02\u{caa}\x02\u{cac}\
		\x02\u{cb5}\x02\u{cb7}\x02\u{cbb}\x02\u{cbf}\x02\u{cc6}\x02\u{cc8}\x02\
		\u{cca}\x02\u{ccc}\x02\u{cce}\x02\u{cd7}\x02\u{cd8}\x02\u{ce0}\x02\u{ce0}\
		\x02\u{ce2}\x02\u{ce5}\x02\u{cf3}\x02\u{cf4}\x02\u{d02}\x02\u{d05}\x02\
		\u{d07}\x02\u{d0e}\x02\u{d10}\x02\u{d12}\x02\u{d14}\x02\u{d3c}\x02\u{d3f}\
		\x02\u{d46}\x02\u{d48}\x02\u{d4a}\x02\u{d4c}\x02\u{d4e}\x02\u{d50}\x02\
		\u{d50}\x02\u{d56}\x02\u{d59}\x02\u{d61}\x02\u{d65}\x02\u{d7c}\x02\u{d81}\
		\x02\u{d84}\x02\u{d85}\x02\u{d87}\x02\u{d98}\x02\u{d9c}\x02\u{db3}\x02\
		\u{db5}\x02\u{dbd}\x02\u{dbf}\x02\u{dbf}\x02\u{dc2}\x02\u{dc8}\x02\u{dd1}\
		\x02\u{dd6}\x02\u{dd8}\x02\u{dd8}\x02\u{dda}\x02\u{de1}\x02\u{df4}\x02\
		\u{df5}\x02\u{e03}\x02\u{e3c}\x02\u{e42}\x02\u{e48}\x02\u{e4f}\x02\u{e4f}\
		\x02\u{e83}\x02\u{e84}\x02\u{e86}\x02\u{e86}\x02\u{e89}\x02\u{e8a}\x02\
		\u{e8c}\x02\u{e8c}\x02\u{e8f}\x02\u{e8f}\x02\u{e96}\x02\u{e99}\x02\u{e9b}\
		\x02\u{ea1}\x02\u{ea3}\x02\u{ea5}\x02\u{ea7}\x02\u{ea7}\x02\u{ea9}\x02\
		\u{ea9}\x02\u{eac}\x02\u{ead}\x02\u{eaf}\x02\u{ebb}\x02\u{ebd}\x02\u{ebf}\
		\x02\u{ec2}\x02\u{ec6}\x02\u{ec8}\x02\u{ec8}\x02\u{ecf}\x02\u{ecf}\x02\
		\u{ede}\x02\u{ee1}\x02\u{f02}\x02\u{f02}\x02\u{f42}\x02\u{f49}\x02\u{f4b}\
		\x02\u{f6e}\x02\u{f73}\x02\u{f83}\x02\u{f8a}\x02\u{f99}\x02\u{f9b}\x02\
		\u{fbe}\x02\u{1002}\x02\u{1038}\x02\u{103a}\x02\u{103a}\x02\u{103d}\x02\
		\u{1041}\x02\u{1052}\x02\u{1064}\x02\u{1067}\x02\u{106a}\x02\u{1070}\x02\
		\u{1088}\x02\u{1090}\x02\u{1090}\x02\u{109e}\x02\u{109f}\x02\u{10a2}\x02\
		\u{10c7}\x02\u{10c9}\x02\u{10c9}\x02\u{10cf}\x02\u{10cf}\x02\u{10d2}\x02\
		\u{10fc}\x02\u{10fe}\x02\u{124a}\x02\u{124c}\x02\u{124f}\x02\u{1252}\x02\
		\u{1258}\x02\u{125a}\x02\u{125a}\x02\u{125c}\x02\u{125f}\x02\u{1262}\x02\
		\u{128a}\x02\u{128c}\x02\u{128f}\x02\u{1292}\x02\u{12b2}\x02\u{12b4}\x02\
		\u{12b7}\x02\u{12ba}\x02\u{12c0}\x02\u{12c2}\x02\u{12c2}\x02\u{12c4}\x02\
		\u{12c7}\x02\u{12ca}\x02\u{12d8}\x02\u{12da}\x02\u{1312}\x02\u{1314}\x02\
		\u{1317}\x02\u{131a}\x02\u{135c}\x02\u{1361}\x02\u{1361}\x02\u{1382}\x02\
		\u{1391}\x02\u{13a2}\x02\u{13f7}\x02\u{13fa}\x02\u{13ff}\x02\u{1403}\x02\
		\u{166e}\x02\u{1671}\x02\u{1681}\x02\u{1683}\x02\u{169c}\x02\u{16a2}\x02\
		\u{16ec}\x02\u{16f0}\x02\u{16fa}\x02\u{1702}\x02\u{170e}\x02\u{1710}\x02\
		\u{1715}\x02\u{1722}\x02\u{1735}\x02\u{1742}\x02\u{1755}\x02\u{1762}\x02\
		\u{176e}\x02\u{1770}\x02\u{1772}\x02\u{1774}\x02\u{1775}\x02\u{1782}\x02\
		\u{17b5}\x02\u{17b8}\x02\u{17ca}\x02\u{17d9}\x02\u{17d9}\x02\u{17de}\x02\
		\u{17de}\x02\u{1822}\x02\u{1879}\x02\u{1882}\x02\u{18ac}\x02\u{18b2}\x02\
		\u{18f7}\x02\u{1902}\x02\u{1920}\x02\u{1922}\x02\u{192d}\x02\u{1932}\x02\
		\u{193a}\x02\u{1952}\x02\u{196f}\x02\u{1972}\x02\u{1976}\x02\u{1982}\x02\
		\u{19ad}\x02\u{19b2}\x02\u{19cb}\x02\u{1a02}\x02\u{1a1d}\x02\u{1a22}\x02\
		\u{1a60}\x02\u{1a63}\x02\u{1a76}\x02\u{1aa9}\x02\u{1aa9}\x02\u{1b02}\x02\
		\u{1b35}\x02\u{1b37}\x02\u{1b45}\x02\u{1b47}\x02\u{1b4d}\x02\u{1b82}\x02\
		\u{1bab}\x02\u{1bae}\x02\u{1bb1}\x02\u{1bbc}\x02\u{1be7}\x02\u{1be9}\x02\
		\u{1bf3}\x02\u{1c02}\x02\u{1c37}\x02\u{1c4f}\x02\u{1c51}\x02\u{1c5c}\x02\
		\u{1c7f}\x02\u{1c82}\x02\u{1c8a}\x02\u{1ceb}\x02\u{1cee}\x02\u{1cf0}\x02\
		\u{1cf5}\x02\u{1cf7}\x02\u{1cf8}\x02\u{1d02}\x02\u{1dc1}\x02\u{1de9}\x02\
		\u{1df6}\x02\u{1e02}\x02\u{1f17}\x02\u{1f1a}\x02\u{1f1f}\x02\u{1f22}\x02\
		\u{1f47}\x02\u{1f4a}\x02\u{1f4f}\x02\u{1f52}\x02\u{1f59}\x02\u{1f5b}\x02\
		\u{1f5b}\x02\u{1f5d}\x02\u{1f5d}\x02\u{1f5f}\x02\u{1f5f}\x02\u{1f61}\x02\
		\u{1f7f}\x02\u{1f82}\x02\u{1fb6}\x02\u{1fb8}\x02\u{1fbe}\x02\u{1fc0}\x02\
		\u{1fc0}\x02\u{1fc4}\x02\u{1fc6}\x02\u{1fc8}\x02\u{1fce}\x02\u{1fd2}\x02\
		\u{1fd5}\x02\u{1fd8}\x02\u{1fdd}\x02\u{1fe2}\x02\u{1fee}\x02\u{1ff4}\x02\
		\u{1ff6}\x02\u{1ff8}\x02\u{1ffe}\x02\u{2073}\x02\u{2073}\x02\u{2081}\x02\
		\u{2081}\x02\u{2092}\x02\u{209e}\x02\u{2104}\x02\u{2104}\x02\u{2109}\x02\
		\u{2109}\x02\u{210c}\x02\u{2115}\x02\u{2117}\x02\u{2117}\x02\u{211b}\x02\
		\u{211f}\x02\u{2126}\x02\u{2126}\x02\u{2128}\x02\u{2128}\x02\u{212a}\x02\
		\u{212a}\x02\u{212c}\x02\u{212f}\x02\u{2131}\x02\u{213b}\x02\u{213e}\x02\
		\u{2141}\x02\u{2147}\x02\u{214b}\x02\u{2150}\x02\u{2150}\x02\u{2162}\x02\
		\u{218a}\x02\u{24b8}\x02\u{24eb}\x02\u{2c02}\x02\u{2c30}\x02\u{2c32}\x02\
		\u{2c60}\x02\u{2c62}\x02\u{2ce6}\x02\u{2ced}\x02\u{2cf0}\x02\u{2cf4}\x02\
		\u{2cf5}\x02\u{2d02}\x02\u{2d27}\x02\u{2d29}\x02\u{2d29}\x02\u{2d2f}\x02\
		\u{2d2f}\x02\u{2d32}\x02\u{2d69}\x02\u{2d71}\x02\u{2d71}\x02\u{2d82}\x02\
		\u{2d98}\x02\u{2da2}\x02\u{2da8}\x02\u{2daa}\x02\u{2db0}\x02\u{2db2}\x02\
		\u{2db8}\x02\u{2dba}\x02\u{2dc0}\x02\u{2dc2}\x02\u{2dc8}\x02\u{2dca}\x02\
		\u{2dd0}\x02\u{2dd2}\x02\u{2dd8}\x02\u{2dda}\x02\u{2de0}\x02\u{2de2}\x02\
		\u{2e01}\x02\u{2e31}\x02\u{2e31}\x02\u{3007}\x02\u{3009}\x02\u{3023}\x02\
		\u{302b}\x02\u{3033}\x02\u{3037}\x02\u{303a}\x02\u{303e}\x02\u{3043}\x02\
		\u{3098}\x02\u{309f}\x02\u{30a1}\x02\u{30a3}\x02\u{30fc}\x02\u{30fe}\x02\
		\u{3101}\x02\u{3107}\x02\u{3130}\x02\u{3133}\x02\u{3190}\x02\u{31a2}\x02\
		\u{31bc}\x02\u{31f2}\x02\u{3201}\x02\u{3402}\x02\u{4db7}\x02\u{4e02}\x02\
		\u{9fec}\x02\u{a002}\x02\u{a48e}\x02\u{a4d2}\x02\u{a4ff}\x02\u{a502}\x02\
		\u{a60e}\x02\u{a612}\x02\u{a621}\x02\u{a62c}\x02\u{a62d}\x02\u{a642}\x02\
		\u{a670}\x02\u{a676}\x02\u{a67d}\x02\u{a681}\x02\u{a6f1}\x02\u{a719}\x02\
		\u{a721}\x02\u{a724}\x02\u{a78a}\x02\u{a78d}\x02\u{a7b0}\x02\u{a7b2}\x02\
		\u{a7b9}\x02\u{a7f9}\x02\u{a803}\x02\u{a805}\x02\u{a807}\x02\u{a809}\x02\
		\u{a80c}\x02\u{a80e}\x02\u{a829}\x02\u{a842}\x02\u{a875}\x02\u{a882}\x02\
		\u{a8c5}\x02\u{a8c7}\x02\u{a8c7}\x02\u{a8f4}\x02\u{a8f9}\x02\u{a8fd}\x02\
		\u{a8fd}\x02\u{a8ff}\x02\u{a8ff}\x02\u{a90c}\x02\u{a92c}\x02\u{a932}\x02\
		\u{a954}\x02\u{a962}\x02\u{a97e}\x02\u{a982}\x02\u{a9b4}\x02\u{a9b6}\x02\
		\u{a9c1}\x02\u{a9d1}\x02\u{a9d1}\x02\u{a9e2}\x02\u{a9e6}\x02\u{a9e8}\x02\
		\u{a9f1}\x02\u{a9fc}\x02\u{aa00}\x02\u{aa02}\x02\u{aa38}\x02\u{aa42}\x02\
		\u{aa4f}\x02\u{aa62}\x02\u{aa78}\x02\u{aa7c}\x02\u{aa7c}\x02\u{aa80}\x02\
		\u{aac0}\x02\u{aac2}\x02\u{aac2}\x02\u{aac4}\x02\u{aac4}\x02\u{aadd}\x02\
		\u{aadf}\x02\u{aae2}\x02\u{aaf1}\x02\u{aaf4}\x02\u{aaf7}\x02\u{ab03}\x02\
		\u{ab08}\x02\u{ab0b}\x02\u{ab10}\x02\u{ab13}\x02\u{ab18}\x02\u{ab22}\x02\
		\u{ab28}\x02\u{ab2a}\x02\u{ab30}\x02\u{ab32}\x02\u{ab5c}\x02\u{ab5e}\x02\
		\u{ab67}\x02\u{ab72}\x02\u{abec}\x02\u{ac02}\x02\u{d7a5}\x02\u{d7b2}\x02\
		\u{d7c8}\x02\u{d7cd}\x02\u{d7fd}\x02\u{f902}\x02\u{fa6f}\x02\u{fa72}\x02\
		\u{fadb}\x02\u{fb02}\x02\u{fb08}\x02\u{fb15}\x02\u{fb19}\x02\u{fb1f}\x02\
		\u{fb2a}\x02\u{fb2c}\x02\u{fb38}\x02\u{fb3a}\x02\u{fb3e}\x02\u{fb40}\x02\
		\u{fb40}\x02\u{fb42}\x02\u{fb43}\x02\u{fb45}\x02\u{fb46}\x02\u{fb48}\x02\
		\u{fbb3}\x02\u{fbd5}\x02\u{fd3f}\x02\u{fd52}\x02\u{fd91}\x02\u{fd94}\x02\
		\u{fdc9}\x02\u{fdf2}\x02\u{fdfd}\x02\u{fe72}\x02\u{fe76}\x02\u{fe78}\x02\
		\u{fefe}\x02\u{ff23}\x02\u{ff3c}\x02\u{ff43}\x02\u{ff5c}\x02\u{ff68}\x02\
		\u{ffc0}\x02\u{ffc4}\x02\u{ffc9}\x02\u{ffcc}\x02\u{ffd1}\x02\u{ffd4}\x02\
		\u{ffd9}\x02\u{ffdc}\x02\u{ffde}\x02\x02\x03\x0d\x03\x0f\x03\x28\x03\x2a\
		\x03\x3c\x03\x3e\x03\x3f\x03\x41\x03\x4f\x03\x52\x03\x5f\x03\u{82}\x03\
		\u{fc}\x03\u{142}\x03\u{176}\x03\u{282}\x03\u{29e}\x03\u{2a2}\x03\u{2d2}\
		\x03\u{302}\x03\u{321}\x03\u{32f}\x03\u{34c}\x03\u{352}\x03\u{37c}\x03\
		\u{382}\x03\u{39f}\x03\u{3a2}\x03\u{3c5}\x03\u{3ca}\x03\u{3d1}\x03\u{3d3}\
		\x03\u{3d7}\x03\u{402}\x03\u{49f}\x03\u{4b2}\x03\u{4d5}\x03\u{4da}\x03\
		\u{4fd}\x03\u{502}\x03\u{529}\x03\u{532}\x03\u{565}\x03\u{602}\x03\u{738}\
		\x03\u{742}\x03\u{757}\x03\u{762}\x03\u{769}\x03\u{802}\x03\u{807}\x03\
		\u{80a}\x03\u{80a}\x03\u{80c}\x03\u{837}\x03\u{839}\x03\u{83a}\x03\u{83e}\
		\x03\u{83e}\x03\u{841}\x03\u{857}\x03\u{862}\x03\u{878}\x03\u{882}\x03\
		\u{8a0}\x03\u{8e2}\x03\u{8f4}\x03\u{8f6}\x03\u{8f7}\x03\u{902}\x03\u{917}\
		\x03\u{922}\x03\u{93b}\x03\u{982}\x03\u{9b9}\x03\u{9c0}\x03\u{9c1}\x03\
		\u{a02}\x03\u{a05}\x03\u{a07}\x03\u{a08}\x03\u{a0e}\x03\u{a15}\x03\u{a17}\
		\x03\u{a19}\x03\u{a1b}\x03\u{a35}\x03\u{a62}\x03\u{a7e}\x03\u{a82}\x03\
		\u{a9e}\x03\u{ac2}\x03\u{ac9}\x03\u{acb}\x03\u{ae6}\x03\u{b02}\x03\u{b37}\
		\x03\u{b42}\x03\u{b57}\x03\u{b62}\x03\u{b74}\x03\u{b82}\x03\u{b93}\x03\
		\u{c02}\x03\u{c4a}\x03\u{c82}\x03\u{cb4}\x03\u{cc2}\x03\u{cf4}\x03\u{1002}\
		\x03\u{1047}\x03\u{1084}\x03\u{10ba}\x03\u{10d2}\x03\u{10ea}\x03\u{1102}\
		\x03\u{1134}\x03\u{1152}\x03\u{1174}\x03\u{1178}\x03\u{1178}\x03\u{1182}\
		\x03\u{11c1}\x03\u{11c3}\x03\u{11c6}\x03\u{11dc}\x03\u{11dc}\x03\u{11de}\
		\x03\u{11de}\x03\u{1202}\x03\u{1213}\x03\u{1215}\x03\u{1236}\x03\u{1239}\
		\x03\u{1239}\x03\u{1240}\x03\u{1240}\x03\u{1282}\x03\u{1288}\x03\u{128a}\
		\x03\u{128a}\x03\u{128c}\x03\u{128f}\x03\u{1291}\x03\u{129f}\x03\u{12a1}\
		\x03\u{12aa}\x03\u{12b2}\x03\u{12ea}\x03\u{1302}\x03\u{1305}\x03\u{1307}\
		\x03\u{130e}\x03\u{1311}\x03\u{1312}\x03\u{1315}\x03\u{132a}\x03\u{132c}\
		\x03\u{1332}\x03\u{1334}\x03\u{1335}\x03\u{1337}\x03\u{133b}\x03\u{133f}\
		\x03\u{1346}\x03\u{1349}\x03\u{134a}\x03\u{134d}\x03\u{134e}\x03\u{1352}\
		\x03\u{1352}\x03\u{1359}\x03\u{1359}\x03\u{135f}\x03\u{1365}\x03\u{1402}\
		\x03\u{1443}\x03\u{1445}\x03\u{1447}\x03\u{1449}\x03\u{144c}\x03\u{1482}\
		\x03\u{14c3}\x03\u{14c6}\x03\u{14c7}\x03\u{14c9}\x03\u{14c9}\x03\u{1582}\
		\x03\u{15b7}\x03\u{15ba}\x03\u{15c0}\x03\u{15da}\x03\u{15df}\x03\u{1602}\
		\x03\u{1640}\x03\u{1642}\x03\u{1642}\x03\u{1646}\x03\u{1646}\x03\u{1682}\
		\x03\u{16b7}\x03\u{1702}\x03\u{171b}\x03\u{171f}\x03\u{172c}\x03\u{18a2}\
		\x03\u{18e1}\x03\u{1901}\x03\u{1901}\x03\u{1a02}\x03\u{1a34}\x03\u{1a37}\
		\x03\u{1a40}\x03\u{1a52}\x03\u{1a85}\x03\u{1a88}\x03\u{1a99}\x03\u{1ac2}\
		\x03\u{1afa}\x03\u{1c02}\x03\u{1c0a}\x03\u{1c0c}\x03\u{1c38}\x03\u{1c3a}\
		\x03\u{1c40}\x03\u{1c42}\x03\u{1c42}\x03\u{1c74}\x03\u{1c91}\x03\u{1c94}\
		\x03\u{1ca9}\x03\u{1cab}\x03\u{1cb8}\x03\u{1d02}\x03\u{1d08}\x03\u{1d0a}\
		\x03\u{1d0b}\x03\u{1d0d}\x03\u{1d38}\x03\u{1d3c}\x03\u{1d3c}\x03\u{1d3e}\
		\x03\u{1d3f}\x03\u{1d41}\x03\u{1d43}\x03\u{1d45}\x03\u{1d45}\x03\u{1d48}\
		\x03\u{1d49}\x03\u{2002}\x03\u{239b}\x03\u{2402}\x03\u{2470}\x03\u{2482}\
		\x03\u{2545}\x03\u{3002}\x03\u{3430}\x03\u{4402}\x03\u{4648}\x03\u{6802}\
		\x03\u{6a3a}\x03\u{6a42}\x03\u{6a60}\x03\u{6ad2}\x03\u{6aef}\x03\u{6b02}\
		\x03\u{6b38}\x03\u{6b42}\x03\u{6b45}\x03\u{6b65}\x03\u{6b79}\x03\u{6b7f}\
		\x03\u{6b91}\x03\u{6f02}\x03\u{6f46}\x03\u{6f52}\x03\u{6f80}\x03\u{6f95}\
		\x03\u{6fa1}\x03\u{6fe2}\x03\u{6fe3}\x03\u{7002}\x03\u{87ee}\x03\u{8802}\
		\x03\u{8af4}\x03\u{b002}\x03\u{b120}\x03\u{b172}\x03\u{b2fd}\x03\u{bc02}\
		\x03\u{bc6c}\x03\u{bc72}\x03\u{bc7e}\x03\u{bc82}\x03\u{bc8a}\x03\u{bc92}\
		\x03\u{bc9b}\x03\u{bca0}\x03\u{bca0}\x03\u{d402}\x03\u{d456}\x03\u{d458}\
		\x03\u{d49e}\x03\u{d4a0}\x03\u{d4a1}\x03\u{d4a4}\x03\u{d4a4}\x03\u{d4a7}\
		\x03\u{d4a8}\x03\u{d4ab}\x03\u{d4ae}\x03\u{d4b0}\x03\u{d4bb}\x03\u{d4bd}\
		\x03\u{d4bd}\x03\u{d4bf}\x03\u{d4c5}\x03\u{d4c7}\x03\u{d507}\x03\u{d509}\
		\x03\u{d50c}\x03\u{d50f}\x03\u{d516}\x03\u{d518}\x03\u{d51e}\x03\u{d520}\
		\x03\u{d53b}\x03\u{d53d}\x03\u{d540}\x03\u{d542}\x03\u{d546}\x03\u{d548}\
		\x03\u{d548}\x03\u{d54c}\x03\u{d552}\x03\u{d554}\x03\u{d6a7}\x03\u{d6aa}\
		\x03\u{d6c2}\x03\u{d6c4}\x03\u{d6dc}\x03\u{d6de}\x03\u{d6fc}\x03\u{d6fe}\
		\x03\u{d716}\x03\u{d718}\x03\u{d736}\x03\u{d738}\x03\u{d750}\x03\u{d752}\
		\x03\u{d770}\x03\u{d772}\x03\u{d78a}\x03\u{d78c}\x03\u{d7aa}\x03\u{d7ac}\
		\x03\u{d7c4}\x03\u{d7c6}\x03\u{d7cd}\x03\u{e002}\x03\u{e008}\x03\u{e00a}\
		\x03\u{e01a}\x03\u{e01d}\x03\u{e023}\x03\u{e025}\x03\u{e026}\x03\u{e028}\
		\x03\u{e02c}\x03\u{e802}\x03\u{e8c6}\x03\u{e902}\x03\u{e945}\x03\u{e949}\
		\x03\u{e949}\x03\u{ee02}\x03\u{ee05}\x03\u{ee07}\x03\u{ee21}\x03\u{ee23}\
		\x03\u{ee24}\x03\u{ee26}\x03\u{ee26}\x03\u{ee29}\x03\u{ee29}\x03\u{ee2b}\
		\x03\u{ee34}\x03\u{ee36}\x03\u{ee39}\x03\u{ee3b}\x03\u{ee3b}\x03\u{ee3d}\
		\x03\u{ee3d}\x03\u{ee44}\x03\u{ee44}\x03\u{ee49}\x03\u{ee49}\x03\u{ee4b}\
		\x03\u{ee4b}\x03\u{ee4d}\x03\u{ee4d}\x03\u{ee4f}\x03\u{ee51}\x03\u{ee53}\
		\x03\u{ee54}\x03\u{ee56}\x03\u{ee56}\x03\u{ee59}\x03\u{ee59}\x03\u{ee5b}\
		\x03\u{ee5b}\x03\u{ee5d}\x03\u{ee5d}\x03\u{ee5f}\x03\u{ee5f}\x03\u{ee61}\
		\x03\u{ee61}\x03\u{ee63}\x03\u{ee64}\x03\u{ee66}\x03\u{ee66}\x03\u{ee69}\
		\x03\u{ee6c}\x03\u{ee6e}\x03\u{ee74}\x03\u{ee76}\x03\u{ee79}\x03\u{ee7b}\
		\x03\u{ee7e}\x03\u{ee80}\x03\u{ee80}\x03\u{ee82}\x03\u{ee8b}\x03\u{ee8d}\
		\x03\u{ee9d}\x03\u{eea3}\x03\u{eea5}\x03\u{eea7}\x03\u{eeab}\x03\u{eead}\
		\x03\u{eebd}\x03\u{f132}\x03\u{f14b}\x03\u{f152}\x03\u{f16b}\x03\u{f172}\
		\x03\u{f18b}\x03\x02\x04\u{a6d8}\x04\u{a702}\x04\u{b736}\x04\u{b742}\x04\
		\u{b81f}\x04\u{b822}\x04\u{cea3}\x04\u{ceb2}\x04\u{ebe2}\x04\u{f802}\x04\
		\u{fa1f}\x04\u{2ba}\x02\x32\x02\x3b\x02\x43\x02\x5c\x02\x61\x02\x61\x02\
		\x63\x02\x7c\x02\u{ac}\x02\u{ac}\x02\u{b7}\x02\u{b7}\x02\u{bc}\x02\u{bc}\
		\x02\u{c2}\x02\u{d8}\x02\u{da}\x02\u{f8}\x02\u{fa}\x02\u{2c3}\x02\u{2c8}\
		\x02\u{2d3}\x02\u{2e2}\x02\u{2e6}\x02\u{2ee}\x02\u{2ee}\x02\u{2f0}\x02\
		\u{2f0}\x02\u{347}\x02\u{347}\x02\u{372}\x02\u{376}\x02\u{378}\x02\u{379}\
		\x02\u{37c}\x02\u{37f}\x02\u{381}\x02\u{381}\x02\u{388}\x02\u{388}\x02\
		\u{38a}\x02\u{38c}\x02\u{38e}\x02\u{38e}\x02\u{390}\x02\u{3a3}\x02\u{3a5}\
		\x02\u{3f7}\x02\u{3f9}\x02\u{483}\x02\u{48c}\x02\u{531}\x02\u{533}\x02\
		\u{558}\x02\u{55b}\x02\u{55b}\x02\u{563}\x02\u{589}\x02\u{5b2}\x02\u{5bf}\
		\x02\u{5c1}\x02\u{5c1}\x02\u{5c3}\x02\u{5c4}\x02\u{5c6}\x02\u{5c7}\x02\
		\u{5c9}\x02\u{5c9}\x02\u{5d2}\x02\u{5ec}\x02\u{5f2}\x02\u{5f4}\x02\u{612}\
		\x02\u{61c}\x02\u{622}\x02\u{659}\x02\u{65b}\x02\u{66b}\x02\u{670}\x02\
		\u{6d5}\x02\u{6d7}\x02\u{6de}\x02\u{6e3}\x02\u{6ea}\x02\u{6ef}\x02\u{6fe}\
		\x02\u{701}\x02\u{701}\x02\u{712}\x02\u{741}\x02\u{74f}\x02\u{7b3}\x02\
		\u{7c2}\x02\u{7ec}\x02\u{7f6}\x02\u{7f7}\x02\u{7fc}\x02\u{7fc}\x02\u{802}\
		\x02\u{819}\x02\u{81c}\x02\u{82e}\x02\u{842}\x02\u{85a}\x02\u{862}\x02\
		\u{86c}\x02\u{8a2}\x02\u{8b6}\x02\u{8b8}\x02\u{8bf}\x02\u{8d6}\x02\u{8e1}\
		\x02\u{8e5}\x02\u{8eb}\x02\u{8f2}\x02\u{93d}\x02\u{93f}\x02\u{94e}\x02\
		\u{950}\x02\u{952}\x02\u{957}\x02\u{965}\x02\u{968}\x02\u{971}\x02\u{973}\
		\x02\u{985}\x02\u{987}\x02\u{98e}\x02\u{991}\x02\u{992}\x02\u{995}\x02\
		\u{9aa}\x02\u{9ac}\x02\u{9b2}\x02\u{9b4}\x02\u{9b4}\x02\u{9b8}\x02\u{9bb}\
		\x02\u{9bf}\x02\u{9c6}\x02\u{9c9}\x02\u{9ca}\x02\u{9cd}\x02\u{9ce}\x02\
		\u{9d0}\x02\u{9d0}\x02\u{9d9}\x02\u{9d9}\x02\u{9de}\x02\u{9df}\x02\u{9e1}\
		\x02\u{9e5}\x02\u{9e8}\x02\u{9f3}\x02\u{9fe}\x02\u{9fe}\x02\u{a03}\x02\
		\u{a05}\x02\u{a07}\x02\u{a0c}\x02\u{a11}\x02\u{a12}\x02\u{a15}\x02\u{a2a}\
		\x02\u{a2c}\x02\u{a32}\x02\u{a34}\x02\u{a35}\x02\u{a37}\x02\u{a38}\x02\
		\u{a3a}\x02\u{a3b}\x02\u{a40}\x02\u{a44}\x02\u{a49}\x02\u{a4a}\x02\u{a4d}\
		\x02\u{a4e}\x02\u{a53}\x02\u{a53}\x02\u{a5b}\x02\u{a5e}\x02\u{a60}\x02\
		\u{a60}\x02\u{a68}\x02\u{a77}\x02\u{a83}\x02\u{a85}\x02\u{a87}\x02\u{a8f}\
		\x02\u{a91}\x02\u{a93}\x02\u{a95}\x02\u{aaa}\x02\u{aac}\x02\u{ab2}\x02\
		\u{ab4}\x02\u{ab5}\x02\u{ab7}\x02\u{abb}\x02\u{abf}\x02\u{ac7}\x02\u{ac9}\
		\x02\u{acb}\x02\u{acd}\x02\u{ace}\x02\u{ad2}\x02\u{ad2}\x02\u{ae2}\x02\
		\u{ae5}\x02\u{ae8}\x02\u{af1}\x02\u{afb}\x02\u{afe}\x02\u{b03}\x02\u{b05}\
		\x02\u{b07}\x02\u{b0e}\x02\u{b11}\x02\u{b12}\x02\u{b15}\x02\u{b2a}\x02\
		\u{b2c}\x02\u{b32}\x02\u{b34}\x02\u{b35}\x02\u{b37}\x02\u{b3b}\x02\u{b3f}\
		\x02\u{b46}\x02\u{b49}\x02\u{b4a}\x02\u{b4d}\x02\u{b4e}\x02\u{b58}\x02\
		\u{b59}\x02\u{b5e}\x02\u{b5f}\x02\u{b61}\x02\u{b65}\x02\u{b68}\x02\u{b71}\
		\x02\u{b73}\x02\u{b73}\x02\u{b84}\x02\u{b85}\x02\u{b87}\x02\u{b8c}\x02\
		\u{b90}\x02\u{b92}\x02\u{b94}\x02\u{b97}\x02\u{b9b}\x02\u{b9c}\x02\u{b9e}\
		\x02\u{b9e}\x02\u{ba0}\x02\u{ba1}\x02\u{ba5}\x02\u{ba6}\x02\u{baa}\x02\
		\u{bac}\x02\u{bb0}\x02\u{bbb}\x02\u{bc0}\x02\u{bc4}\x02\u{bc8}\x02\u{bca}\
		\x02\u{bcc}\x02\u{bce}\x02\u{bd2}\x02\u{bd2}\x02\u{bd9}\x02\u{bd9}\x02\
		\u{be8}\x02\u{bf1}\x02\u{c02}\x02\u{c05}\x02\u{c07}\x02\u{c0e}\x02\u{c10}\
		\x02\u{c12}\x02\u{c14}\x02\u{c2a}\x02\u{c2c}\x02\u{c3b}\x02\u{c3f}\x02\
		\u{c46}\x02\u{c48}\x02\u{c4a}\x02\u{c4c}\x02\u{c4e}\x02\u{c57}\x02\u{c58}\
		\x02\u{c5a}\x02\u{c5c}\x02\u{c62}\x02\u{c65}\x02\u{c68}\x02\u{c71}\x02\
		\u{c82}\x02\u{c85}\x02\u{c87}\x02\u{c8e}\x02\u{c90}\x02\u{c92}\x02\u{c94}\
		\x02\u{caa}\x02\u{cac}\x02\u{cb5}\x02\u{cb7}\x02\u{cbb}\x02\u{cbf}\x02\
		\u{cc6}\x02\u{cc8}\x02\u{cca}\x02\u{ccc}\x02\u{cce}\x02\u{cd7}\x02\u{cd8}\
		\x02\u{ce0}\x02\u{ce0}\x02\u{ce2}\x02\u{ce5}\x02\u{ce8}\x02\u{cf1}\x02\
		\u{cf3}\x02\u{cf4}\x02\u{d02}\x02\u{d05}\x02\u{d07}\x02\u{d0e}\x02\u{d10}\
		\x02\u{d12}\x02\u{d14}\x02\u{d3c}\x02\u{d3f}\x02\u{d46}\x02\u{d48}\x02\
		\u{d4a}\x02\u{d4c}\x02\u{d4e}\x02\u{d50}\x02\u{d50}\x02\u{d56}\x02\u{d59}\
		\x02\u{d61}\x02\u{d65}\x02\u{d68}\x02\u{d71}\x02\u{d7c}\x02\u{d81}\x02\
		\u{d84}\x02\u{d85}\x02\u{d87}\x02\u{d98}\x02\u{d9c}\x02\u{db3}\x02\u{db5}\
		\x02\u{dbd}\x02\u{dbf}\x02\u{dbf}\x02\u{dc2}\x02\u{dc8}\x02\u{dd1}\x02\
		\u{dd6}\x02\u{dd8}\x02\u{dd8}\x02\u{dda}\x02\u{de1}\x02\u{de8}\x02\u{df1}\
		\x02\u{df4}\x02\u{df5}\x02\u{e03}\x02\u{e3c}\x02\u{e42}\x02\u{e48}\x02\
		\u{e4f}\x02\u{e4f}\x02\u{e52}\x02\u{e5b}\x02\u{e83}\x02\u{e84}\x02\u{e86}\
		\x02\u{e86}\x02\u{e89}\x02\u{e8a}\x02\u{e8c}\x02\u{e8c}\x02\u{e8f}\x02\
		\u{e8f}\x02\u{e96}\x02\u{e99}\x02\u{e9b}\x02\u{ea1}\x02\u{ea3}\x02\u{ea5}\
		\x02\u{ea7}\x02\u{ea7}\x02\u{ea9}\x02\u{ea9}\x02\u{eac}\x02\u{ead}\x02\
		\u{eaf}\x02\u{ebb}\x02\u{ebd}\x02\u{ebf}\x02\u{ec2}\x02\u{ec6}\x02\u{ec8}\
		\x02\u{ec8}\x02\u{ecf}\x02\u{ecf}\x02\u{ed2}\x02\u{edb}\x02\u{ede}\x02\
		\u{ee1}\x02\u{f02}\x02\u{f02}\x02\u{f22}\x02\u{f2b}\x02\u{f42}\x02\u{f49}\
		\x02\u{f4b}\x02\u{f6e}\x02\u{f73}\x02\u{f83}\x02\u{f8a}\x02\u{f99}\x02\
		\u{f9b}\x02\u{fbe}\x02\u{1002}\x02\u{1038}\x02\u{103a}\x02\u{103a}\x02\
		\u{103d}\x02\u{104b}\x02\u{1052}\x02\u{1064}\x02\u{1067}\x02\u{106a}\x02\
		\u{1070}\x02\u{1088}\x02\u{1090}\x02\u{1090}\x02\u{1092}\x02\u{109b}\x02\
		\u{109e}\x02\u{109f}\x02\u{10a2}\x02\u{10c7}\x02\u{10c9}\x02\u{10c9}\x02\
		\u{10cf}\x02\u{10cf}\x02\u{10d2}\x02\u{10fc}\x02\u{10fe}\x02\u{124a}\x02\
		\u{124c}\x02\u{124f}\x02\u{1252}\x02\u{1258}\x02\u{125a}\x02\u{125a}\x02\
		\u{125c}\x02\u{125f}\x02\u{1262}\x02\u{128a}\x02\u{128c}\x02\u{128f}\x02\
		\u{1292}\x02\u{12b2}\x02\u{12b4}\x02\u{12b7}\x02\u{12ba}\x02\u{12c0}\x02\
		\u{12c2}\x02\u{12c2}\x02\u{12c4}\x02\u{12c7}\x02\u{12ca}\x02\u{12d8}\x02\
		\u{12da}\x02\u{1312}\x02\u{1314}\x02\u{1317}\x02\u{131a}\x02\u{135c}\x02\
		\u{1361}\x02\u{1361}\x02\u{1382}\x02\u{1391}\x02\u{13a2}\x02\u{13f7}\x02\
		\u{13fa}\x02\u{13ff}\x02\u{1403}\x02\u{166e}\x02\u{1671}\x02\u{1681}\x02\
		\u{1683}\x02\u{169c}\x02\u{16a2}\x02\u{16ec}\x02\u{16f0}\x02\u{16fa}\x02\
		\u{1702}\x02\u{170e}\x02\u{1710}\x02\u{1715}\x02\u{1722}\x02\u{1735}\x02\
		\u{1742}\x02\u{1755}\x02\u{1762}\x02\u{176e}\x02\u{1770}\x02\u{1772}\x02\
		\u{1774}\x02\u{1775}\x02\u{1782}\x02\u{17b5}\x02\u{17b8}\x02\u{17ca}\x02\
		\u{17d9}\x02\u{17d9}\x02\u{17de}\x02\u{17de}\x02\u{17e2}\x02\u{17eb}\x02\
		\u{1812}\x02\u{181b}\x02\u{1822}\x02\u{1879}\x02\u{1882}\x02\u{18ac}\x02\
		\u{18b2}\x02\u{18f7}\x02\u{1902}\x02\u{1920}\x02\u{1922}\x02\u{192d}\x02\
		\u{1932}\x02\u{193a}\x02\u{1948}\x02\u{196f}\x02\u{1972}\x02\u{1976}\x02\
		\u{1982}\x02\u{19ad}\x02\u{19b2}\x02\u{19cb}\x02\u{19d2}\x02\u{19db}\x02\
		\u{1a02}\x02\u{1a1d}\x02\u{1a22}\x02\u{1a60}\x02\u{1a63}\x02\u{1a76}\x02\
		\u{1a82}\x02\u{1a8b}\x02\u{1a92}\x02\u{1a9b}\x02\u{1aa9}\x02\u{1aa9}\x02\
		\u{1b02}\x02\u{1b35}\x02\u{1b37}\x02\u{1b45}\x02\u{1b47}\x02\u{1b4d}\x02\
		\u{1b52}\x02\u{1b5b}\x02\u{1b82}\x02\u{1bab}\x02\u{1bae}\x02\u{1be7}\x02\
		\u{1be9}\x02\u{1bf3}\x02\u{1c02}\x02\u{1c37}\x02\u{1c42}\x02\u{1c4b}\x02\
		\u{1c4f}\x02\u{1c7f}\x02\u{1c82}\x02\u{1c8a}\x02\u{1ceb}\x02\u{1cee}\x02\
		\u{1cf0}\x02\u{1cf5}\x02\u{1cf7}\x02\u{1cf8}\x02\u{1d02}\x02\u{1dc1}\x02\
		\u{1de9}\x02\u{1df6}\x02\u{1e02}\x02\u{1f17}\x02\u{1f1a}\x02\u{1f1f}\x02\
		\u{1f22}\x02\u{1f47}\x02\u{1f4a}\x02\u{1f4f}\x02\u{1f52}\x02\u{1f59}\x02\
		\u{1f5b}\x02\u{1f5b}\x02\u{1f5d}\x02\u{1f5d}\x02\u{1f5f}\x02\u{1f5f}\x02\
		\u{1f61}\x02\u{1f7f}\x02\u{1f82}\x02\u{1fb6}\x02\u{1fb8}\x02\u{1fbe}\x02\
		\u{1fc0}\x02\u{1fc0}\x02\u{1fc4}\x02\u{1fc6}\x02\u{1fc8}\x02\u{1fce}\x02\
		\u{1fd2}\x02\u{1fd5}\x02\u{1fd8}\x02\u{1fdd}\x02\u{1fe2}\x02\u{1fee}\x02\
		\u{1ff4}\x02\u{1ff6}\x02\u{1ff8}\x02\u{1ffe}\x02\u{2073}\x02\u{2073}\x02\
		\u{2081}\x02\u{2081}\x02\u{2092}\x02\u{209e}\x02\u{2104}\x02\u{2104}\x02\
		\u{2109}\x02\u{2109}\x02\u{210c}\x02\u{2115}\x02\u{2117}\x02\u{2117}\x02\
		\u{211b}\x02\u{211f}\x02\u{2126}\x02\u{2126}\x02\u{2128}\x02\u{2128}\x02\
		\u{212a}\x02\u{212a}\x02\u{212c}\x02\u{212f}\x02\u{2131}\x02\u{213b}\x02\
		\u{213e}\x02\u{2141}\x02\u{2147}\x02\u{214b}\x02\u{2150}\x02\u{2150}\x02\
		\u{2162}\x02\u{218a}\x02\u{24b8}\x02\u{24eb}\x02\u{2c02}\x02\u{2c30}\x02\
		\u{2c32}\x02\u{2c60}\x02\u{2c62}\x02\u{2ce6}\x02\u{2ced}\x02\u{2cf0}\x02\
		\u{2cf4}\x02\u{2cf5}\x02\u{2d02}\x02\u{2d27}\x02\u{2d29}\x02\u{2d29}\x02\
		\u{2d2f}\x02\u{2d2f}\x02\u{2d32}\x02\u{2d69}\x02\u{2d71}\x02\u{2d71}\x02\
		\u{2d82}\x02\u{2d98}\x02\u{2da2}\x02\u{2da8}\x02\u{2daa}\x02\u{2db0}\x02\
		\u{2db2}\x02\u{2db8}\x02\u{2dba}\x02\u{2dc0}\x02\u{2dc2}\x02\u{2dc8}\x02\
		\u{2dca}\x02\u{2dd0}\x02\u{2dd2}\x02\u{2dd8}\x02\u{2dda}\x02\u{2de0}\x02\
		\u{2de2}\x02\u{2e01}\x02\u{2e31}\x02\u{2e31}\x02\u{3007}\x02\u{3009}\x02\
		\u{3023}\x02\u{302b}\x02\u{3033}\x02\u{3037}\x02\u{303a}\x02\u{303e}\x02\
		\u{3043}\x02\u{3098}\x02\u{309f}\x02\u{30a1}\x02\u{30a3}\x02\u{30fc}\x02\
		\u{30fe}\x02\u{3101}\x02\u{3107}\x02\u{3130}\x02\u{3133}\x02\u{3190}\x02\
		\u{31a2}\x02\u{31bc}\x02\u{31f2}\x02\u{3201}\x02\u{3402}\x02\u{4db7}\x02\
		\u{4e02}\x02\u{9fec}\x02\u{a002}\x02\u{a48e}\x02\u{a4d2}\x02\u{a4ff}\x02\
		\u{a502}\x02\u{a60e}\x02\u{a612}\x02\u{a62d}\x02\u{a642}\x02\u{a670}\x02\
		\u{a676}\x02\u{a67d}\x02\u{a681}\x02\u{a6f1}\x02\u{a719}\x02\u{a721}\x02\
		\u{a724}\x02\u{a78a}\x02\u{a78d}\x02\u{a7b0}\x02\u{a7b2}\x02\u{a7b9}\x02\
		\u{a7f9}\x02\u{a803}\x02\u{a805}\x02\u{a807}\x02\u{a809}\x02\u{a80c}\x02\
		\u{a80e}\x02\u{a829}\x02\u{a842}\x02\u{a875}\x02\u{a882}\x02\u{a8c5}\x02\
		\u{a8c7}\x02\u{a8c7}\x02\u{a8d2}\x02\u{a8db}\x02\u{a8f4}\x02\u{a8f9}\x02\
		\u{a8fd}\x02\u{a8fd}\x02\u{a8ff}\x02\u{a8ff}\x02\u{a902}\x02\u{a92c}\x02\
		\u{a932}\x02\u{a954}\x02\u{a962}\x02\u{a97e}\x02\u{a982}\x02\u{a9b4}\x02\
		\u{a9b6}\x02\u{a9c1}\x02\u{a9d1}\x02\u{a9db}\x02\u{a9e2}\x02\u{a9e6}\x02\
		\u{a9e8}\x02\u{aa00}\x02\u{aa02}\x02\u{aa38}\x02\u{aa42}\x02\u{aa4f}\x02\
		\u{aa52}\x02\u{aa5b}\x02\u{aa62}\x02\u{aa78}\x02\u{aa7c}\x02\u{aa7c}\x02\
		\u{aa80}\x02\u{aac0}\x02\u{aac2}\x02\u{aac2}\x02\u{aac4}\x02\u{aac4}\x02\
		\u{aadd}\x02\u{aadf}\x02\u{aae2}\x02\u{aaf1}\x02\u{aaf4}\x02\u{aaf7}\x02\
		\u{ab03}\x02\u{ab08}\x02\u{ab0b}\x02\u{ab10}\x02\u{ab13}\x02\u{ab18}\x02\
		\u{ab22}\x02\u{ab28}\x02\u{ab2a}\x02\u{ab30}\x02\u{ab32}\x02\u{ab5c}\x02\
		\u{ab5e}\x02\u{ab67}\x02\u{ab72}\x02\u{abec}\x02\u{abf2}\x02\u{abfb}\x02\
		\u{ac02}\x02\u{d7a5}\x02\u{d7b2}\x02\u{d7c8}\x02\u{d7cd}\x02\u{d7fd}\x02\
		\u{f902}\x02\u{fa6f}\x02\u{fa72}\x02\u{fadb}\x02\u{fb02}\x02\u{fb08}\x02\
		\u{fb15}\x02\u{fb19}\x02\u{fb1f}\x02\u{fb2a}\x02\u{fb2c}\x02\u{fb38}\x02\
		\u{fb3a}\x02\u{fb3e}\x02\u{fb40}\x02\u{fb40}\x02\u{fb42}\x02\u{fb43}\x02\
		\u{fb45}\x02\u{fb46}\x02\u{fb48}\x02\u{fbb3}\x02\u{fbd5}\x02\u{fd3f}\x02\
		\u{fd52}\x02\u{fd91}\x02\u{fd94}\x02\u{fdc9}\x02\u{fdf2}\x02\u{fdfd}\x02\
		\u{fe72}\x02\u{fe76}\x02\u{fe78}\x02\u{fefe}\x02\u{ff12}\x02\u{ff1b}\x02\
		\u{ff23}\x02\u{ff3c}\x02\u{ff43}\x02\u{ff5c}\x02\u{ff68}\x02\u{ffc0}\x02\
		\u{ffc4}\x02\u{ffc9}\x02\u{ffcc}\x02\u{ffd1}\x02\u{ffd4}\x02\u{ffd9}\x02\
		\u{ffdc}\x02\u{ffde}\x02\x02\x03\x0d\x03\x0f\x03\x28\x03\x2a\x03\x3c\x03\
		\x3e\x03\x3f\x03\x41\x03\x4f\x03\x52\x03\x5f\x03\u{82}\x03\u{fc}\x03\u{142}\
		\x03\u{176}\x03\u{282}\x03\u{29e}\x03\u{2a2}\x03\u{2d2}\x03\u{302}\x03\
		\u{321}\x03\u{32f}\x03\u{34c}\x03\u{352}\x03\u{37c}\x03\u{382}\x03\u{39f}\
		\x03\u{3a2}\x03\u{3c5}\x03\u{3ca}\x03\u{3d1}\x03\u{3d3}\x03\u{3d7}\x03\
		\u{402}\x03\u{49f}\x03\u{4a2}\x03\u{4ab}\x03\u{4b2}\x03\u{4d5}\x03\u{4da}\
		\x03\u{4fd}\x03\u{502}\x03\u{529}\x03\u{532}\x03\u{565}\x03\u{602}\x03\
		\u{738}\x03\u{742}\x03\u{757}\x03\u{762}\x03\u{769}\x03\u{802}\x03\u{807}\
		\x03\u{80a}\x03\u{80a}\x03\u{80c}\x03\u{837}\x03\u{839}\x03\u{83a}\x03\
		\u{83e}\x03\u{83e}\x03\u{841}\x03\u{857}\x03\u{862}\x03\u{878}\x03\u{882}\
		\x03\u{8a0}\x03\u{8e2}\x03\u{8f4}\x03\u{8f6}\x03\u{8f7}\x03\u{902}\x03\
		\u{917}\x03\u{922}\x03\u{93b}\x03\u{982}\x03\u{9b9}\x03\u{9c0}\x03\u{9c1}\
		\x03\u{a02}\x03\u{a05}\x03\u{a07}\x03\u{a08}\x03\u{a0e}\x03\u{a15}\x03\
		\u{a17}\x03\u{a19}\x03\u{a1b}\x03\u{a35}\x03\u{a62}\x03\u{a7e}\x03\u{a82}\
		\x03\u{a9e}\x03\u{ac2}\x03\u{ac9}\x03\u{acb}\x03\u{ae6}\x03\u{b02}\x03\
		\u{b37}\x03\u{b42}\x03\u{b57}\x03\u{b62}\x03\u{b74}\x03\u{b82}\x03\u{b93}\
		\x03\u{c02}\x03\u{c4a}\x03\u{c82}\x03\u{cb4}\x03\u{cc2}\x03\u{cf4}\x03\
		\u{1002}\x03\u{1047}\x03\u{1068}\x03\u{1071}\x03\u{1084}\x03\u{10ba}\x03\
		\u{10d2}\x03\u{10ea}\x03\u{10f2}\x03\u{10fb}\x03\u{1102}\x03\u{1134}\x03\
		\u{1138}\x03\u{1141}\x03\u{1152}\x03\u{1174}\x03\u{1178}\x03\u{1178}\x03\
		\u{1182}\x03\u{11c1}\x03\u{11c3}\x03\u{11c6}\x03\u{11d2}\x03\u{11dc}\x03\
		\u{11de}\x03\u{11de}\x03\u{1202}\x03\u{1213}\x03\u{1215}\x03\u{1236}\x03\
		\u{1239}\x03\u{1239}\x03\u{1240}\x03\u{1240}\x03\u{1282}\x03\u{1288}\x03\
		\u{128a}\x03\u{128a}\x03\u{128c}\x03\u{128f}\x03\u{1291}\x03\u{129f}\x03\
		\u{12a1}\x03\u{12aa}\x03\u{12b2}\x03\u{12ea}\x03\u{12f2}\x03\u{12fb}\x03\
		\u{1302}\x03\u{1305}\x03\u{1307}\x03\u{130e}\x03\u{1311}\x03\u{1312}\x03\
		\u{1315}\x03\u{132a}\x03\u{132c}\x03\u{1332}\x03\u{1334}\x03\u{1335}\x03\
		\u{1337}\x03\u{133b}\x03\u{133f}\x03\u{1346}\x03\u{1349}\x03\u{134a}\x03\
		\u{134d}\x03\u{134e}\x03\u{1352}\x03\u{1352}\x03\u{1359}\x03\u{1359}\x03\
		\u{135f}\x03\u{1365}\x03\u{1402}\x03\u{1443}\x03\u{1445}\x03\u{1447}\x03\
		\u{1449}\x03\u{144c}\x03\u{1452}\x03\u{145b}\x03\u{1482}\x03\u{14c3}\x03\
		\u{14c6}\x03\u{14c7}\x03\u{14c9}\x03\u{14c9}\x03\u{14d2}\x03\u{14db}\x03\
		\u{1582}\x03\u{15b7}\x03\u{15ba}\x03\u{15c0}\x03\u{15da}\x03\u{15df}\x03\
		\u{1602}\x03\u{1640}\x03\u{1642}\x03\u{1642}\x03\u{1646}\x03\u{1646}\x03\
		\u{1652}\x03\u{165b}\x03\u{1682}\x03\u{16b7}\x03\u{16c2}\x03\u{16cb}\x03\
		\u{1702}\x03\u{171b}\x03\u{171f}\x03\u{172c}\x03\u{1732}\x03\u{173b}\x03\
		\u{18a2}\x03\u{18eb}\x03\u{1901}\x03\u{1901}\x03\u{1a02}\x03\u{1a34}\x03\
		\u{1a37}\x03\u{1a40}\x03\u{1a52}\x03\u{1a85}\x03\u{1a88}\x03\u{1a99}\x03\
		\u{1ac2}\x03\u{1afa}\x03\u{1c02}\x03\u{1c0a}\x03\u{1c0c}\x03\u{1c38}\x03\
		\u{1c3a}\x03\u{1c40}\x03\u{1c42}\x03\u{1c42}\x03\u{1c52}\x03\u{1c5b}\x03\
		\u{1c74}\x03\u{1c91}\x03\u{1c94}\x03\u{1ca9}\x03\u{1cab}\x03\u{1cb8}\x03\
		\u{1d02}\x03\u{1d08}\x03\u{1d0a}\x03\u{1d0b}\x03\u{1d0d}\x03\u{1d38}\x03\
		\u{1d3c}\x03\u{1d3c}\x03\u{1d3e}\x03\u{1d3f}\x03\u{1d41}\x03\u{1d43}\x03\
		\u{1d45}\x03\u{1d45}\x03\u{1d48}\x03\u{1d49}\x03\u{1d52}\x03\u{1d5b}\x03\
		\u{2002}\x03\u{239b}\x03\u{2402}\x03\u{2470}\x03\u{2482}\x03\u{2545}\x03\
		\u{3002}\x03\u{3430}\x03\u{4402}\x03\u{4648}\x03\u{6802}\x03\u{6a3a}\x03\
		\u{6a42}\x03\u{6a60}\x03\u{6a62}\x03\u{6a6b}\x03\u{6ad2}\x03\u{6aef}\x03\
		\u{6b02}\x03\u{6b38}\x03\u{6b42}\x03\u{6b45}\x03\u{6b52}\x03\u{6b5b}\x03\
		\u{6b65}\x03\u{6b79}\x03\u{6b7f}\x03\u{6b91}\x03\u{6f02}\x03\u{6f46}\x03\
		\u{6f52}\x03\u{6f80}\x03\u{6f95}\x03\u{6fa1}\x03\u{6fe2}\x03\u{6fe3}\x03\
		\u{7002}\x03\u{87ee}\x03\u{8802}\x03\u{8af4}\x03\u{b002}\x03\u{b120}\x03\
		\u{b172}\x03\u{b2fd}\x03\u{bc02}\x03\u{bc6c}\x03\u{bc72}\x03\u{bc7e}\x03\
		\u{bc82}\x03\u{bc8a}\x03\u{bc92}\x03\u{bc9b}\x03\u{bca0}\x03\u{bca0}\x03\
		\u{d402}\x03\u{d456}\x03\u{d458}\x03\u{d49e}\x03\u{d4a0}\x03\u{d4a1}\x03\
		\u{d4a4}\x03\u{d4a4}\x03\u{d4a7}\x03\u{d4a8}\x03\u{d4ab}\x03\u{d4ae}\x03\
		\u{d4b0}\x03\u{d4bb}\x03\u{d4bd}\x03\u{d4bd}\x03\u{d4bf}\x03\u{d4c5}\x03\
		\u{d4c7}\x03\u{d507}\x03\u{d509}\x03\u{d50c}\x03\u{d50f}\x03\u{d516}\x03\
		\u{d518}\x03\u{d51e}\x03\u{d520}\x03\u{d53b}\x03\u{d53d}\x03\u{d540}\x03\
		\u{d542}\x03\u{d546}\x03\u{d548}\x03\u{d548}\x03\u{d54c}\x03\u{d552}\x03\
		\u{d554}\x03\u{d6a7}\x03\u{d6aa}\x03\u{d6c2}\x03\u{d6c4}\x03\u{d6dc}\x03\
		\u{d6de}\x03\u{d6fc}\x03\u{d6fe}\x03\u{d716}\x03\u{d718}\x03\u{d736}\x03\
		\u{d738}\x03\u{d750}\x03\u{d752}\x03\u{d770}\x03\u{d772}\x03\u{d78a}\x03\
		\u{d78c}\x03\u{d7aa}\x03\u{d7ac}\x03\u{d7c4}\x03\u{d7c6}\x03\u{d7cd}\x03\
		\u{d7d0}\x03\u{10801}\x03\u{e002}\x03\u{e008}\x03\u{e00a}\x03\u{e01a}\x03\
		\u{e01d}\x03\u{e023}\x03\u{e025}\x03\u{e026}\x03\u{e028}\x03\u{e02c}\x03\
		\u{e802}\x03\u{e8c6}\x03\u{e902}\x03\u{e945}\x03\u{e949}\x03\u{e949}\x03\
		\u{e952}\x03\u{e95b}\x03\u{ee02}\x03\u{ee05}\x03\u{ee07}\x03\u{ee21}\x03\
		\u{ee23}\x03\u{ee24}\x03\u{ee26}\x03\u{ee26}\x03\u{ee29}\x03\u{ee29}\x03\
		\u{ee2b}\x03\u{ee34}\x03\u{ee36}\x03\u{ee39}\x03\u{ee3b}\x03\u{ee3b}\x03\
		\u{ee3d}\x03\u{ee3d}\x03\u{ee44}\x03\u{ee44}\x03\u{ee49}\x03\u{ee49}\x03\
		\u{ee4b}\x03\u{ee4b}\x03\u{ee4d}\x03\u{ee4d}\x03\u{ee4f}\x03\u{ee51}\x03\
		\u{ee53}\x03\u{ee54}\x03\u{ee56}\x03\u{ee56}\x03\u{ee59}\x03\u{ee59}\x03\
		\u{ee5b}\x03\u{ee5b}\x03\u{ee5d}\x03\u{ee5d}\x03\u{ee5f}\x03\u{ee5f}\x03\
		\u{ee61}\x03\u{ee61}\x03\u{ee63}\x03\u{ee64}\x03\u{ee66}\x03\u{ee66}\x03\
		\u{ee69}\x03\u{ee6c}\x03\u{ee6e}\x03\u{ee74}\x03\u{ee76}\x03\u{ee79}\x03\
		\u{ee7b}\x03\u{ee7e}\x03\u{ee80}\x03\u{ee80}\x03\u{ee82}\x03\u{ee8b}\x03\
		\u{ee8d}\x03\u{ee9d}\x03\u{eea3}\x03\u{eea5}\x03\u{eea7}\x03\u{eeab}\x03\
		\u{eead}\x03\u{eebd}\x03\u{f132}\x03\u{f14b}\x03\u{f152}\x03\u{f16b}\x03\
		\u{f172}\x03\u{f18b}\x03\x02\x04\u{a6d8}\x04\u{a702}\x04\u{b736}\x04\u{b742}\
		\x04\u{b81f}\x04\u{b822}\x04\u{cea3}\x04\u{ceb2}\x04\u{ebe2}\x04\u{f802}\
		\x04\u{fa1f}\x04\u{101}\x02\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\x02\
		\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\x02\
		\x02\x0d\x03\x02\x02\x02\x02\x0f\x03\x02\x02\x02\x02\x11\x03\x02\x02\x02\
		\x02\x13\x03\x02\x02\x02\x02\x15\x03\x02\x02\x02\x02\x17\x03\x02\x02\x02\
		\x02\x19\x03\x02\x02\x02\x02\x1b\x03\x02\x02\x02\x02\x1d\x03\x02\x02\x02\
		\x02\x1f\x03\x02\x02\x02\x02\x21\x03\x02\x02\x02\x02\x23\x03\x02\x02\x02\
		\x02\x25\x03\x02\x02\x02\x02\x27\x03\x02\x02\x02\x02\x29\x03\x02\x02\x02\
		\x02\x2b\x03\x02\x02\x02\x02\x2d\x03\x02\x02\x02\x02\x2f\x03\x02\x02\x02\
		\x02\x31\x03\x02\x02\x02\x02\x33\x03\x02\x02\x02\x02\x35\x03\x02\x02\x02\
		\x02\x37\x03\x02\x02\x02\x02\x39\x03\x02\x02\x02\x02\x3b\x03\x02\x02\x02\
		\x03\x3d\x03\x02\x02\x02\x05\x40\x03\x02\x02\x02\x07\x45\x03\x02\x02\x02\
		\x09\x4b\x03\x02\x02\x02\x0b\x4e\x03\x02\x02\x02\x0d\x52\x03\x02\x02\x02\
		\x0f\x56\x03\x02\x02\x02\x11\x5d\x03\x02\x02\x02\x13\x64\x03\x02\x02\x02\
		\x15\x66\x03\x02\x02\x02\x17\x68\x03\x02\x02\x02\x19\x6a\x03\x02\x02\x02\
		\x1b\x6c\x03\x02\x02\x02\x1d\x6e\x03\x02\x02\x02\x1f\x70\x03\x02\x02\x02\
		\x21\x72\x03\x02\x02\x02\x23\x7b\x03\x02\x02\x02\x25\u{83}\x03\x02\x02\
		\x02\x27\u{8a}\x03\x02\x02\x02\x29\u{92}\x03\x02\x02\x02\x2b\u{9a}\x03\
		\x02\x02\x02\x2d\u{9c}\x03\x02\x02\x02\x2f\u{9e}\x03\x02\x02\x02\x31\u{a0}\
		\x03\x02\x02\x02\x33\u{a3}\x03\x02\x02\x02\x35\u{a5}\x03\x02\x02\x02\x37\
		\u{c4}\x03\x02\x02\x02\x39\u{e4}\x03\x02\x02\x02\x3b\u{e7}\x03\x02\x02\
		\x02\x3d\x3e\x07\x6b\x02\x02\x3e\x3f\x07\x68\x02\x02\x3f\x04\x03\x02\x02\
		\x02\x40\x41\x07\x67\x02\x02\x41\x42\x07\x6e\x02\x02\x42\x43\x07\x75\x02\
		\x02\x43\x44\x07\x67\x02\x02\x44\x06\x03\x02\x02\x02\x45\x46\x07\x79\x02\
		\x02\x46\x47\x07\x6a\x02\x02\x47\x48\x07\x6b\x02\x02\x48\x49\x07\x6e\x02\
		\x02\x49\x4a\x07\x67\x02\x02\x4a\x08\x03\x02\x02\x02\x4b\x4c\x07\x68\x02\
		\x02\x4c\x4d\x07\x70\x02\x02\x4d\x0a\x03\x02\x02\x02\x4e\x4f\x07\x6e\x02\
		\x02\x4f\x50\x07\x67\x02\x02\x50\x51\x07\x76\x02\x02\x51\x0c\x03\x02\x02\
		\x02\x52\x53\x07\x78\x02\x02\x53\x54\x07\x63\x02\x02\x54\x55\x07\x74\x02\
		\x02\x55\x0e\x03\x02\x02\x02\x56\x57\x07\x74\x02\x02\x57\x58\x07\x67\x02\
		\x02\x58\x59\x07\x76\x02\x02\x59\x5a\x07\x77\x02\x02\x5a\x5b\x07\x74\x02\
		\x02\x5b\x5c\x07\x70\x02\x02\x5c\x10\x03\x02\x02\x02\x5d\x5e\x07\x6b\x02\
		\x02\x5e\x5f\x07\x6f\x02\x02\x5f\x60\x07\x72\x02\x02\x60\x61\x07\x71\x02\
		\x02\x61\x62\x07\x74\x02\x02\x62\x63\x07\x76\x02\x02\x63\x12\x03\x02\x02\
		\x02\x64\x65\x07\x2a\x02\x02\x65\x14\x03\x02\x02\x02\x66\x67\x07\x2b\x02\
		\x02\x67\x16\x03\x02\x02\x02\x68\x69\x07\x5d\x02\x02\x69\x18\x03\x02\x02\
		\x02\x6a\x6b\x07\x5f\x02\x02\x6b\x1a\x03\x02\x02\x02\x6c\x6d\x07\x7d\x02\
		\x02\x6d\x1c\x03\x02\x02\x02\x6e\x6f\x07\x7f\x02\x02\x6f\x1e\x03\x02\x02\
		\x02\x70\x71\x07\x3f\x02\x02\x71\x20\x03\x02\x02\x02\x72\x73\x07\x3f\x02\
		\x02\x73\x74\x07\x3f\x02\x02\x74\x22\x03\x02\x02\x02\x75\x76\x07\x62\x02\
		\x02\x76\x77\x07\x6f\x02\x02\x77\x78\x07\x71\x02\x02\x78\x79\x07\x66\x02\
		\x02\x79\x7c\x07\x62\x02\x02\x7a\x7c\x07\x27\x02\x02\x7b\x75\x03\x02\x02\
		\x02\x7b\x7a\x03\x02\x02\x02\x7c\x24\x03\x02\x02\x02\x7d\x7e\x07\x62\x02\
		\x02\x7e\x7f\x07\x63\x02\x02\x7f\u{80}\x07\x70\x02\x02\u{80}\u{81}\x07\
		\x66\x02\x02\u{81}\u{84}\x07\x62\x02\x02\u{82}\u{84}\x07\x28\x02\x02\u{83}\
		\x7d\x03\x02\x02\x02\u{83}\u{82}\x03\x02\x02\x02\u{84}\x26\x03\x02\x02\
		\x02\u{85}\u{86}\x07\x62\x02\x02\u{86}\u{87}\x07\x71\x02\x02\u{87}\u{88}\
		\x07\x74\x02\x02\u{88}\u{8b}\x07\x62\x02\x02\u{89}\u{8b}\x07\x7e\x02\x02\
		\u{8a}\u{85}\x03\x02\x02\x02\u{8a}\u{89}\x03\x02\x02\x02\u{8b}\x28\x03\
		\x02\x02\x02\u{8c}\u{8d}\x07\x62\x02\x02\u{8d}\u{8e}\x07\x66\x02\x02\u{8e}\
		\u{8f}\x07\x6b\x02\x02\u{8f}\u{90}\x07\x78\x02\x02\u{90}\u{93}\x07\x62\
		\x02\x02\u{91}\u{93}\x07\x31\x02\x02\u{92}\u{8c}\x03\x02\x02\x02\u{92}\
		\u{91}\x03\x02\x02\x02\u{93}\x2a\x03\x02\x02\x02\u{94}\u{95}\x07\x62\x02\
		\x02\u{95}\u{96}\x07\x6f\x02\x02\u{96}\u{97}\x07\x77\x02\x02\u{97}\u{98}\
		\x07\x6e\x02\x02\u{98}\u{9b}\x07\x62\x02\x02\u{99}\u{9b}\x07\x2c\x02\x02\
		\u{9a}\u{94}\x03\x02\x02\x02\u{9a}\u{99}\x03\x02\x02\x02\u{9b}\x2c\x03\
		\x02\x02\x02\u{9c}\u{9d}\x07\x2d\x02\x02\u{9d}\x2e\x03\x02\x02\x02\u{9e}\
		\u{9f}\x07\x2f\x02\x02\u{9f}\x30\x03\x02\x02\x02\u{a0}\u{a1}\x07\x3c\x02\
		\x02\u{a1}\u{a2}\x07\x3c\x02\x02\u{a2}\x32\x03\x02\x02\x02\u{a3}\u{a4}\
		\x07\x3d\x02\x02\u{a4}\x34\x03\x02\x02\x02\u{a5}\u{a9}\x09\x09\x02\x02\
		\u{a6}\u{a8}\x09\x0a\x02\x02\u{a7}\u{a6}\x03\x02\x02\x02\u{a8}\u{ab}\x03\
		\x02\x02\x02\u{a9}\u{a7}\x03\x02\x02\x02\u{a9}\u{aa}\x03\x02\x02\x02\u{aa}\
		\x36\x03\x02\x02\x02\u{ab}\u{a9}\x03\x02\x02\x02\u{ac}\u{b4}\x07\x29\x02\
		\x02\u{ad}\u{ae}\x07\x5e\x02\x02\u{ae}\u{b3}\x07\x5e\x02\x02\u{af}\u{b0}\
		\x07\x5e\x02\x02\u{b0}\u{b3}\x07\x29\x02\x02\u{b1}\u{b3}\x0a\x02\x02\x02\
		\u{b2}\u{ad}\x03\x02\x02\x02\u{b2}\u{af}\x03\x02\x02\x02\u{b2}\u{b1}\x03\
		\x02\x02\x02\u{b3}\u{b6}\x03\x02\x02\x02\u{b4}\u{b2}\x03\x02\x02\x02\u{b4}\
		\u{b5}\x03\x02\x02\x02\u{b5}\u{b7}\x03\x02\x02\x02\u{b6}\u{b4}\x03\x02\
		\x02\x02\u{b7}\u{c5}\x07\x29\x02\x02\u{b8}\u{c0}\x07\x24\x02\x02\u{b9}\
		\u{ba}\x07\x5e\x02\x02\u{ba}\u{bf}\x07\x5e\x02\x02\u{bb}\u{bc}\x07\x5e\
		\x02\x02\u{bc}\u{bf}\x07\x24\x02\x02\u{bd}\u{bf}\x0a\x02\x02\x02\u{be}\
		\u{b9}\x03\x02\x02\x02\u{be}\u{bb}\x03\x02\x02\x02\u{be}\u{bd}\x03\x02\
		\x02\x02\u{bf}\u{c2}\x03\x02\x02\x02\u{c0}\u{be}\x03\x02\x02\x02\u{c0}\
		\u{c1}\x03\x02\x02\x02\u{c1}\u{c3}\x03\x02\x02\x02\u{c2}\u{c0}\x03\x02\
		\x02\x02\u{c3}\u{c5}\x07\x24\x02\x02\u{c4}\u{ac}\x03\x02\x02\x02\u{c4}\
		\u{b8}\x03\x02\x02\x02\u{c5}\x38\x03\x02\x02\x02\u{c6}\u{ca}\x07\x32\x02\
		\x02\u{c7}\u{c9}\x09\x03\x02\x02\u{c8}\u{c7}\x03\x02\x02\x02\u{c9}\u{cc}\
		\x03\x02\x02\x02\u{ca}\u{c8}\x03\x02\x02\x02\u{ca}\u{cb}\x03\x02\x02\x02\
		\u{cb}\u{e5}\x03\x02\x02\x02\u{cc}\u{ca}\x03\x02\x02\x02\u{cd}\u{ce}\x07\
		\x32\x02\x02\u{ce}\u{cf}\x07\x7a\x02\x02\u{cf}\u{d1}\x03\x02\x02\x02\u{d0}\
		\u{d2}\x09\x04\x02\x02\u{d1}\u{d0}\x03\x02\x02\x02\u{d2}\u{d3}\x03\x02\
		\x02\x02\u{d3}\u{d1}\x03\x02\x02\x02\u{d3}\u{d4}\x03\x02\x02\x02\u{d4}\
		\u{e5}\x03\x02\x02\x02\u{d5}\u{d6}\x07\x32\x02\x02\u{d6}\u{d7}\x07\x64\
		\x02\x02\u{d7}\u{d9}\x03\x02\x02\x02\u{d8}\u{da}\x09\x05\x02\x02\u{d9}\
		\u{d8}\x03\x02\x02\x02\u{da}\u{db}\x03\x02\x02\x02\u{db}\u{d9}\x03\x02\
		\x02\x02\u{db}\u{dc}\x03\x02\x02\x02\u{dc}\u{e5}\x03\x02\x02\x02\u{dd}\
		\u{e1}\x09\x06\x02\x02\u{de}\u{e0}\x09\x07\x02\x02\u{df}\u{de}\x03\x02\
		\x02\x02\u{e0}\u{e3}\x03\x02\x02\x02\u{e1}\u{df}\x03\x02\x02\x02\u{e1}\
		\u{e2}\x03\x02\x02\x02\u{e2}\u{e5}\x03\x02\x02\x02\u{e3}\u{e1}\x03\x02\
		\x02\x02\u{e4}\u{c6}\x03\x02\x02\x02\u{e4}\u{cd}\x03\x02\x02\x02\u{e4}\
		\u{d5}\x03\x02\x02\x02\u{e4}\u{dd}\x03\x02\x02\x02\u{e5}\x3a\x03\x02\x02\
		\x02\u{e6}\u{e8}\x09\x08\x02\x02\u{e7}\u{e6}\x03\x02\x02\x02\u{e8}\u{e9}\
		\x03\x02\x02\x02\u{e9}\u{e7}\x03\x02\x02\x02\u{e9}\u{ea}\x03\x02\x02\x02\
		\u{ea}\u{eb}\x03\x02\x02\x02\u{eb}\u{ec}\x08\x1e\x02\x02\u{ec}\x3c\x03\
		\x02\x02\x02\x14\x02\x7b\u{83}\u{8a}\u{92}\u{9a}\u{a9}\u{b2}\u{b4}\u{be}\
		\u{c0}\u{c4}\u{ca}\u{d3}\u{db}\u{e1}\u{e4}\u{e9}\x03\x02\x03\x02";
