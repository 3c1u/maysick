// Generated from lib/grammar/Maysick.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use super::maysicklistener::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{
    cast, cast_mut, BaseParserRuleContext, ParserRuleContext, ParserRuleContextType,
};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_source::TokenSource;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::{ParseTree, TerminalNode};
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;

use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
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
pub const RULE_prog: usize = 0;
pub const RULE_stmt: usize = 1;
pub const RULE_stmt_let: usize = 2;
pub const RULE_stmt_var: usize = 3;
pub const RULE_stmt_return: usize = 4;
pub const RULE_stmt_import: usize = 5;
pub const RULE_stmt_assign: usize = 6;
pub const RULE_stmt_fn_def: usize = 7;
pub const RULE_fn_call: usize = 8;
pub const RULE_expr: usize = 9;
pub const RULE_expr_ident: usize = 10;
pub const RULE_expr_unary: usize = 11;
pub const RULE_expr_mul: usize = 12;
pub const RULE_expr_add: usize = 13;
pub const RULE_expr_and: usize = 14;
pub const RULE_expr_or: usize = 15;
pub const RULE_block: usize = 16;
pub const RULE_if_expr: usize = 17;
pub const RULE_else_stmt: usize = 18;
pub const RULE_while_expr: usize = 19;
pub const ruleNames: [&'static str; 20] = [
    "prog",
    "stmt",
    "stmt_let",
    "stmt_var",
    "stmt_return",
    "stmt_import",
    "stmt_assign",
    "stmt_fn_def",
    "fn_call",
    "expr",
    "expr_ident",
    "expr_unary",
    "expr_mul",
    "expr_add",
    "expr_and",
    "expr_or",
    "block",
    "if_expr",
    "else_stmt",
    "while_expr",
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

type BaseParserType = BaseParser<MaysickParserExt, dyn MaysickListener>;

pub struct MaysickParser {
    base: BaseParserType,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy>,
}

impl MaysickParser {
    pub fn get_serialized_atn() -> &'static str {
        unimplemented!()
    }

    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy>) {
        self.err_handler = strategy
    }

    pub fn new(input: Box<dyn TokenStream>) -> Self {
        antlr_rust::recognizer::check_version("0", "1");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(input, Arc::clone(&interpreter), MaysickParserExt {}),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: Box::new(DefaultErrorStrategy::new()),
        }
    }
}

impl Deref for MaysickParser {
    type Target = BaseParserType;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for MaysickParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct MaysickParserExt {}

impl MaysickParserExt {}

impl ParserRecog for MaysickParserExt {}

impl Recognizer for MaysickParserExt {
    fn get_grammar_file_name(&self) -> &str {
        "Maysick.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
}

impl Actions for MaysickParserExt {
    type Recog = BaseParserType;
    fn sempred(
        _localctx: &dyn ParserRuleContext,
        rule_index: isize,
        pred_index: isize,
        recog: &mut <Self as Actions>::Recog,
    ) -> bool {
        match rule_index {
            12 => Self::expr_mul_sempred(cast::<_, Expr_mulContext>(_localctx), pred_index, recog),
            13 => Self::expr_add_sempred(cast::<_, Expr_addContext>(_localctx), pred_index, recog),
            14 => Self::expr_and_sempred(cast::<_, Expr_andContext>(_localctx), pred_index, recog),
            15 => Self::expr_or_sempred(cast::<_, Expr_orContext>(_localctx), pred_index, recog),
            _ => true,
        }
    }
}
impl MaysickParserExt {
    fn expr_mul_sempred(
        _localctx: &Expr_mulContext,
        pred_index: isize,
        recog: &mut <Self as Actions>::Recog,
    ) -> bool {
        match pred_index {
            0 => recog.precpred(None, 3),
            1 => recog.precpred(None, 2),
            2 => recog.precpred(None, 1),
            _ => true,
        }
    }

    fn expr_add_sempred(
        _localctx: &Expr_addContext,
        pred_index: isize,
        recog: &mut <Self as Actions>::Recog,
    ) -> bool {
        match pred_index {
            3 => recog.precpred(None, 2),
            4 => recog.precpred(None, 1),
            _ => true,
        }
    }

    fn expr_and_sempred(
        _localctx: &Expr_andContext,
        pred_index: isize,
        recog: &mut <Self as Actions>::Recog,
    ) -> bool {
        match pred_index {
            5 => recog.precpred(None, 1),
            _ => true,
        }
    }

    fn expr_or_sempred(
        _localctx: &Expr_orContext,
        pred_index: isize,
        recog: &mut <Self as Actions>::Recog,
    ) -> bool {
        match pred_index {
            6 => recog.precpred(None, 1),
            _ => true,
        }
    }
}
//------------------- prog ----------------
pub type ProgContextAll = ProgContext;

pub type ProgContext = BaseParserRuleContext<ProgContextExt>;

#[derive(Clone)]
pub struct ProgContextExt {}
impl CustomRuleContext for ProgContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_prog
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_prog(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_prog(ctx));
    }
}

impl ProgContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<ProgContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ProgContextExt {},
        ))
    }
}

pub trait ProgContextAttrs: ParserRuleContext + BorrowMut<ProgContextExt> {
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
    fn stmt_all(&self) -> Vec<Rc<StmtContextAll>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl ProgContextAttrs for ProgContext {}

//impl ProgContext{

//}

impl MaysickParser {
    pub fn prog(&mut self) -> Result<Rc<ProgContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ProgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_prog);
        let mut _localctx: Rc<ProgContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(43);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << IF)
                            | (1usize << WHILE)
                            | (1usize << FN)
                            | (1usize << LET)
                            | (1usize << VAR)
                            | (1usize << RETURN)
                            | (1usize << IMPORT)
                            | (1usize << SYM_LPAREN)
                            | (1usize << SYM_LCURLY)
                            | (1usize << SYM_SUB)
                            | (1usize << IDENT)
                            | (1usize << LIT_STRING)
                            | (1usize << LIT_NUMBER)))
                        != 0)
                {
                    {
                        {
                            /*InvokeRule stmt*/
                            recog.base.set_state(40);
                            recog.stmt()?;
                        }
                    }
                    recog.base.set_state(45);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(46);
                recog.base.match_token(EOF, recog.err_handler.as_mut())?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- stmt ----------------
pub type StmtContextAll = StmtContext;

pub type StmtContext = BaseParserRuleContext<StmtContextExt>;

#[derive(Clone)]
pub struct StmtContextExt {}
impl CustomRuleContext for StmtContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_stmt(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_stmt(ctx));
    }
}

impl StmtContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<StmtContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StmtContextExt {},
        ))
    }
}

pub trait StmtContextAttrs: ParserRuleContext + BorrowMut<StmtContextExt> {
    fn expr(&self) -> Option<Rc<ExprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_SEMICOLON
    /// Returns `None` if there is no child corresponding to token SYM_SEMICOLON
    fn SYM_SEMICOLON(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_SEMICOLON, 0)
    }
    fn stmt_var(&self) -> Option<Rc<Stmt_varContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn stmt_let(&self) -> Option<Rc<Stmt_letContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn stmt_return(&self) -> Option<Rc<Stmt_returnContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn stmt_import(&self) -> Option<Rc<Stmt_importContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn stmt_assign(&self) -> Option<Rc<Stmt_assignContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn stmt_fn_def(&self) -> Option<Rc<Stmt_fn_defContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn if_expr(&self) -> Option<Rc<If_exprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn while_expr(&self) -> Option<Rc<While_exprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl StmtContextAttrs for StmtContext {}

//impl StmtContext{

//}

impl MaysickParser {
    pub fn stmt(&mut self) -> Result<Rc<StmtContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_stmt);
        let mut _localctx: Rc<StmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(60);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(1, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule expr*/
                        recog.base.set_state(48);
                        recog.expr()?;

                        recog.base.set_state(49);
                        recog
                            .base
                            .match_token(SYM_SEMICOLON, recog.err_handler.as_mut())?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule stmt_var*/
                        recog.base.set_state(51);
                        recog.stmt_var()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule stmt_let*/
                        recog.base.set_state(52);
                        recog.stmt_let()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule stmt_return*/
                        recog.base.set_state(53);
                        recog.stmt_return()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule stmt_import*/
                        recog.base.set_state(54);
                        recog.stmt_import()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule stmt_assign*/
                        recog.base.set_state(55);
                        recog.stmt_assign()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        /*InvokeRule stmt_fn_def*/
                        recog.base.set_state(56);
                        recog.stmt_fn_def()?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        /*InvokeRule block*/
                        recog.base.set_state(57);
                        recog.block()?;
                    }
                }
                9 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 9);
                    recog.base.enter_outer_alt(None, 9);
                    {
                        /*InvokeRule if_expr*/
                        recog.base.set_state(58);
                        recog.if_expr()?;
                    }
                }
                10 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 10);
                    recog.base.enter_outer_alt(None, 10);
                    {
                        /*InvokeRule while_expr*/
                        recog.base.set_state(59);
                        recog.while_expr()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- stmt_let ----------------
pub type Stmt_letContextAll = Stmt_letContext;

pub type Stmt_letContext = BaseParserRuleContext<Stmt_letContextExt>;

#[derive(Clone)]
pub struct Stmt_letContextExt {}
impl CustomRuleContext for Stmt_letContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_let
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_stmt_let(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_stmt_let(ctx));
    }
}

impl Stmt_letContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Stmt_letContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Stmt_letContextExt {},
        ))
    }
}

pub trait Stmt_letContextAttrs: ParserRuleContext + BorrowMut<Stmt_letContextExt> {
    /// Retrieves first TerminalNode corresponding to token LET
    /// Returns `None` if there is no child corresponding to token LET
    fn LET(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(LET, 0)
    }
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(IDENT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_EQ
    /// Returns `None` if there is no child corresponding to token SYM_EQ
    fn SYM_EQ(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_EQ, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_SEMICOLON
    /// Returns `None` if there is no child corresponding to token SYM_SEMICOLON
    fn SYM_SEMICOLON(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_SEMICOLON, 0)
    }
}

impl Stmt_letContextAttrs for Stmt_letContext {}

//impl Stmt_letContext{

//}

impl MaysickParser {
    pub fn stmt_let(&mut self) -> Result<Rc<Stmt_letContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_letContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_stmt_let);
        let mut _localctx: Rc<Stmt_letContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(62);
                recog.base.match_token(LET, recog.err_handler.as_mut())?;

                recog.base.set_state(63);
                recog.base.match_token(IDENT, recog.err_handler.as_mut())?;

                recog.base.set_state(64);
                recog.base.match_token(SYM_EQ, recog.err_handler.as_mut())?;

                /*InvokeRule expr*/
                recog.base.set_state(65);
                recog.expr()?;

                recog.base.set_state(66);
                recog
                    .base
                    .match_token(SYM_SEMICOLON, recog.err_handler.as_mut())?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- stmt_var ----------------
pub type Stmt_varContextAll = Stmt_varContext;

pub type Stmt_varContext = BaseParserRuleContext<Stmt_varContextExt>;

#[derive(Clone)]
pub struct Stmt_varContextExt {}
impl CustomRuleContext for Stmt_varContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_var
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_stmt_var(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_stmt_var(ctx));
    }
}

impl Stmt_varContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Stmt_varContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Stmt_varContextExt {},
        ))
    }
}

pub trait Stmt_varContextAttrs: ParserRuleContext + BorrowMut<Stmt_varContextExt> {
    /// Retrieves first TerminalNode corresponding to token VAR
    /// Returns `None` if there is no child corresponding to token VAR
    fn VAR(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(VAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(IDENT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_EQ
    /// Returns `None` if there is no child corresponding to token SYM_EQ
    fn SYM_EQ(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_EQ, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_SEMICOLON
    /// Returns `None` if there is no child corresponding to token SYM_SEMICOLON
    fn SYM_SEMICOLON(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_SEMICOLON, 0)
    }
}

impl Stmt_varContextAttrs for Stmt_varContext {}

//impl Stmt_varContext{

//}

impl MaysickParser {
    pub fn stmt_var(&mut self) -> Result<Rc<Stmt_varContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_varContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_stmt_var);
        let mut _localctx: Rc<Stmt_varContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(68);
                recog.base.match_token(VAR, recog.err_handler.as_mut())?;

                recog.base.set_state(69);
                recog.base.match_token(IDENT, recog.err_handler.as_mut())?;

                recog.base.set_state(70);
                recog.base.match_token(SYM_EQ, recog.err_handler.as_mut())?;

                /*InvokeRule expr*/
                recog.base.set_state(71);
                recog.expr()?;

                recog.base.set_state(72);
                recog
                    .base
                    .match_token(SYM_SEMICOLON, recog.err_handler.as_mut())?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- stmt_return ----------------
pub type Stmt_returnContextAll = Stmt_returnContext;

pub type Stmt_returnContext = BaseParserRuleContext<Stmt_returnContextExt>;

#[derive(Clone)]
pub struct Stmt_returnContextExt {}
impl CustomRuleContext for Stmt_returnContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_return
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_stmt_return(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_stmt_return(ctx));
    }
}

impl Stmt_returnContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Stmt_returnContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Stmt_returnContextExt {},
        ))
    }
}

pub trait Stmt_returnContextAttrs: ParserRuleContext + BorrowMut<Stmt_returnContextExt> {
    /// Retrieves first TerminalNode corresponding to token RETURN
    /// Returns `None` if there is no child corresponding to token RETURN
    fn RETURN(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(RETURN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_SEMICOLON
    /// Returns `None` if there is no child corresponding to token SYM_SEMICOLON
    fn SYM_SEMICOLON(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_SEMICOLON, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl Stmt_returnContextAttrs for Stmt_returnContext {}

//impl Stmt_returnContext{

//}

impl MaysickParser {
    pub fn stmt_return(&mut self) -> Result<Rc<Stmt_returnContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_returnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 8, RULE_stmt_return);
        let mut _localctx: Rc<Stmt_returnContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(74);
                recog.base.match_token(RETURN, recog.err_handler.as_mut())?;

                recog.base.set_state(76);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << IF)
                            | (1usize << WHILE)
                            | (1usize << SYM_LPAREN)
                            | (1usize << SYM_SUB)
                            | (1usize << IDENT)
                            | (1usize << LIT_STRING)
                            | (1usize << LIT_NUMBER)))
                        != 0)
                {
                    {
                        /*InvokeRule expr*/
                        recog.base.set_state(75);
                        recog.expr()?;
                    }
                }

                recog.base.set_state(78);
                recog
                    .base
                    .match_token(SYM_SEMICOLON, recog.err_handler.as_mut())?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- stmt_import ----------------
pub type Stmt_importContextAll = Stmt_importContext;

pub type Stmt_importContext = BaseParserRuleContext<Stmt_importContextExt>;

#[derive(Clone)]
pub struct Stmt_importContextExt {}
impl CustomRuleContext for Stmt_importContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_import
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_stmt_import(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_stmt_import(ctx));
    }
}

impl Stmt_importContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Stmt_importContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Stmt_importContextExt {},
        ))
    }
}

pub trait Stmt_importContextAttrs: ParserRuleContext + BorrowMut<Stmt_importContextExt> {
    /// Retrieves first TerminalNode corresponding to token IMPORT
    /// Returns `None` if there is no child corresponding to token IMPORT
    fn IMPORT(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(IMPORT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LIT_STRING
    /// Returns `None` if there is no child corresponding to token LIT_STRING
    fn LIT_STRING(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(LIT_STRING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_SEMICOLON
    /// Returns `None` if there is no child corresponding to token SYM_SEMICOLON
    fn SYM_SEMICOLON(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_SEMICOLON, 0)
    }
}

impl Stmt_importContextAttrs for Stmt_importContext {}

//impl Stmt_importContext{

//}

impl MaysickParser {
    pub fn stmt_import(&mut self) -> Result<Rc<Stmt_importContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_importContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 10, RULE_stmt_import);
        let mut _localctx: Rc<Stmt_importContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(80);
                recog.base.match_token(IMPORT, recog.err_handler.as_mut())?;

                recog.base.set_state(81);
                recog
                    .base
                    .match_token(LIT_STRING, recog.err_handler.as_mut())?;

                recog.base.set_state(82);
                recog
                    .base
                    .match_token(SYM_SEMICOLON, recog.err_handler.as_mut())?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- stmt_assign ----------------
pub type Stmt_assignContextAll = Stmt_assignContext;

pub type Stmt_assignContext = BaseParserRuleContext<Stmt_assignContextExt>;

#[derive(Clone)]
pub struct Stmt_assignContextExt {}
impl CustomRuleContext for Stmt_assignContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_assign
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_stmt_assign(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_stmt_assign(ctx));
    }
}

impl Stmt_assignContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Stmt_assignContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Stmt_assignContextExt {},
        ))
    }
}

pub trait Stmt_assignContextAttrs: ParserRuleContext + BorrowMut<Stmt_assignContextExt> {
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(IDENT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_EQ
    /// Returns `None` if there is no child corresponding to token SYM_EQ
    fn SYM_EQ(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_EQ, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_SEMICOLON
    /// Returns `None` if there is no child corresponding to token SYM_SEMICOLON
    fn SYM_SEMICOLON(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_SEMICOLON, 0)
    }
}

impl Stmt_assignContextAttrs for Stmt_assignContext {}

//impl Stmt_assignContext{

//}

impl MaysickParser {
    pub fn stmt_assign(&mut self) -> Result<Rc<Stmt_assignContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_assignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 12, RULE_stmt_assign);
        let mut _localctx: Rc<Stmt_assignContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(84);
                recog.base.match_token(IDENT, recog.err_handler.as_mut())?;

                recog.base.set_state(85);
                recog.base.match_token(SYM_EQ, recog.err_handler.as_mut())?;

                /*InvokeRule expr*/
                recog.base.set_state(86);
                recog.expr()?;

                recog.base.set_state(87);
                recog
                    .base
                    .match_token(SYM_SEMICOLON, recog.err_handler.as_mut())?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- stmt_fn_def ----------------
pub type Stmt_fn_defContextAll = Stmt_fn_defContext;

pub type Stmt_fn_defContext = BaseParserRuleContext<Stmt_fn_defContextExt>;

#[derive(Clone)]
pub struct Stmt_fn_defContextExt {}
impl CustomRuleContext for Stmt_fn_defContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_fn_def
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_stmt_fn_def(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_stmt_fn_def(ctx));
    }
}

impl Stmt_fn_defContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Stmt_fn_defContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Stmt_fn_defContextExt {},
        ))
    }
}

pub trait Stmt_fn_defContextAttrs: ParserRuleContext + BorrowMut<Stmt_fn_defContextExt> {
    /// Retrieves first TerminalNode corresponding to token FN
    /// Returns `None` if there is no child corresponding to token FN
    fn FN(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(FN, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token IDENT in current rule
    fn IDENT_all(&self) -> Vec<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token IDENT, starting from 0.
    /// Returns `None` if number of children corresponding to token IDENT is less or equal than `i`.
    fn IDENT(&self, i: usize) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(IDENT, i)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_LPAREN
    /// Returns `None` if there is no child corresponding to token SYM_LPAREN
    fn SYM_LPAREN(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_LPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_RPAREN
    /// Returns `None` if there is no child corresponding to token SYM_RPAREN
    fn SYM_RPAREN(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_RPAREN, 0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl Stmt_fn_defContextAttrs for Stmt_fn_defContext {}

//impl Stmt_fn_defContext{

//}

impl MaysickParser {
    pub fn stmt_fn_def(&mut self) -> Result<Rc<Stmt_fn_defContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_fn_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 14, RULE_stmt_fn_def);
        let mut _localctx: Rc<Stmt_fn_defContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(89);
                recog.base.match_token(FN, recog.err_handler.as_mut())?;

                recog.base.set_state(90);
                recog.base.match_token(IDENT, recog.err_handler.as_mut())?;

                recog.base.set_state(91);
                recog
                    .base
                    .match_token(SYM_LPAREN, recog.err_handler.as_mut())?;

                recog.base.set_state(95);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == IDENT {
                    {
                        {
                            recog.base.set_state(92);
                            recog.base.match_token(IDENT, recog.err_handler.as_mut())?;
                        }
                    }
                    recog.base.set_state(97);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(98);
                recog
                    .base
                    .match_token(SYM_RPAREN, recog.err_handler.as_mut())?;

                /*InvokeRule block*/
                recog.base.set_state(99);
                recog.block()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- fn_call ----------------
pub type Fn_callContextAll = Fn_callContext;

pub type Fn_callContext = BaseParserRuleContext<Fn_callContextExt>;

#[derive(Clone)]
pub struct Fn_callContextExt {}
impl CustomRuleContext for Fn_callContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_fn_call
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_fn_call(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_fn_call(ctx));
    }
}

impl Fn_callContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Fn_callContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Fn_callContextExt {},
        ))
    }
}

pub trait Fn_callContextAttrs: ParserRuleContext + BorrowMut<Fn_callContextExt> {
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(IDENT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_LPAREN
    /// Returns `None` if there is no child corresponding to token SYM_LPAREN
    fn SYM_LPAREN(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_LPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_RPAREN
    /// Returns `None` if there is no child corresponding to token SYM_RPAREN
    fn SYM_RPAREN(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_RPAREN, 0)
    }
    fn expr_all(&self) -> Vec<Rc<ExprContextAll>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<Rc<ExprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl Fn_callContextAttrs for Fn_callContext {}

//impl Fn_callContext{

//}

impl MaysickParser {
    pub fn fn_call(&mut self) -> Result<Rc<Fn_callContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Fn_callContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_fn_call);
        let mut _localctx: Rc<Fn_callContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(101);
                recog.base.match_token(IDENT, recog.err_handler.as_mut())?;

                recog.base.set_state(102);
                recog
                    .base
                    .match_token(SYM_LPAREN, recog.err_handler.as_mut())?;

                recog.base.set_state(106);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << IF)
                            | (1usize << WHILE)
                            | (1usize << SYM_LPAREN)
                            | (1usize << SYM_SUB)
                            | (1usize << IDENT)
                            | (1usize << LIT_STRING)
                            | (1usize << LIT_NUMBER)))
                        != 0)
                {
                    {
                        {
                            /*InvokeRule expr*/
                            recog.base.set_state(103);
                            recog.expr()?;
                        }
                    }
                    recog.base.set_state(108);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(109);
                recog
                    .base
                    .match_token(SYM_RPAREN, recog.err_handler.as_mut())?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- expr ----------------
pub type ExprContextAll = ExprContext;

pub type ExprContext = BaseParserRuleContext<ExprContextExt>;

#[derive(Clone)]
pub struct ExprContextExt {}
impl CustomRuleContext for ExprContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_expr(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_expr(ctx));
    }
}

impl ExprContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<ExprContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            ExprContextExt {},
        ))
    }
}

pub trait ExprContextAttrs: ParserRuleContext + BorrowMut<ExprContextExt> {
    fn expr_or(&self) -> Option<Rc<Expr_orContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprContextAttrs for ExprContext {}

//impl ExprContext{

//}

impl MaysickParser {
    pub fn expr(&mut self) -> Result<Rc<ExprContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_expr);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule expr_or*/
                recog.base.set_state(111);
                recog.expr_or_rec(0)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- expr_ident ----------------
pub type Expr_identContextAll = Expr_identContext;

pub type Expr_identContext = BaseParserRuleContext<Expr_identContextExt>;

#[derive(Clone)]
pub struct Expr_identContextExt {}
impl CustomRuleContext for Expr_identContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_ident
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_expr_ident(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_expr_ident(ctx));
    }
}

impl Expr_identContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Expr_identContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Expr_identContextExt {},
        ))
    }
}

pub trait Expr_identContextAttrs: ParserRuleContext + BorrowMut<Expr_identContextExt> {
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(IDENT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LIT_STRING
    /// Returns `None` if there is no child corresponding to token LIT_STRING
    fn LIT_STRING(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(LIT_STRING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LIT_NUMBER
    /// Returns `None` if there is no child corresponding to token LIT_NUMBER
    fn LIT_NUMBER(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(LIT_NUMBER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_LPAREN
    /// Returns `None` if there is no child corresponding to token SYM_LPAREN
    fn SYM_LPAREN(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_LPAREN, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_RPAREN
    /// Returns `None` if there is no child corresponding to token SYM_RPAREN
    fn SYM_RPAREN(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_RPAREN, 0)
    }
    fn if_expr(&self) -> Option<Rc<If_exprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn while_expr(&self) -> Option<Rc<While_exprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn fn_call(&self) -> Option<Rc<Fn_callContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl Expr_identContextAttrs for Expr_identContext {}

//impl Expr_identContext{

//}

impl MaysickParser {
    pub fn expr_ident(&mut self) -> Result<Rc<Expr_identContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Expr_identContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 20, RULE_expr_ident);
        let mut _localctx: Rc<Expr_identContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(123);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(5, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(113);
                        recog.base.match_token(IDENT, recog.err_handler.as_mut())?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(114);
                        recog
                            .base
                            .match_token(LIT_STRING, recog.err_handler.as_mut())?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        recog.base.set_state(115);
                        recog
                            .base
                            .match_token(LIT_NUMBER, recog.err_handler.as_mut())?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(116);
                        recog
                            .base
                            .match_token(SYM_LPAREN, recog.err_handler.as_mut())?;

                        /*InvokeRule expr*/
                        recog.base.set_state(117);
                        recog.expr()?;

                        recog.base.set_state(118);
                        recog
                            .base
                            .match_token(SYM_RPAREN, recog.err_handler.as_mut())?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule if_expr*/
                        recog.base.set_state(120);
                        recog.if_expr()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule while_expr*/
                        recog.base.set_state(121);
                        recog.while_expr()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        /*InvokeRule fn_call*/
                        recog.base.set_state(122);
                        recog.fn_call()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- expr_unary ----------------
pub type Expr_unaryContextAll = Expr_unaryContext;

pub type Expr_unaryContext = BaseParserRuleContext<Expr_unaryContextExt>;

#[derive(Clone)]
pub struct Expr_unaryContextExt {}
impl CustomRuleContext for Expr_unaryContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_unary
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_expr_unary(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_expr_unary(ctx));
    }
}

impl Expr_unaryContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Expr_unaryContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Expr_unaryContextExt {},
        ))
    }
}

pub trait Expr_unaryContextAttrs: ParserRuleContext + BorrowMut<Expr_unaryContextExt> {
    /// Retrieves first TerminalNode corresponding to token SYM_SUB
    /// Returns `None` if there is no child corresponding to token SYM_SUB
    fn SYM_SUB(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_SUB, 0)
    }
    fn expr_ident(&self) -> Option<Rc<Expr_identContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl Expr_unaryContextAttrs for Expr_unaryContext {}

//impl Expr_unaryContext{

//}

impl MaysickParser {
    pub fn expr_unary(&mut self) -> Result<Rc<Expr_unaryContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Expr_unaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 22, RULE_expr_unary);
        let mut _localctx: Rc<Expr_unaryContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(128);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SYM_SUB => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(125);
                        recog
                            .base
                            .match_token(SYM_SUB, recog.err_handler.as_mut())?;

                        /*InvokeRule expr_ident*/
                        recog.base.set_state(126);
                        recog.expr_ident()?;
                    }
                }

                IF | WHILE | SYM_LPAREN | IDENT | LIT_STRING | LIT_NUMBER => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule expr_ident*/
                        recog.base.set_state(127);
                        recog.expr_ident()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- expr_mul ----------------
pub type Expr_mulContextAll = Expr_mulContext;

pub type Expr_mulContext = BaseParserRuleContext<Expr_mulContextExt>;

#[derive(Clone)]
pub struct Expr_mulContextExt {}
impl CustomRuleContext for Expr_mulContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_mul
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_expr_mul(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_expr_mul(ctx));
    }
}

impl Expr_mulContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Expr_mulContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Expr_mulContextExt {},
        ))
    }
}

pub trait Expr_mulContextAttrs: ParserRuleContext + BorrowMut<Expr_mulContextExt> {
    fn expr_unary(&self) -> Option<Rc<Expr_unaryContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expr_mul(&self) -> Option<Rc<Expr_mulContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_MUL
    /// Returns `None` if there is no child corresponding to token SYM_MUL
    fn SYM_MUL(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_MUL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_DIV
    /// Returns `None` if there is no child corresponding to token SYM_DIV
    fn SYM_DIV(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_DIV, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_MOD
    /// Returns `None` if there is no child corresponding to token SYM_MOD
    fn SYM_MOD(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_MOD, 0)
    }
}

impl Expr_mulContextAttrs for Expr_mulContext {}

//impl Expr_mulContext{

//}

impl MaysickParser {
    pub fn expr_mul(&mut self) -> Result<Rc<Expr_mulContextAll>, ANTLRError> {
        self.expr_mul_rec(0)
    }

    fn expr_mul_rec(&mut self, _p: isize) -> Result<Rc<Expr_mulContextAll>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = Expr_mulContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 24, RULE_expr_mul, _p);
        let mut _localctx: Rc<Expr_mulContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 24;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    /*InvokeRule expr_unary*/
                    recog.base.set_state(131);
                    recog.expr_unary()?;
                }
                let tmp = recog.input.lt(-1).map(Token::to_owned);
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(144);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(8, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(142);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(7, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp = Expr_mulContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr_mul,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(133);
                                        if !({ recog.precpred(None, 3) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 3)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(134);
                                        recog
                                            .base
                                            .match_token(SYM_MUL, recog.err_handler.as_mut())?;

                                        /*InvokeRule expr_unary*/
                                        recog.base.set_state(135);
                                        recog.expr_unary()?;
                                    }
                                }
                                2 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp = Expr_mulContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr_mul,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(136);
                                        if !({ recog.precpred(None, 2) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 2)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(137);
                                        recog
                                            .base
                                            .match_token(SYM_DIV, recog.err_handler.as_mut())?;

                                        /*InvokeRule expr_unary*/
                                        recog.base.set_state(138);
                                        recog.expr_unary()?;
                                    }
                                }
                                3 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp = Expr_mulContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr_mul,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(139);
                                        if !({ recog.precpred(None, 1) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 1)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(140);
                                        recog
                                            .base
                                            .match_token(SYM_MOD, recog.err_handler.as_mut())?;

                                        /*InvokeRule expr_unary*/
                                        recog.base.set_state(141);
                                        recog.expr_unary()?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(146);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(8, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- expr_add ----------------
pub type Expr_addContextAll = Expr_addContext;

pub type Expr_addContext = BaseParserRuleContext<Expr_addContextExt>;

#[derive(Clone)]
pub struct Expr_addContextExt {}
impl CustomRuleContext for Expr_addContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_add
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_expr_add(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_expr_add(ctx));
    }
}

impl Expr_addContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Expr_addContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Expr_addContextExt {},
        ))
    }
}

pub trait Expr_addContextAttrs: ParserRuleContext + BorrowMut<Expr_addContextExt> {
    fn expr_mul(&self) -> Option<Rc<Expr_mulContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expr_add(&self) -> Option<Rc<Expr_addContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_ADD
    /// Returns `None` if there is no child corresponding to token SYM_ADD
    fn SYM_ADD(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_ADD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_SUB
    /// Returns `None` if there is no child corresponding to token SYM_SUB
    fn SYM_SUB(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_SUB, 0)
    }
}

impl Expr_addContextAttrs for Expr_addContext {}

//impl Expr_addContext{

//}

impl MaysickParser {
    pub fn expr_add(&mut self) -> Result<Rc<Expr_addContextAll>, ANTLRError> {
        self.expr_add_rec(0)
    }

    fn expr_add_rec(&mut self, _p: isize) -> Result<Rc<Expr_addContextAll>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = Expr_addContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 26, RULE_expr_add, _p);
        let mut _localctx: Rc<Expr_addContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 26;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    /*InvokeRule expr_mul*/
                    recog.base.set_state(148);
                    recog.expr_mul_rec(0)?;
                }
                let tmp = recog.input.lt(-1).map(Token::to_owned);
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(158);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(10, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(156);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(9, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp = Expr_addContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr_add,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(150);
                                        if !({ recog.precpred(None, 2) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 2)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(151);
                                        recog
                                            .base
                                            .match_token(SYM_ADD, recog.err_handler.as_mut())?;

                                        /*InvokeRule expr_mul*/
                                        recog.base.set_state(152);
                                        recog.expr_mul_rec(0)?;
                                    }
                                }
                                2 => {
                                    {
                                        /*recRuleAltStartAction*/
                                        let mut tmp = Expr_addContextExt::new(
                                            _parentctx.clone(),
                                            _parentState,
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr_add,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(153);
                                        if !({ recog.precpred(None, 1) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 1)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(154);
                                        recog
                                            .base
                                            .match_token(SYM_SUB, recog.err_handler.as_mut())?;

                                        /*InvokeRule expr_mul*/
                                        recog.base.set_state(155);
                                        recog.expr_mul_rec(0)?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(160);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(10, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- expr_and ----------------
pub type Expr_andContextAll = Expr_andContext;

pub type Expr_andContext = BaseParserRuleContext<Expr_andContextExt>;

#[derive(Clone)]
pub struct Expr_andContextExt {}
impl CustomRuleContext for Expr_andContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_and
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_expr_and(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_expr_and(ctx));
    }
}

impl Expr_andContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Expr_andContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Expr_andContextExt {},
        ))
    }
}

pub trait Expr_andContextAttrs: ParserRuleContext + BorrowMut<Expr_andContextExt> {
    fn expr_add(&self) -> Option<Rc<Expr_addContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expr_and(&self) -> Option<Rc<Expr_andContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_AND
    /// Returns `None` if there is no child corresponding to token SYM_AND
    fn SYM_AND(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_AND, 0)
    }
}

impl Expr_andContextAttrs for Expr_andContext {}

//impl Expr_andContext{

//}

impl MaysickParser {
    pub fn expr_and(&mut self) -> Result<Rc<Expr_andContextAll>, ANTLRError> {
        self.expr_and_rec(0)
    }

    fn expr_and_rec(&mut self, _p: isize) -> Result<Rc<Expr_andContextAll>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = Expr_andContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 28, RULE_expr_and, _p);
        let mut _localctx: Rc<Expr_andContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 28;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    /*InvokeRule expr_add*/
                    recog.base.set_state(162);
                    recog.expr_add_rec(0)?;
                }
                let tmp = recog.input.lt(-1).map(Token::to_owned);
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(169);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(11, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            {
                                /*recRuleAltStartAction*/
                                let mut tmp =
                                    Expr_andContextExt::new(_parentctx.clone(), _parentState);
                                recog.push_new_recursion_context(
                                    tmp.clone(),
                                    _startState,
                                    RULE_expr_and,
                                );
                                _localctx = tmp;
                                recog.base.set_state(164);
                                if !({ recog.precpred(None, 1) }) {
                                    Err(FailedPredicateError::new(
                                        &mut recog.base,
                                        Some("recog.precpred(None, 1)".to_owned()),
                                        None,
                                    ))?;
                                }
                                recog.base.set_state(165);
                                recog
                                    .base
                                    .match_token(SYM_AND, recog.err_handler.as_mut())?;

                                /*InvokeRule expr_add*/
                                recog.base.set_state(166);
                                recog.expr_add_rec(0)?;
                            }
                        }
                    }
                    recog.base.set_state(171);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(11, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- expr_or ----------------
pub type Expr_orContextAll = Expr_orContext;

pub type Expr_orContext = BaseParserRuleContext<Expr_orContextExt>;

#[derive(Clone)]
pub struct Expr_orContextExt {}
impl CustomRuleContext for Expr_orContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_or
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_expr_or(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_expr_or(ctx));
    }
}

impl Expr_orContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Expr_orContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Expr_orContextExt {},
        ))
    }
}

pub trait Expr_orContextAttrs: ParserRuleContext + BorrowMut<Expr_orContextExt> {
    fn expr_and(&self) -> Option<Rc<Expr_andContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expr_or(&self) -> Option<Rc<Expr_orContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_OR
    /// Returns `None` if there is no child corresponding to token SYM_OR
    fn SYM_OR(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_OR, 0)
    }
}

impl Expr_orContextAttrs for Expr_orContext {}

//impl Expr_orContext{

//}

impl MaysickParser {
    pub fn expr_or(&mut self) -> Result<Rc<Expr_orContextAll>, ANTLRError> {
        self.expr_or_rec(0)
    }

    fn expr_or_rec(&mut self, _p: isize) -> Result<Rc<Expr_orContextAll>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = Expr_orContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 30, RULE_expr_or, _p);
        let mut _localctx: Rc<Expr_orContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 30;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    /*InvokeRule expr_and*/
                    recog.base.set_state(173);
                    recog.expr_and_rec(0)?;
                }
                let tmp = recog.input.lt(-1).map(Token::to_owned);
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(180);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(12, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            {
                                /*recRuleAltStartAction*/
                                let mut tmp =
                                    Expr_orContextExt::new(_parentctx.clone(), _parentState);
                                recog.push_new_recursion_context(
                                    tmp.clone(),
                                    _startState,
                                    RULE_expr_or,
                                );
                                _localctx = tmp;
                                recog.base.set_state(175);
                                if !({ recog.precpred(None, 1) }) {
                                    Err(FailedPredicateError::new(
                                        &mut recog.base,
                                        Some("recog.precpred(None, 1)".to_owned()),
                                        None,
                                    ))?;
                                }
                                recog.base.set_state(176);
                                recog.base.match_token(SYM_OR, recog.err_handler.as_mut())?;

                                /*InvokeRule expr_and*/
                                recog.base.set_state(177);
                                recog.expr_and_rec(0)?;
                            }
                        }
                    }
                    recog.base.set_state(182);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(12, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- block ----------------
pub type BlockContextAll = BlockContext;

pub type BlockContext = BaseParserRuleContext<BlockContextExt>;

#[derive(Clone)]
pub struct BlockContextExt {}
impl CustomRuleContext for BlockContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_block
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_block(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_block(ctx));
    }
}

impl BlockContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<BlockContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            BlockContextExt {},
        ))
    }
}

pub trait BlockContextAttrs: ParserRuleContext + BorrowMut<BlockContextExt> {
    /// Retrieves first TerminalNode corresponding to token SYM_LCURLY
    /// Returns `None` if there is no child corresponding to token SYM_LCURLY
    fn SYM_LCURLY(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_LCURLY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_RCURLY
    /// Returns `None` if there is no child corresponding to token SYM_RCURLY
    fn SYM_RCURLY(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_RCURLY, 0)
    }
    fn stmt_all(&self) -> Vec<Rc<StmtContextAll>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl BlockContextAttrs for BlockContext {}

//impl BlockContext{

//}

impl MaysickParser {
    pub fn block(&mut self) -> Result<Rc<BlockContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = BlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            recog.base.set_state(201);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(15, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        recog.base.set_state(183);
                        recog
                            .base
                            .match_token(SYM_LCURLY, recog.err_handler.as_mut())?;

                        recog.base.set_state(187);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << IF)
                                    | (1usize << WHILE)
                                    | (1usize << FN)
                                    | (1usize << LET)
                                    | (1usize << VAR)
                                    | (1usize << RETURN)
                                    | (1usize << IMPORT)
                                    | (1usize << SYM_LPAREN)
                                    | (1usize << SYM_LCURLY)
                                    | (1usize << SYM_SUB)
                                    | (1usize << IDENT)
                                    | (1usize << LIT_STRING)
                                    | (1usize << LIT_NUMBER)))
                                != 0)
                        {
                            {
                                {
                                    /*InvokeRule stmt*/
                                    recog.base.set_state(184);
                                    recog.stmt()?;
                                }
                            }
                            recog.base.set_state(189);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(190);
                        recog
                            .base
                            .match_token(SYM_RCURLY, recog.err_handler.as_mut())?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(191);
                        recog
                            .base
                            .match_token(SYM_LCURLY, recog.err_handler.as_mut())?;

                        recog.base.set_state(195);
                        recog.err_handler.sync(&mut recog.base)?;
                        _alt = recog.interpreter.adaptive_predict(14, &mut recog.base)?;
                        while { _alt != 2 && _alt != INVALID_ALT } {
                            if _alt == 1 {
                                {
                                    {
                                        /*InvokeRule stmt*/
                                        recog.base.set_state(192);
                                        recog.stmt()?;
                                    }
                                }
                            }
                            recog.base.set_state(197);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(14, &mut recog.base)?;
                        }
                        /*InvokeRule expr*/
                        recog.base.set_state(198);
                        recog.expr()?;

                        recog.base.set_state(199);
                        recog
                            .base
                            .match_token(SYM_RCURLY, recog.err_handler.as_mut())?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- if_expr ----------------
pub type If_exprContextAll = If_exprContext;

pub type If_exprContext = BaseParserRuleContext<If_exprContextExt>;

#[derive(Clone)]
pub struct If_exprContextExt {}
impl CustomRuleContext for If_exprContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_if_expr
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_if_expr(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_if_expr(ctx));
    }
}

impl If_exprContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<If_exprContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            If_exprContextExt {},
        ))
    }
}

pub trait If_exprContextAttrs: ParserRuleContext + BorrowMut<If_exprContextExt> {
    /// Retrieves first TerminalNode corresponding to token IF
    /// Returns `None` if there is no child corresponding to token IF
    fn IF(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(IF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_LPAREN
    /// Returns `None` if there is no child corresponding to token SYM_LPAREN
    fn SYM_LPAREN(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_LPAREN, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_RPAREN
    /// Returns `None` if there is no child corresponding to token SYM_RPAREN
    fn SYM_RPAREN(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_RPAREN, 0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn else_stmt(&self) -> Option<Rc<Else_stmtContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl If_exprContextAttrs for If_exprContext {}

//impl If_exprContext{

//}

impl MaysickParser {
    pub fn if_expr(&mut self) -> Result<Rc<If_exprContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = If_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_if_expr);
        let mut _localctx: Rc<If_exprContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(203);
                recog.base.match_token(IF, recog.err_handler.as_mut())?;

                recog.base.set_state(204);
                recog
                    .base
                    .match_token(SYM_LPAREN, recog.err_handler.as_mut())?;

                /*InvokeRule expr*/
                recog.base.set_state(205);
                recog.expr()?;

                recog.base.set_state(206);
                recog
                    .base
                    .match_token(SYM_RPAREN, recog.err_handler.as_mut())?;

                /*InvokeRule block*/
                recog.base.set_state(207);
                recog.block()?;

                recog.base.set_state(209);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(16, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule else_stmt*/
                            recog.base.set_state(208);
                            recog.else_stmt()?;
                        }
                    }

                    _ => {}
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- else_stmt ----------------
pub type Else_stmtContextAll = Else_stmtContext;

pub type Else_stmtContext = BaseParserRuleContext<Else_stmtContextExt>;

#[derive(Clone)]
pub struct Else_stmtContextExt {}
impl CustomRuleContext for Else_stmtContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_else_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_else_stmt(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_else_stmt(ctx));
    }
}

impl Else_stmtContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Else_stmtContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Else_stmtContextExt {},
        ))
    }
}

pub trait Else_stmtContextAttrs: ParserRuleContext + BorrowMut<Else_stmtContextExt> {
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(ELSE, 0)
    }
    fn stmt(&self) -> Option<Rc<StmtContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl Else_stmtContextAttrs for Else_stmtContext {}

//impl Else_stmtContext{

//}

impl MaysickParser {
    pub fn else_stmt(&mut self) -> Result<Rc<Else_stmtContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Else_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_else_stmt);
        let mut _localctx: Rc<Else_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(211);
                recog.base.match_token(ELSE, recog.err_handler.as_mut())?;

                /*InvokeRule stmt*/
                recog.base.set_state(212);
                recog.stmt()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- while_expr ----------------
pub type While_exprContextAll = While_exprContext;

pub type While_exprContext = BaseParserRuleContext<While_exprContextExt>;

#[derive(Clone)]
pub struct While_exprContextExt {}
impl CustomRuleContext for While_exprContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_while_expr
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_while_expr(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_while_expr(ctx));
    }
}

impl While_exprContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<While_exprContextAll> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            While_exprContextExt {},
        ))
    }
}

pub trait While_exprContextAttrs: ParserRuleContext + BorrowMut<While_exprContextExt> {
    /// Retrieves first TerminalNode corresponding to token WHILE
    /// Returns `None` if there is no child corresponding to token WHILE
    fn WHILE(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(WHILE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_LPAREN
    /// Returns `None` if there is no child corresponding to token SYM_LPAREN
    fn SYM_LPAREN(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_LPAREN, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_RPAREN
    /// Returns `None` if there is no child corresponding to token SYM_RPAREN
    fn SYM_RPAREN(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_RPAREN, 0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl While_exprContextAttrs for While_exprContext {}

//impl While_exprContext{

//}

impl MaysickParser {
    pub fn while_expr(&mut self) -> Result<Rc<While_exprContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = While_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 38, RULE_while_expr);
        let mut _localctx: Rc<While_exprContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(214);
                recog.base.match_token(WHILE, recog.err_handler.as_mut())?;

                recog.base.set_state(215);
                recog
                    .base
                    .match_token(SYM_LPAREN, recog.err_handler.as_mut())?;

                /*InvokeRule expr*/
                recog.base.set_state(216);
                recog.expr()?;

                recog.base.set_state(217);
                recog
                    .base
                    .match_token(SYM_RPAREN, recog.err_handler.as_mut())?;

                /*InvokeRule block*/
                recog.base.set_state(218);
                recog.block()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
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
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x1f\u{df}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\x13\
	\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x03\x02\x07\x02\x2c\x0a\x02\x0c\
	\x02\x0e\x02\x2f\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x05\x03\
	\x3f\x0a\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\x05\x06\x4f\x0a\x06\
	\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\x08\
	\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x09\x07\x09\x60\x0a\x09\x0c\
	\x09\x0e\x09\x63\x0b\x09\x03\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\
	\x07\x0a\x6b\x0a\x0a\x0c\x0a\x0e\x0a\x6e\x0b\x0a\x03\x0a\x03\x0a\x03\x0b\
	\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x05\x0c\x7e\x0a\x0c\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{83}\
	\x0a\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x07\x0e\u{91}\x0a\x0e\x0c\x0e\x0e\x0e\u{94}\
	\x0b\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\
	\x03\x0f\x07\x0f\u{9f}\x0a\x0f\x0c\x0f\x0e\x0f\u{a2}\x0b\x0f\x03\x10\x03\
	\x10\x03\x10\x03\x10\x03\x10\x03\x10\x07\x10\u{aa}\x0a\x10\x0c\x10\x0e\x10\
	\u{ad}\x0b\x10\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x07\x11\u{b5}\
	\x0a\x11\x0c\x11\x0e\x11\u{b8}\x0b\x11\x03\x12\x03\x12\x07\x12\u{bc}\x0a\
	\x12\x0c\x12\x0e\x12\u{bf}\x0b\x12\x03\x12\x03\x12\x03\x12\x07\x12\u{c4}\
	\x0a\x12\x0c\x12\x0e\x12\u{c7}\x0b\x12\x03\x12\x03\x12\x03\x12\x05\x12\u{cc}\
	\x0a\x12\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\x05\x13\u{d4}\x0a\
	\x13\x03\x14\x03\x14\x03\x14\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x03\
	\x15\x03\x15\x02\x06\x1a\x1c\x1e\x20\x16\x02\x04\x06\x08\x0a\x0c\x0e\x10\
	\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x02\x02\x02\u{e9}\x02\x2d\
	\x03\x02\x02\x02\x04\x3e\x03\x02\x02\x02\x06\x40\x03\x02\x02\x02\x08\x46\
	\x03\x02\x02\x02\x0a\x4c\x03\x02\x02\x02\x0c\x52\x03\x02\x02\x02\x0e\x56\
	\x03\x02\x02\x02\x10\x5b\x03\x02\x02\x02\x12\x67\x03\x02\x02\x02\x14\x71\
	\x03\x02\x02\x02\x16\x7d\x03\x02\x02\x02\x18\u{82}\x03\x02\x02\x02\x1a\u{84}\
	\x03\x02\x02\x02\x1c\u{95}\x03\x02\x02\x02\x1e\u{a3}\x03\x02\x02\x02\x20\
	\u{ae}\x03\x02\x02\x02\x22\u{cb}\x03\x02\x02\x02\x24\u{cd}\x03\x02\x02\x02\
	\x26\u{d5}\x03\x02\x02\x02\x28\u{d8}\x03\x02\x02\x02\x2a\x2c\x05\x04\x03\
	\x02\x2b\x2a\x03\x02\x02\x02\x2c\x2f\x03\x02\x02\x02\x2d\x2b\x03\x02\x02\
	\x02\x2d\x2e\x03\x02\x02\x02\x2e\x30\x03\x02\x02\x02\x2f\x2d\x03\x02\x02\
	\x02\x30\x31\x07\x02\x02\x03\x31\x03\x03\x02\x02\x02\x32\x33\x05\x14\x0b\
	\x02\x33\x34\x07\x1b\x02\x02\x34\x3f\x03\x02\x02\x02\x35\x3f\x05\x08\x05\
	\x02\x36\x3f\x05\x06\x04\x02\x37\x3f\x05\x0a\x06\x02\x38\x3f\x05\x0c\x07\
	\x02\x39\x3f\x05\x0e\x08\x02\x3a\x3f\x05\x10\x09\x02\x3b\x3f\x05\x22\x12\
	\x02\x3c\x3f\x05\x24\x13\x02\x3d\x3f\x05\x28\x15\x02\x3e\x32\x03\x02\x02\
	\x02\x3e\x35\x03\x02\x02\x02\x3e\x36\x03\x02\x02\x02\x3e\x37\x03\x02\x02\
	\x02\x3e\x38\x03\x02\x02\x02\x3e\x39\x03\x02\x02\x02\x3e\x3a\x03\x02\x02\
	\x02\x3e\x3b\x03\x02\x02\x02\x3e\x3c\x03\x02\x02\x02\x3e\x3d\x03\x02\x02\
	\x02\x3f\x05\x03\x02\x02\x02\x40\x41\x07\x07\x02\x02\x41\x42\x07\x1c\x02\
	\x02\x42\x43\x07\x11\x02\x02\x43\x44\x05\x14\x0b\x02\x44\x45\x07\x1b\x02\
	\x02\x45\x07\x03\x02\x02\x02\x46\x47\x07\x08\x02\x02\x47\x48\x07\x1c\x02\
	\x02\x48\x49\x07\x11\x02\x02\x49\x4a\x05\x14\x0b\x02\x4a\x4b\x07\x1b\x02\
	\x02\x4b\x09\x03\x02\x02\x02\x4c\x4e\x07\x09\x02\x02\x4d\x4f\x05\x14\x0b\
	\x02\x4e\x4d\x03\x02\x02\x02\x4e\x4f\x03\x02\x02\x02\x4f\x50\x03\x02\x02\
	\x02\x50\x51\x07\x1b\x02\x02\x51\x0b\x03\x02\x02\x02\x52\x53\x07\x0a\x02\
	\x02\x53\x54\x07\x1d\x02\x02\x54\x55\x07\x1b\x02\x02\x55\x0d\x03\x02\x02\
	\x02\x56\x57\x07\x1c\x02\x02\x57\x58\x07\x11\x02\x02\x58\x59\x05\x14\x0b\
	\x02\x59\x5a\x07\x1b\x02\x02\x5a\x0f\x03\x02\x02\x02\x5b\x5c\x07\x06\x02\
	\x02\x5c\x5d\x07\x1c\x02\x02\x5d\x61\x07\x0b\x02\x02\x5e\x60\x07\x1c\x02\
	\x02\x5f\x5e\x03\x02\x02\x02\x60\x63\x03\x02\x02\x02\x61\x5f\x03\x02\x02\
	\x02\x61\x62\x03\x02\x02\x02\x62\x64\x03\x02\x02\x02\x63\x61\x03\x02\x02\
	\x02\x64\x65\x07\x0c\x02\x02\x65\x66\x05\x22\x12\x02\x66\x11\x03\x02\x02\
	\x02\x67\x68\x07\x1c\x02\x02\x68\x6c\x07\x0b\x02\x02\x69\x6b\x05\x14\x0b\
	\x02\x6a\x69\x03\x02\x02\x02\x6b\x6e\x03\x02\x02\x02\x6c\x6a\x03\x02\x02\
	\x02\x6c\x6d\x03\x02\x02\x02\x6d\x6f\x03\x02\x02\x02\x6e\x6c\x03\x02\x02\
	\x02\x6f\x70\x07\x0c\x02\x02\x70\x13\x03\x02\x02\x02\x71\x72\x05\x20\x11\
	\x02\x72\x15\x03\x02\x02\x02\x73\x7e\x07\x1c\x02\x02\x74\x7e\x07\x1d\x02\
	\x02\x75\x7e\x07\x1e\x02\x02\x76\x77\x07\x0b\x02\x02\x77\x78\x05\x14\x0b\
	\x02\x78\x79\x07\x0c\x02\x02\x79\x7e\x03\x02\x02\x02\x7a\x7e\x05\x24\x13\
	\x02\x7b\x7e\x05\x28\x15\x02\x7c\x7e\x05\x12\x0a\x02\x7d\x73\x03\x02\x02\
	\x02\x7d\x74\x03\x02\x02\x02\x7d\x75\x03\x02\x02\x02\x7d\x76\x03\x02\x02\
	\x02\x7d\x7a\x03\x02\x02\x02\x7d\x7b\x03\x02\x02\x02\x7d\x7c\x03\x02\x02\
	\x02\x7e\x17\x03\x02\x02\x02\x7f\u{80}\x07\x19\x02\x02\u{80}\u{83}\x05\x16\
	\x0c\x02\u{81}\u{83}\x05\x16\x0c\x02\u{82}\x7f\x03\x02\x02\x02\u{82}\u{81}\
	\x03\x02\x02\x02\u{83}\x19\x03\x02\x02\x02\u{84}\u{85}\x08\x0e\x01\x02\u{85}\
	\u{86}\x05\x18\x0d\x02\u{86}\u{92}\x03\x02\x02\x02\u{87}\u{88}\x0c\x05\x02\
	\x02\u{88}\u{89}\x07\x17\x02\x02\u{89}\u{91}\x05\x18\x0d\x02\u{8a}\u{8b}\
	\x0c\x04\x02\x02\u{8b}\u{8c}\x07\x16\x02\x02\u{8c}\u{91}\x05\x18\x0d\x02\
	\u{8d}\u{8e}\x0c\x03\x02\x02\u{8e}\u{8f}\x07\x13\x02\x02\u{8f}\u{91}\x05\
	\x18\x0d\x02\u{90}\u{87}\x03\x02\x02\x02\u{90}\u{8a}\x03\x02\x02\x02\u{90}\
	\u{8d}\x03\x02\x02\x02\u{91}\u{94}\x03\x02\x02\x02\u{92}\u{90}\x03\x02\x02\
	\x02\u{92}\u{93}\x03\x02\x02\x02\u{93}\x1b\x03\x02\x02\x02\u{94}\u{92}\x03\
	\x02\x02\x02\u{95}\u{96}\x08\x0f\x01\x02\u{96}\u{97}\x05\x1a\x0e\x02\u{97}\
	\u{a0}\x03\x02\x02\x02\u{98}\u{99}\x0c\x04\x02\x02\u{99}\u{9a}\x07\x18\x02\
	\x02\u{9a}\u{9f}\x05\x1a\x0e\x02\u{9b}\u{9c}\x0c\x03\x02\x02\u{9c}\u{9d}\
	\x07\x19\x02\x02\u{9d}\u{9f}\x05\x1a\x0e\x02\u{9e}\u{98}\x03\x02\x02\x02\
	\u{9e}\u{9b}\x03\x02\x02\x02\u{9f}\u{a2}\x03\x02\x02\x02\u{a0}\u{9e}\x03\
	\x02\x02\x02\u{a0}\u{a1}\x03\x02\x02\x02\u{a1}\x1d\x03\x02\x02\x02\u{a2}\
	\u{a0}\x03\x02\x02\x02\u{a3}\u{a4}\x08\x10\x01\x02\u{a4}\u{a5}\x05\x1c\x0f\
	\x02\u{a5}\u{ab}\x03\x02\x02\x02\u{a6}\u{a7}\x0c\x03\x02\x02\u{a7}\u{a8}\
	\x07\x14\x02\x02\u{a8}\u{aa}\x05\x1c\x0f\x02\u{a9}\u{a6}\x03\x02\x02\x02\
	\u{aa}\u{ad}\x03\x02\x02\x02\u{ab}\u{a9}\x03\x02\x02\x02\u{ab}\u{ac}\x03\
	\x02\x02\x02\u{ac}\x1f\x03\x02\x02\x02\u{ad}\u{ab}\x03\x02\x02\x02\u{ae}\
	\u{af}\x08\x11\x01\x02\u{af}\u{b0}\x05\x1e\x10\x02\u{b0}\u{b6}\x03\x02\x02\
	\x02\u{b1}\u{b2}\x0c\x03\x02\x02\u{b2}\u{b3}\x07\x15\x02\x02\u{b3}\u{b5}\
	\x05\x1e\x10\x02\u{b4}\u{b1}\x03\x02\x02\x02\u{b5}\u{b8}\x03\x02\x02\x02\
	\u{b6}\u{b4}\x03\x02\x02\x02\u{b6}\u{b7}\x03\x02\x02\x02\u{b7}\x21\x03\x02\
	\x02\x02\u{b8}\u{b6}\x03\x02\x02\x02\u{b9}\u{bd}\x07\x0f\x02\x02\u{ba}\u{bc}\
	\x05\x04\x03\x02\u{bb}\u{ba}\x03\x02\x02\x02\u{bc}\u{bf}\x03\x02\x02\x02\
	\u{bd}\u{bb}\x03\x02\x02\x02\u{bd}\u{be}\x03\x02\x02\x02\u{be}\u{c0}\x03\
	\x02\x02\x02\u{bf}\u{bd}\x03\x02\x02\x02\u{c0}\u{cc}\x07\x10\x02\x02\u{c1}\
	\u{c5}\x07\x0f\x02\x02\u{c2}\u{c4}\x05\x04\x03\x02\u{c3}\u{c2}\x03\x02\x02\
	\x02\u{c4}\u{c7}\x03\x02\x02\x02\u{c5}\u{c3}\x03\x02\x02\x02\u{c5}\u{c6}\
	\x03\x02\x02\x02\u{c6}\u{c8}\x03\x02\x02\x02\u{c7}\u{c5}\x03\x02\x02\x02\
	\u{c8}\u{c9}\x05\x14\x0b\x02\u{c9}\u{ca}\x07\x10\x02\x02\u{ca}\u{cc}\x03\
	\x02\x02\x02\u{cb}\u{b9}\x03\x02\x02\x02\u{cb}\u{c1}\x03\x02\x02\x02\u{cc}\
	\x23\x03\x02\x02\x02\u{cd}\u{ce}\x07\x03\x02\x02\u{ce}\u{cf}\x07\x0b\x02\
	\x02\u{cf}\u{d0}\x05\x14\x0b\x02\u{d0}\u{d1}\x07\x0c\x02\x02\u{d1}\u{d3}\
	\x05\x22\x12\x02\u{d2}\u{d4}\x05\x26\x14\x02\u{d3}\u{d2}\x03\x02\x02\x02\
	\u{d3}\u{d4}\x03\x02\x02\x02\u{d4}\x25\x03\x02\x02\x02\u{d5}\u{d6}\x07\x04\
	\x02\x02\u{d6}\u{d7}\x05\x04\x03\x02\u{d7}\x27\x03\x02\x02\x02\u{d8}\u{d9}\
	\x07\x05\x02\x02\u{d9}\u{da}\x07\x0b\x02\x02\u{da}\u{db}\x05\x14\x0b\x02\
	\u{db}\u{dc}\x07\x0c\x02\x02\u{dc}\u{dd}\x05\x22\x12\x02\u{dd}\x29\x03\x02\
	\x02\x02\x13\x2d\x3e\x4e\x61\x6c\x7d\u{82}\u{90}\u{92}\u{9e}\u{a0}\u{ab}\
	\u{b6}\u{bd}\u{c5}\u{cb}\u{d3}";
