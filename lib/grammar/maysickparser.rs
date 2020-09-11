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
pub const RULE_stmt_expr: usize = 2;
pub const RULE_stmt_let: usize = 3;
pub const RULE_stmt_var: usize = 4;
pub const RULE_stmt_return: usize = 5;
pub const RULE_stmt_import: usize = 6;
pub const RULE_stmt_assign: usize = 7;
pub const RULE_stmt_fn_def: usize = 8;
pub const RULE_fn_call: usize = 9;
pub const RULE_expr: usize = 10;
pub const RULE_expr_ident: usize = 11;
pub const RULE_expr_unary: usize = 12;
pub const RULE_expr_mul: usize = 13;
pub const RULE_expr_add: usize = 14;
pub const RULE_expr_and: usize = 15;
pub const RULE_expr_or: usize = 16;
pub const RULE_block: usize = 17;
pub const RULE_if_expr: usize = 18;
pub const RULE_else_stmt: usize = 19;
pub const RULE_while_expr: usize = 20;
pub const ruleNames: [&'static str; 21] = [
    "prog",
    "stmt",
    "stmt_expr",
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
            13 => Self::expr_mul_sempred(cast::<_, Expr_mulContext>(_localctx), pred_index, recog),
            14 => Self::expr_add_sempred(cast::<_, Expr_addContext>(_localctx), pred_index, recog),
            15 => Self::expr_and_sempred(cast::<_, Expr_andContext>(_localctx), pred_index, recog),
            16 => Self::expr_or_sempred(cast::<_, Expr_orContext>(_localctx), pred_index, recog),
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
                recog.base.set_state(45);
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
                            recog.base.set_state(42);
                            recog.stmt()?;
                        }
                    }
                    recog.base.set_state(47);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(48);
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
#[derive(Debug)]
pub enum StmtContextAll {
    StmtBlock_Context(StmtBlock_Context),
    StmtExpr_Context(StmtExpr_Context),
    StmtAssgn_Context(StmtAssgn_Context),
    StmtWhileContext(StmtWhileContext),
    StmtLet_Context(StmtLet_Context),
    StmtRet_Context(StmtRet_Context),
    StmtImport_Context(StmtImport_Context),
    StmtFnDef_Context(StmtFnDef_Context),
    StmtIfContext(StmtIfContext),
    StmtVar_Context(StmtVar_Context),
    Error(StmtContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for StmtContextAll {}

impl Deref for StmtContextAll {
    type Target = dyn StmtContextAttrs;
    fn deref(&self) -> &Self::Target {
        use StmtContextAll::*;
        match self {
            StmtBlock_Context(inner) => inner,
            StmtExpr_Context(inner) => inner,
            StmtAssgn_Context(inner) => inner,
            StmtWhileContext(inner) => inner,
            StmtLet_Context(inner) => inner,
            StmtRet_Context(inner) => inner,
            StmtImport_Context(inner) => inner,
            StmtFnDef_Context(inner) => inner,
            StmtIfContext(inner) => inner,
            StmtVar_Context(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type StmtContext = BaseParserRuleContext<StmtContextExt>;

#[derive(Clone)]
pub struct StmtContextExt {}
impl CustomRuleContext for StmtContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
}

impl StmtContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<StmtContextAll> {
        Rc::new(StmtContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, StmtContextExt {}),
        ))
    }
}

pub trait StmtContextAttrs: ParserRuleContext + BorrowMut<StmtContextExt> {}

impl StmtContextAttrs for StmtContext {}

//impl StmtContext{

//public StmtContext() { }
//pub fn copy_into(&self, ctx: StmtContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type StmtBlock_Context = BaseParserRuleContext<StmtBlock_ContextExt>;

pub trait StmtBlock_ContextAttrs: ParserRuleContext {
    fn block(&self) -> Option<Rc<BlockContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl StmtBlock_ContextAttrs for StmtBlock_Context {}

pub struct StmtBlock_ContextExt {
    base: StmtContextExt,
}

impl CustomRuleContext for StmtBlock_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtBlock_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtBlock_(ctx));
    }
}

impl Borrow<StmtContextExt> for StmtBlock_Context {
    fn borrow(&self) -> &StmtContextExt {
        &self.base
    }
}
impl BorrowMut<StmtContextExt> for StmtBlock_Context {
    fn borrow_mut(&mut self) -> &mut StmtContextExt {
        &mut self.base
    }
}

impl StmtContextAttrs for StmtBlock_Context {}

impl StmtBlock_ContextExt {
    fn new(ctx: &dyn StmtContextAttrs) -> Rc<StmtContextAll> {
        //let base = (cast::<_,StmtContext>(&ctx));
        Rc::new(StmtContextAll::StmtBlock_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtBlock_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type StmtExpr_Context = BaseParserRuleContext<StmtExpr_ContextExt>;

pub trait StmtExpr_ContextAttrs: ParserRuleContext {
    fn stmt_expr(&self) -> Option<Rc<Stmt_exprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl StmtExpr_ContextAttrs for StmtExpr_Context {}

pub struct StmtExpr_ContextExt {
    base: StmtContextExt,
}

impl CustomRuleContext for StmtExpr_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtExpr_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtExpr_(ctx));
    }
}

impl Borrow<StmtContextExt> for StmtExpr_Context {
    fn borrow(&self) -> &StmtContextExt {
        &self.base
    }
}
impl BorrowMut<StmtContextExt> for StmtExpr_Context {
    fn borrow_mut(&mut self) -> &mut StmtContextExt {
        &mut self.base
    }
}

impl StmtContextAttrs for StmtExpr_Context {}

impl StmtExpr_ContextExt {
    fn new(ctx: &dyn StmtContextAttrs) -> Rc<StmtContextAll> {
        //let base = (cast::<_,StmtContext>(&ctx));
        Rc::new(StmtContextAll::StmtExpr_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtExpr_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type StmtAssgn_Context = BaseParserRuleContext<StmtAssgn_ContextExt>;

pub trait StmtAssgn_ContextAttrs: ParserRuleContext {
    fn stmt_assign(&self) -> Option<Rc<Stmt_assignContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl StmtAssgn_ContextAttrs for StmtAssgn_Context {}

pub struct StmtAssgn_ContextExt {
    base: StmtContextExt,
}

impl CustomRuleContext for StmtAssgn_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtAssgn_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtAssgn_(ctx));
    }
}

impl Borrow<StmtContextExt> for StmtAssgn_Context {
    fn borrow(&self) -> &StmtContextExt {
        &self.base
    }
}
impl BorrowMut<StmtContextExt> for StmtAssgn_Context {
    fn borrow_mut(&mut self) -> &mut StmtContextExt {
        &mut self.base
    }
}

impl StmtContextAttrs for StmtAssgn_Context {}

impl StmtAssgn_ContextExt {
    fn new(ctx: &dyn StmtContextAttrs) -> Rc<StmtContextAll> {
        //let base = (cast::<_,StmtContext>(&ctx));
        Rc::new(StmtContextAll::StmtAssgn_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtAssgn_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type StmtWhileContext = BaseParserRuleContext<StmtWhileContextExt>;

pub trait StmtWhileContextAttrs: ParserRuleContext {
    fn while_expr(&self) -> Option<Rc<While_exprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl StmtWhileContextAttrs for StmtWhileContext {}

pub struct StmtWhileContextExt {
    base: StmtContextExt,
}

impl CustomRuleContext for StmtWhileContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtWhile(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtWhile(ctx));
    }
}

impl Borrow<StmtContextExt> for StmtWhileContext {
    fn borrow(&self) -> &StmtContextExt {
        &self.base
    }
}
impl BorrowMut<StmtContextExt> for StmtWhileContext {
    fn borrow_mut(&mut self) -> &mut StmtContextExt {
        &mut self.base
    }
}

impl StmtContextAttrs for StmtWhileContext {}

impl StmtWhileContextExt {
    fn new(ctx: &dyn StmtContextAttrs) -> Rc<StmtContextAll> {
        //let base = (cast::<_,StmtContext>(&ctx));
        Rc::new(StmtContextAll::StmtWhileContext(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtWhileContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type StmtLet_Context = BaseParserRuleContext<StmtLet_ContextExt>;

pub trait StmtLet_ContextAttrs: ParserRuleContext {
    fn stmt_let(&self) -> Option<Rc<Stmt_letContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl StmtLet_ContextAttrs for StmtLet_Context {}

pub struct StmtLet_ContextExt {
    base: StmtContextExt,
}

impl CustomRuleContext for StmtLet_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtLet_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtLet_(ctx));
    }
}

impl Borrow<StmtContextExt> for StmtLet_Context {
    fn borrow(&self) -> &StmtContextExt {
        &self.base
    }
}
impl BorrowMut<StmtContextExt> for StmtLet_Context {
    fn borrow_mut(&mut self) -> &mut StmtContextExt {
        &mut self.base
    }
}

impl StmtContextAttrs for StmtLet_Context {}

impl StmtLet_ContextExt {
    fn new(ctx: &dyn StmtContextAttrs) -> Rc<StmtContextAll> {
        //let base = (cast::<_,StmtContext>(&ctx));
        Rc::new(StmtContextAll::StmtLet_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtLet_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type StmtRet_Context = BaseParserRuleContext<StmtRet_ContextExt>;

pub trait StmtRet_ContextAttrs: ParserRuleContext {
    fn stmt_return(&self) -> Option<Rc<Stmt_returnContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl StmtRet_ContextAttrs for StmtRet_Context {}

pub struct StmtRet_ContextExt {
    base: StmtContextExt,
}

impl CustomRuleContext for StmtRet_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtRet_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtRet_(ctx));
    }
}

impl Borrow<StmtContextExt> for StmtRet_Context {
    fn borrow(&self) -> &StmtContextExt {
        &self.base
    }
}
impl BorrowMut<StmtContextExt> for StmtRet_Context {
    fn borrow_mut(&mut self) -> &mut StmtContextExt {
        &mut self.base
    }
}

impl StmtContextAttrs for StmtRet_Context {}

impl StmtRet_ContextExt {
    fn new(ctx: &dyn StmtContextAttrs) -> Rc<StmtContextAll> {
        //let base = (cast::<_,StmtContext>(&ctx));
        Rc::new(StmtContextAll::StmtRet_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtRet_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type StmtImport_Context = BaseParserRuleContext<StmtImport_ContextExt>;

pub trait StmtImport_ContextAttrs: ParserRuleContext {
    fn stmt_import(&self) -> Option<Rc<Stmt_importContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl StmtImport_ContextAttrs for StmtImport_Context {}

pub struct StmtImport_ContextExt {
    base: StmtContextExt,
}

impl CustomRuleContext for StmtImport_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtImport_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtImport_(ctx));
    }
}

impl Borrow<StmtContextExt> for StmtImport_Context {
    fn borrow(&self) -> &StmtContextExt {
        &self.base
    }
}
impl BorrowMut<StmtContextExt> for StmtImport_Context {
    fn borrow_mut(&mut self) -> &mut StmtContextExt {
        &mut self.base
    }
}

impl StmtContextAttrs for StmtImport_Context {}

impl StmtImport_ContextExt {
    fn new(ctx: &dyn StmtContextAttrs) -> Rc<StmtContextAll> {
        //let base = (cast::<_,StmtContext>(&ctx));
        Rc::new(StmtContextAll::StmtImport_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtImport_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type StmtFnDef_Context = BaseParserRuleContext<StmtFnDef_ContextExt>;

pub trait StmtFnDef_ContextAttrs: ParserRuleContext {
    fn stmt_fn_def(&self) -> Option<Rc<Stmt_fn_defContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl StmtFnDef_ContextAttrs for StmtFnDef_Context {}

pub struct StmtFnDef_ContextExt {
    base: StmtContextExt,
}

impl CustomRuleContext for StmtFnDef_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtFnDef_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtFnDef_(ctx));
    }
}

impl Borrow<StmtContextExt> for StmtFnDef_Context {
    fn borrow(&self) -> &StmtContextExt {
        &self.base
    }
}
impl BorrowMut<StmtContextExt> for StmtFnDef_Context {
    fn borrow_mut(&mut self) -> &mut StmtContextExt {
        &mut self.base
    }
}

impl StmtContextAttrs for StmtFnDef_Context {}

impl StmtFnDef_ContextExt {
    fn new(ctx: &dyn StmtContextAttrs) -> Rc<StmtContextAll> {
        //let base = (cast::<_,StmtContext>(&ctx));
        Rc::new(StmtContextAll::StmtFnDef_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtFnDef_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type StmtIfContext = BaseParserRuleContext<StmtIfContextExt>;

pub trait StmtIfContextAttrs: ParserRuleContext {
    fn if_expr(&self) -> Option<Rc<If_exprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl StmtIfContextAttrs for StmtIfContext {}

pub struct StmtIfContextExt {
    base: StmtContextExt,
}

impl CustomRuleContext for StmtIfContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtIf(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtIf(ctx));
    }
}

impl Borrow<StmtContextExt> for StmtIfContext {
    fn borrow(&self) -> &StmtContextExt {
        &self.base
    }
}
impl BorrowMut<StmtContextExt> for StmtIfContext {
    fn borrow_mut(&mut self) -> &mut StmtContextExt {
        &mut self.base
    }
}

impl StmtContextAttrs for StmtIfContext {}

impl StmtIfContextExt {
    fn new(ctx: &dyn StmtContextAttrs) -> Rc<StmtContextAll> {
        //let base = (cast::<_,StmtContext>(&ctx));
        Rc::new(StmtContextAll::StmtIfContext(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtIfContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type StmtVar_Context = BaseParserRuleContext<StmtVar_ContextExt>;

pub trait StmtVar_ContextAttrs: ParserRuleContext {
    fn stmt_var(&self) -> Option<Rc<Stmt_varContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl StmtVar_ContextAttrs for StmtVar_Context {}

pub struct StmtVar_ContextExt {
    base: StmtContextExt,
}

impl CustomRuleContext for StmtVar_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtVar_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtVar_(ctx));
    }
}

impl Borrow<StmtContextExt> for StmtVar_Context {
    fn borrow(&self) -> &StmtContextExt {
        &self.base
    }
}
impl BorrowMut<StmtContextExt> for StmtVar_Context {
    fn borrow_mut(&mut self) -> &mut StmtContextExt {
        &mut self.base
    }
}

impl StmtContextAttrs for StmtVar_Context {}

impl StmtVar_ContextExt {
    fn new(ctx: &dyn StmtContextAttrs) -> Rc<StmtContextAll> {
        //let base = (cast::<_,StmtContext>(&ctx));
        Rc::new(StmtContextAll::StmtVar_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtVar_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

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
                    let tmp = StmtExpr_ContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        /*InvokeRule stmt_expr*/
                        recog.base.set_state(50);
                        recog.stmt_expr()?;
                    }
                }
                2 => {
                    let tmp = StmtVar_ContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        /*InvokeRule stmt_var*/
                        recog.base.set_state(51);
                        recog.stmt_var()?;
                    }
                }
                3 => {
                    let tmp = StmtLet_ContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        /*InvokeRule stmt_let*/
                        recog.base.set_state(52);
                        recog.stmt_let()?;
                    }
                }
                4 => {
                    let tmp = StmtRet_ContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        /*InvokeRule stmt_return*/
                        recog.base.set_state(53);
                        recog.stmt_return()?;
                    }
                }
                5 => {
                    let tmp = StmtImport_ContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 5);
                    _localctx = tmp;
                    {
                        /*InvokeRule stmt_import*/
                        recog.base.set_state(54);
                        recog.stmt_import()?;
                    }
                }
                6 => {
                    let tmp = StmtAssgn_ContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 6);
                    _localctx = tmp;
                    {
                        /*InvokeRule stmt_assign*/
                        recog.base.set_state(55);
                        recog.stmt_assign()?;
                    }
                }
                7 => {
                    let tmp = StmtFnDef_ContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 7);
                    _localctx = tmp;
                    {
                        /*InvokeRule stmt_fn_def*/
                        recog.base.set_state(56);
                        recog.stmt_fn_def()?;
                    }
                }
                8 => {
                    let tmp = StmtBlock_ContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 8);
                    _localctx = tmp;
                    {
                        /*InvokeRule block*/
                        recog.base.set_state(57);
                        recog.block()?;
                    }
                }
                9 => {
                    let tmp = StmtIfContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 9);
                    _localctx = tmp;
                    {
                        /*InvokeRule if_expr*/
                        recog.base.set_state(58);
                        recog.if_expr()?;
                    }
                }
                10 => {
                    let tmp = StmtWhileContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 10);
                    _localctx = tmp;
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
//------------------- stmt_expr ----------------
#[derive(Debug)]
pub enum Stmt_exprContextAll {
    StmtExprContext(StmtExprContext),
    Error(Stmt_exprContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Stmt_exprContextAll {}

impl Deref for Stmt_exprContextAll {
    type Target = dyn Stmt_exprContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Stmt_exprContextAll::*;
        match self {
            StmtExprContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Stmt_exprContext = BaseParserRuleContext<Stmt_exprContextExt>;

#[derive(Clone)]
pub struct Stmt_exprContextExt {}
impl CustomRuleContext for Stmt_exprContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_expr
    }
}

impl Stmt_exprContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Stmt_exprContextAll> {
        Rc::new(Stmt_exprContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Stmt_exprContextExt {}),
        ))
    }
}

pub trait Stmt_exprContextAttrs: ParserRuleContext + BorrowMut<Stmt_exprContextExt> {}

impl Stmt_exprContextAttrs for Stmt_exprContext {}

//impl Stmt_exprContext{

//public Stmt_exprContext() { }
//pub fn copy_into(&self, ctx: Stmt_exprContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type StmtExprContext = BaseParserRuleContext<StmtExprContextExt>;

pub trait StmtExprContextAttrs: ParserRuleContext {
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

impl StmtExprContextAttrs for StmtExprContext {}

pub struct StmtExprContextExt {
    base: Stmt_exprContextExt,
}

impl CustomRuleContext for StmtExprContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_expr
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtExpr(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtExpr(ctx));
    }
}

impl Borrow<Stmt_exprContextExt> for StmtExprContext {
    fn borrow(&self) -> &Stmt_exprContextExt {
        &self.base
    }
}
impl BorrowMut<Stmt_exprContextExt> for StmtExprContext {
    fn borrow_mut(&mut self) -> &mut Stmt_exprContextExt {
        &mut self.base
    }
}

impl Stmt_exprContextAttrs for StmtExprContext {}

impl StmtExprContextExt {
    fn new(ctx: &dyn Stmt_exprContextAttrs) -> Rc<Stmt_exprContextAll> {
        //let base = (cast::<_,Stmt_exprContext>(&ctx));
        Rc::new(Stmt_exprContextAll::StmtExprContext(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtExprContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn stmt_expr(&mut self) -> Result<Rc<Stmt_exprContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_stmt_expr);
        let mut _localctx: Rc<Stmt_exprContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let tmp = StmtExprContextExt::new(&**_localctx);
            recog.base.enter_outer_alt(Some(tmp.clone()), 1);
            _localctx = tmp;
            {
                /*InvokeRule expr*/
                recog.base.set_state(62);
                recog.expr()?;

                recog.base.set_state(63);
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
//------------------- stmt_let ----------------
#[derive(Debug)]
pub enum Stmt_letContextAll {
    StmtLetContext(StmtLetContext),
    Error(Stmt_letContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Stmt_letContextAll {}

impl Deref for Stmt_letContextAll {
    type Target = dyn Stmt_letContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Stmt_letContextAll::*;
        match self {
            StmtLetContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Stmt_letContext = BaseParserRuleContext<Stmt_letContextExt>;

#[derive(Clone)]
pub struct Stmt_letContextExt {}
impl CustomRuleContext for Stmt_letContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_let
    }
}

impl Stmt_letContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Stmt_letContextAll> {
        Rc::new(Stmt_letContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Stmt_letContextExt {}),
        ))
    }
}

pub trait Stmt_letContextAttrs: ParserRuleContext + BorrowMut<Stmt_letContextExt> {}

impl Stmt_letContextAttrs for Stmt_letContext {}

//impl Stmt_letContext{

//public Stmt_letContext() { }
//pub fn copy_into(&self, ctx: Stmt_letContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type StmtLetContext = BaseParserRuleContext<StmtLetContextExt>;

pub trait StmtLetContextAttrs: ParserRuleContext {
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

impl StmtLetContextAttrs for StmtLetContext {}

pub struct StmtLetContextExt {
    base: Stmt_letContextExt,
}

impl CustomRuleContext for StmtLetContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_let
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtLet(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtLet(ctx));
    }
}

impl Borrow<Stmt_letContextExt> for StmtLetContext {
    fn borrow(&self) -> &Stmt_letContextExt {
        &self.base
    }
}
impl BorrowMut<Stmt_letContextExt> for StmtLetContext {
    fn borrow_mut(&mut self) -> &mut Stmt_letContextExt {
        &mut self.base
    }
}

impl Stmt_letContextAttrs for StmtLetContext {}

impl StmtLetContextExt {
    fn new(ctx: &dyn Stmt_letContextAttrs) -> Rc<Stmt_letContextAll> {
        //let base = (cast::<_,Stmt_letContext>(&ctx));
        Rc::new(Stmt_letContextAll::StmtLetContext(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtLetContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn stmt_let(&mut self) -> Result<Rc<Stmt_letContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_letContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_stmt_let);
        let mut _localctx: Rc<Stmt_letContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let tmp = StmtLetContextExt::new(&**_localctx);
            recog.base.enter_outer_alt(Some(tmp.clone()), 1);
            _localctx = tmp;
            {
                recog.base.set_state(65);
                recog.base.match_token(LET, recog.err_handler.as_mut())?;

                recog.base.set_state(66);
                recog.base.match_token(IDENT, recog.err_handler.as_mut())?;

                recog.base.set_state(67);
                recog.base.match_token(SYM_EQ, recog.err_handler.as_mut())?;

                /*InvokeRule expr*/
                recog.base.set_state(68);
                recog.expr()?;

                recog.base.set_state(69);
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
#[derive(Debug)]
pub enum Stmt_varContextAll {
    StmtVarContext(StmtVarContext),
    Error(Stmt_varContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Stmt_varContextAll {}

impl Deref for Stmt_varContextAll {
    type Target = dyn Stmt_varContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Stmt_varContextAll::*;
        match self {
            StmtVarContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Stmt_varContext = BaseParserRuleContext<Stmt_varContextExt>;

#[derive(Clone)]
pub struct Stmt_varContextExt {}
impl CustomRuleContext for Stmt_varContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_var
    }
}

impl Stmt_varContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Stmt_varContextAll> {
        Rc::new(Stmt_varContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Stmt_varContextExt {}),
        ))
    }
}

pub trait Stmt_varContextAttrs: ParserRuleContext + BorrowMut<Stmt_varContextExt> {}

impl Stmt_varContextAttrs for Stmt_varContext {}

//impl Stmt_varContext{

//public Stmt_varContext() { }
//pub fn copy_into(&self, ctx: Stmt_varContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type StmtVarContext = BaseParserRuleContext<StmtVarContextExt>;

pub trait StmtVarContextAttrs: ParserRuleContext {
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

impl StmtVarContextAttrs for StmtVarContext {}

pub struct StmtVarContextExt {
    base: Stmt_varContextExt,
}

impl CustomRuleContext for StmtVarContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_var
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtVar(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtVar(ctx));
    }
}

impl Borrow<Stmt_varContextExt> for StmtVarContext {
    fn borrow(&self) -> &Stmt_varContextExt {
        &self.base
    }
}
impl BorrowMut<Stmt_varContextExt> for StmtVarContext {
    fn borrow_mut(&mut self) -> &mut Stmt_varContextExt {
        &mut self.base
    }
}

impl Stmt_varContextAttrs for StmtVarContext {}

impl StmtVarContextExt {
    fn new(ctx: &dyn Stmt_varContextAttrs) -> Rc<Stmt_varContextAll> {
        //let base = (cast::<_,Stmt_varContext>(&ctx));
        Rc::new(Stmt_varContextAll::StmtVarContext(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtVarContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn stmt_var(&mut self) -> Result<Rc<Stmt_varContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_varContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_stmt_var);
        let mut _localctx: Rc<Stmt_varContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let tmp = StmtVarContextExt::new(&**_localctx);
            recog.base.enter_outer_alt(Some(tmp.clone()), 1);
            _localctx = tmp;
            {
                recog.base.set_state(71);
                recog.base.match_token(VAR, recog.err_handler.as_mut())?;

                recog.base.set_state(72);
                recog.base.match_token(IDENT, recog.err_handler.as_mut())?;

                recog.base.set_state(73);
                recog.base.match_token(SYM_EQ, recog.err_handler.as_mut())?;

                /*InvokeRule expr*/
                recog.base.set_state(74);
                recog.expr()?;

                recog.base.set_state(75);
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
#[derive(Debug)]
pub enum Stmt_returnContextAll {
    StmtRetContext(StmtRetContext),
    Error(Stmt_returnContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Stmt_returnContextAll {}

impl Deref for Stmt_returnContextAll {
    type Target = dyn Stmt_returnContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Stmt_returnContextAll::*;
        match self {
            StmtRetContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Stmt_returnContext = BaseParserRuleContext<Stmt_returnContextExt>;

#[derive(Clone)]
pub struct Stmt_returnContextExt {}
impl CustomRuleContext for Stmt_returnContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_return
    }
}

impl Stmt_returnContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Stmt_returnContextAll> {
        Rc::new(Stmt_returnContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Stmt_returnContextExt {}),
        ))
    }
}

pub trait Stmt_returnContextAttrs: ParserRuleContext + BorrowMut<Stmt_returnContextExt> {}

impl Stmt_returnContextAttrs for Stmt_returnContext {}

//impl Stmt_returnContext{

//public Stmt_returnContext() { }
//pub fn copy_into(&self, ctx: Stmt_returnContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type StmtRetContext = BaseParserRuleContext<StmtRetContextExt>;

pub trait StmtRetContextAttrs: ParserRuleContext {
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

impl StmtRetContextAttrs for StmtRetContext {}

pub struct StmtRetContextExt {
    base: Stmt_returnContextExt,
}

impl CustomRuleContext for StmtRetContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_return
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtRet(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtRet(ctx));
    }
}

impl Borrow<Stmt_returnContextExt> for StmtRetContext {
    fn borrow(&self) -> &Stmt_returnContextExt {
        &self.base
    }
}
impl BorrowMut<Stmt_returnContextExt> for StmtRetContext {
    fn borrow_mut(&mut self) -> &mut Stmt_returnContextExt {
        &mut self.base
    }
}

impl Stmt_returnContextAttrs for StmtRetContext {}

impl StmtRetContextExt {
    fn new(ctx: &dyn Stmt_returnContextAttrs) -> Rc<Stmt_returnContextAll> {
        //let base = (cast::<_,Stmt_returnContext>(&ctx));
        Rc::new(Stmt_returnContextAll::StmtRetContext(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtRetContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn stmt_return(&mut self) -> Result<Rc<Stmt_returnContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_returnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 10, RULE_stmt_return);
        let mut _localctx: Rc<Stmt_returnContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let tmp = StmtRetContextExt::new(&**_localctx);
            recog.base.enter_outer_alt(Some(tmp.clone()), 1);
            _localctx = tmp;
            {
                recog.base.set_state(77);
                recog.base.match_token(RETURN, recog.err_handler.as_mut())?;

                recog.base.set_state(79);
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
                        recog.base.set_state(78);
                        recog.expr()?;
                    }
                }

                recog.base.set_state(81);
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
#[derive(Debug)]
pub enum Stmt_importContextAll {
    StmtImportContext(StmtImportContext),
    Error(Stmt_importContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Stmt_importContextAll {}

impl Deref for Stmt_importContextAll {
    type Target = dyn Stmt_importContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Stmt_importContextAll::*;
        match self {
            StmtImportContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Stmt_importContext = BaseParserRuleContext<Stmt_importContextExt>;

#[derive(Clone)]
pub struct Stmt_importContextExt {}
impl CustomRuleContext for Stmt_importContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_import
    }
}

impl Stmt_importContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Stmt_importContextAll> {
        Rc::new(Stmt_importContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Stmt_importContextExt {}),
        ))
    }
}

pub trait Stmt_importContextAttrs: ParserRuleContext + BorrowMut<Stmt_importContextExt> {}

impl Stmt_importContextAttrs for Stmt_importContext {}

//impl Stmt_importContext{

//public Stmt_importContext() { }
//pub fn copy_into(&self, ctx: Stmt_importContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type StmtImportContext = BaseParserRuleContext<StmtImportContextExt>;

pub trait StmtImportContextAttrs: ParserRuleContext {
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

impl StmtImportContextAttrs for StmtImportContext {}

pub struct StmtImportContextExt {
    base: Stmt_importContextExt,
}

impl CustomRuleContext for StmtImportContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_import
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtImport(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtImport(ctx));
    }
}

impl Borrow<Stmt_importContextExt> for StmtImportContext {
    fn borrow(&self) -> &Stmt_importContextExt {
        &self.base
    }
}
impl BorrowMut<Stmt_importContextExt> for StmtImportContext {
    fn borrow_mut(&mut self) -> &mut Stmt_importContextExt {
        &mut self.base
    }
}

impl Stmt_importContextAttrs for StmtImportContext {}

impl StmtImportContextExt {
    fn new(ctx: &dyn Stmt_importContextAttrs) -> Rc<Stmt_importContextAll> {
        //let base = (cast::<_,Stmt_importContext>(&ctx));
        Rc::new(Stmt_importContextAll::StmtImportContext(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtImportContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn stmt_import(&mut self) -> Result<Rc<Stmt_importContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_importContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 12, RULE_stmt_import);
        let mut _localctx: Rc<Stmt_importContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let tmp = StmtImportContextExt::new(&**_localctx);
            recog.base.enter_outer_alt(Some(tmp.clone()), 1);
            _localctx = tmp;
            {
                recog.base.set_state(83);
                recog.base.match_token(IMPORT, recog.err_handler.as_mut())?;

                recog.base.set_state(84);
                recog
                    .base
                    .match_token(LIT_STRING, recog.err_handler.as_mut())?;

                recog.base.set_state(85);
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
#[derive(Debug)]
pub enum Stmt_assignContextAll {
    StmtAssgnContext(StmtAssgnContext),
    Error(Stmt_assignContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Stmt_assignContextAll {}

impl Deref for Stmt_assignContextAll {
    type Target = dyn Stmt_assignContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Stmt_assignContextAll::*;
        match self {
            StmtAssgnContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Stmt_assignContext = BaseParserRuleContext<Stmt_assignContextExt>;

#[derive(Clone)]
pub struct Stmt_assignContextExt {}
impl CustomRuleContext for Stmt_assignContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_assign
    }
}

impl Stmt_assignContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Stmt_assignContextAll> {
        Rc::new(Stmt_assignContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Stmt_assignContextExt {}),
        ))
    }
}

pub trait Stmt_assignContextAttrs: ParserRuleContext + BorrowMut<Stmt_assignContextExt> {}

impl Stmt_assignContextAttrs for Stmt_assignContext {}

//impl Stmt_assignContext{

//public Stmt_assignContext() { }
//pub fn copy_into(&self, ctx: Stmt_assignContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type StmtAssgnContext = BaseParserRuleContext<StmtAssgnContextExt>;

pub trait StmtAssgnContextAttrs: ParserRuleContext {
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

impl StmtAssgnContextAttrs for StmtAssgnContext {}

pub struct StmtAssgnContextExt {
    base: Stmt_assignContextExt,
}

impl CustomRuleContext for StmtAssgnContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_assign
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtAssgn(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtAssgn(ctx));
    }
}

impl Borrow<Stmt_assignContextExt> for StmtAssgnContext {
    fn borrow(&self) -> &Stmt_assignContextExt {
        &self.base
    }
}
impl BorrowMut<Stmt_assignContextExt> for StmtAssgnContext {
    fn borrow_mut(&mut self) -> &mut Stmt_assignContextExt {
        &mut self.base
    }
}

impl Stmt_assignContextAttrs for StmtAssgnContext {}

impl StmtAssgnContextExt {
    fn new(ctx: &dyn Stmt_assignContextAttrs) -> Rc<Stmt_assignContextAll> {
        //let base = (cast::<_,Stmt_assignContext>(&ctx));
        Rc::new(Stmt_assignContextAll::StmtAssgnContext(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtAssgnContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn stmt_assign(&mut self) -> Result<Rc<Stmt_assignContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_assignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 14, RULE_stmt_assign);
        let mut _localctx: Rc<Stmt_assignContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let tmp = StmtAssgnContextExt::new(&**_localctx);
            recog.base.enter_outer_alt(Some(tmp.clone()), 1);
            _localctx = tmp;
            {
                recog.base.set_state(87);
                recog.base.match_token(IDENT, recog.err_handler.as_mut())?;

                recog.base.set_state(88);
                recog.base.match_token(SYM_EQ, recog.err_handler.as_mut())?;

                /*InvokeRule expr*/
                recog.base.set_state(89);
                recog.expr()?;

                recog.base.set_state(90);
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
#[derive(Debug)]
pub enum Stmt_fn_defContextAll {
    StmtFnDefContext(StmtFnDefContext),
    Error(Stmt_fn_defContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Stmt_fn_defContextAll {}

impl Deref for Stmt_fn_defContextAll {
    type Target = dyn Stmt_fn_defContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Stmt_fn_defContextAll::*;
        match self {
            StmtFnDefContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Stmt_fn_defContext = BaseParserRuleContext<Stmt_fn_defContextExt>;

#[derive(Clone)]
pub struct Stmt_fn_defContextExt {}
impl CustomRuleContext for Stmt_fn_defContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_fn_def
    }
}

impl Stmt_fn_defContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Stmt_fn_defContextAll> {
        Rc::new(Stmt_fn_defContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Stmt_fn_defContextExt {}),
        ))
    }
}

pub trait Stmt_fn_defContextAttrs: ParserRuleContext + BorrowMut<Stmt_fn_defContextExt> {}

impl Stmt_fn_defContextAttrs for Stmt_fn_defContext {}

//impl Stmt_fn_defContext{

//public Stmt_fn_defContext() { }
//pub fn copy_into(&self, ctx: Stmt_fn_defContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type StmtFnDefContext = BaseParserRuleContext<StmtFnDefContextExt>;

pub trait StmtFnDefContextAttrs: ParserRuleContext {
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

impl StmtFnDefContextAttrs for StmtFnDefContext {}

pub struct StmtFnDefContextExt {
    base: Stmt_fn_defContextExt,
}

impl CustomRuleContext for StmtFnDefContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_stmt_fn_def
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_StmtFnDef(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_StmtFnDef(ctx));
    }
}

impl Borrow<Stmt_fn_defContextExt> for StmtFnDefContext {
    fn borrow(&self) -> &Stmt_fn_defContextExt {
        &self.base
    }
}
impl BorrowMut<Stmt_fn_defContextExt> for StmtFnDefContext {
    fn borrow_mut(&mut self) -> &mut Stmt_fn_defContextExt {
        &mut self.base
    }
}

impl Stmt_fn_defContextAttrs for StmtFnDefContext {}

impl StmtFnDefContextExt {
    fn new(ctx: &dyn Stmt_fn_defContextAttrs) -> Rc<Stmt_fn_defContextAll> {
        //let base = (cast::<_,Stmt_fn_defContext>(&ctx));
        Rc::new(Stmt_fn_defContextAll::StmtFnDefContext(
            BaseParserRuleContext::copy_from(
                ctx,
                StmtFnDefContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn stmt_fn_def(&mut self) -> Result<Rc<Stmt_fn_defContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Stmt_fn_defContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 16, RULE_stmt_fn_def);
        let mut _localctx: Rc<Stmt_fn_defContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let tmp = StmtFnDefContextExt::new(&**_localctx);
            recog.base.enter_outer_alt(Some(tmp.clone()), 1);
            _localctx = tmp;
            {
                recog.base.set_state(92);
                recog.base.match_token(FN, recog.err_handler.as_mut())?;

                recog.base.set_state(93);
                recog.base.match_token(IDENT, recog.err_handler.as_mut())?;

                recog.base.set_state(94);
                recog
                    .base
                    .match_token(SYM_LPAREN, recog.err_handler.as_mut())?;

                recog.base.set_state(98);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == IDENT {
                    {
                        {
                            recog.base.set_state(95);
                            recog.base.match_token(IDENT, recog.err_handler.as_mut())?;
                        }
                    }
                    recog.base.set_state(100);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(101);
                recog
                    .base
                    .match_token(SYM_RPAREN, recog.err_handler.as_mut())?;

                /*InvokeRule block*/
                recog.base.set_state(102);
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
#[derive(Debug)]
pub enum Fn_callContextAll {
    FnCallContext(FnCallContext),
    Error(Fn_callContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Fn_callContextAll {}

impl Deref for Fn_callContextAll {
    type Target = dyn Fn_callContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Fn_callContextAll::*;
        match self {
            FnCallContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Fn_callContext = BaseParserRuleContext<Fn_callContextExt>;

#[derive(Clone)]
pub struct Fn_callContextExt {}
impl CustomRuleContext for Fn_callContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_fn_call
    }
}

impl Fn_callContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Fn_callContextAll> {
        Rc::new(Fn_callContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Fn_callContextExt {}),
        ))
    }
}

pub trait Fn_callContextAttrs: ParserRuleContext + BorrowMut<Fn_callContextExt> {}

impl Fn_callContextAttrs for Fn_callContext {}

//impl Fn_callContext{

//public Fn_callContext() { }
//pub fn copy_into(&self, ctx: Fn_callContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type FnCallContext = BaseParserRuleContext<FnCallContextExt>;

pub trait FnCallContextAttrs: ParserRuleContext {
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

impl FnCallContextAttrs for FnCallContext {}

pub struct FnCallContextExt {
    base: Fn_callContextExt,
}

impl CustomRuleContext for FnCallContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_fn_call
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_FnCall(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_FnCall(ctx));
    }
}

impl Borrow<Fn_callContextExt> for FnCallContext {
    fn borrow(&self) -> &Fn_callContextExt {
        &self.base
    }
}
impl BorrowMut<Fn_callContextExt> for FnCallContext {
    fn borrow_mut(&mut self) -> &mut Fn_callContextExt {
        &mut self.base
    }
}

impl Fn_callContextAttrs for FnCallContext {}

impl FnCallContextExt {
    fn new(ctx: &dyn Fn_callContextAttrs) -> Rc<Fn_callContextAll> {
        //let base = (cast::<_,Fn_callContext>(&ctx));
        Rc::new(Fn_callContextAll::FnCallContext(
            BaseParserRuleContext::copy_from(
                ctx,
                FnCallContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn fn_call(&mut self) -> Result<Rc<Fn_callContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Fn_callContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_fn_call);
        let mut _localctx: Rc<Fn_callContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let tmp = FnCallContextExt::new(&**_localctx);
            recog.base.enter_outer_alt(Some(tmp.clone()), 1);
            _localctx = tmp;
            {
                recog.base.set_state(104);
                recog.base.match_token(IDENT, recog.err_handler.as_mut())?;

                recog.base.set_state(105);
                recog
                    .base
                    .match_token(SYM_LPAREN, recog.err_handler.as_mut())?;

                recog.base.set_state(109);
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
                            recog.base.set_state(106);
                            recog.expr()?;
                        }
                    }
                    recog.base.set_state(111);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(112);
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
#[derive(Debug)]
pub enum ExprContextAll {
    Expr_Or_Context(Expr_Or_Context),
    Error(ExprContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for ExprContextAll {}

impl Deref for ExprContextAll {
    type Target = dyn ExprContextAttrs;
    fn deref(&self) -> &Self::Target {
        use ExprContextAll::*;
        match self {
            Expr_Or_Context(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type ExprContext = BaseParserRuleContext<ExprContextExt>;

#[derive(Clone)]
pub struct ExprContextExt {}
impl CustomRuleContext for ExprContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
}

impl ExprContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<ExprContextAll> {
        Rc::new(ExprContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, ExprContextExt {}),
        ))
    }
}

pub trait ExprContextAttrs: ParserRuleContext + BorrowMut<ExprContextExt> {}

impl ExprContextAttrs for ExprContext {}

//impl ExprContext{

//public ExprContext() { }
//pub fn copy_into(&self, ctx: ExprContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type Expr_Or_Context = BaseParserRuleContext<Expr_Or_ContextExt>;

pub trait Expr_Or_ContextAttrs: ParserRuleContext {
    fn expr_or(&self) -> Option<Rc<Expr_orContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl Expr_Or_ContextAttrs for Expr_Or_Context {}

pub struct Expr_Or_ContextExt {
    base: ExprContextExt,
}

impl CustomRuleContext for Expr_Or_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_Expr_Or_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_Expr_Or_(ctx));
    }
}

impl Borrow<ExprContextExt> for Expr_Or_Context {
    fn borrow(&self) -> &ExprContextExt {
        &self.base
    }
}
impl BorrowMut<ExprContextExt> for Expr_Or_Context {
    fn borrow_mut(&mut self) -> &mut ExprContextExt {
        &mut self.base
    }
}

impl ExprContextAttrs for Expr_Or_Context {}

impl Expr_Or_ContextExt {
    fn new(ctx: &dyn ExprContextAttrs) -> Rc<ExprContextAll> {
        //let base = (cast::<_,ExprContext>(&ctx));
        Rc::new(ExprContextAll::Expr_Or_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                Expr_Or_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn expr(&mut self) -> Result<Rc<ExprContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_expr);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let tmp = Expr_Or_ContextExt::new(&**_localctx);
            recog.base.enter_outer_alt(Some(tmp.clone()), 1);
            _localctx = tmp;
            {
                /*InvokeRule expr_or*/
                recog.base.set_state(114);
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
#[derive(Debug)]
pub enum Expr_identContextAll {
    ExprIdent_NumLiteralContext(ExprIdent_NumLiteralContext),
    ExprIdent_ParenContext(ExprIdent_ParenContext),
    ExprIdent_IdentContext(ExprIdent_IdentContext),
    ExprIdent_StrLiteralContext(ExprIdent_StrLiteralContext),
    Error(Expr_identContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Expr_identContextAll {}

impl Deref for Expr_identContextAll {
    type Target = dyn Expr_identContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Expr_identContextAll::*;
        match self {
            ExprIdent_NumLiteralContext(inner) => inner,
            ExprIdent_ParenContext(inner) => inner,
            ExprIdent_IdentContext(inner) => inner,
            ExprIdent_StrLiteralContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Expr_identContext = BaseParserRuleContext<Expr_identContextExt>;

#[derive(Clone)]
pub struct Expr_identContextExt {}
impl CustomRuleContext for Expr_identContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_ident
    }
}

impl Expr_identContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Expr_identContextAll> {
        Rc::new(Expr_identContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Expr_identContextExt {}),
        ))
    }
}

pub trait Expr_identContextAttrs: ParserRuleContext + BorrowMut<Expr_identContextExt> {}

impl Expr_identContextAttrs for Expr_identContext {}

//impl Expr_identContext{

//public Expr_identContext() { }
//pub fn copy_into(&self, ctx: Expr_identContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type ExprIdent_NumLiteralContext = BaseParserRuleContext<ExprIdent_NumLiteralContextExt>;

pub trait ExprIdent_NumLiteralContextAttrs: ParserRuleContext {
    /// Retrieves first TerminalNode corresponding to token LIT_NUMBER
    /// Returns `None` if there is no child corresponding to token LIT_NUMBER
    fn LIT_NUMBER(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(LIT_NUMBER, 0)
    }
}

impl ExprIdent_NumLiteralContextAttrs for ExprIdent_NumLiteralContext {}

pub struct ExprIdent_NumLiteralContextExt {
    base: Expr_identContextExt,
}

impl CustomRuleContext for ExprIdent_NumLiteralContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_ident
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprIdent_NumLiteral(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprIdent_NumLiteral(ctx));
    }
}

impl Borrow<Expr_identContextExt> for ExprIdent_NumLiteralContext {
    fn borrow(&self) -> &Expr_identContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_identContextExt> for ExprIdent_NumLiteralContext {
    fn borrow_mut(&mut self) -> &mut Expr_identContextExt {
        &mut self.base
    }
}

impl Expr_identContextAttrs for ExprIdent_NumLiteralContext {}

impl ExprIdent_NumLiteralContextExt {
    fn new(ctx: &dyn Expr_identContextAttrs) -> Rc<Expr_identContextAll> {
        //let base = (cast::<_,Expr_identContext>(&ctx));
        Rc::new(Expr_identContextAll::ExprIdent_NumLiteralContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprIdent_NumLiteralContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type ExprIdent_ParenContext = BaseParserRuleContext<ExprIdent_ParenContextExt>;

pub trait ExprIdent_ParenContextAttrs: ParserRuleContext {
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
}

impl ExprIdent_ParenContextAttrs for ExprIdent_ParenContext {}

pub struct ExprIdent_ParenContextExt {
    base: Expr_identContextExt,
}

impl CustomRuleContext for ExprIdent_ParenContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_ident
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprIdent_Paren(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprIdent_Paren(ctx));
    }
}

impl Borrow<Expr_identContextExt> for ExprIdent_ParenContext {
    fn borrow(&self) -> &Expr_identContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_identContextExt> for ExprIdent_ParenContext {
    fn borrow_mut(&mut self) -> &mut Expr_identContextExt {
        &mut self.base
    }
}

impl Expr_identContextAttrs for ExprIdent_ParenContext {}

impl ExprIdent_ParenContextExt {
    fn new(ctx: &dyn Expr_identContextAttrs) -> Rc<Expr_identContextAll> {
        //let base = (cast::<_,Expr_identContext>(&ctx));
        Rc::new(Expr_identContextAll::ExprIdent_ParenContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprIdent_ParenContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type ExprIdent_IdentContext = BaseParserRuleContext<ExprIdent_IdentContextExt>;

pub trait ExprIdent_IdentContextAttrs: ParserRuleContext {
    /// Retrieves first TerminalNode corresponding to token IDENT
    /// Returns `None` if there is no child corresponding to token IDENT
    fn IDENT(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(IDENT, 0)
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

impl ExprIdent_IdentContextAttrs for ExprIdent_IdentContext {}

pub struct ExprIdent_IdentContextExt {
    base: Expr_identContextExt,
}

impl CustomRuleContext for ExprIdent_IdentContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_ident
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprIdent_Ident(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprIdent_Ident(ctx));
    }
}

impl Borrow<Expr_identContextExt> for ExprIdent_IdentContext {
    fn borrow(&self) -> &Expr_identContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_identContextExt> for ExprIdent_IdentContext {
    fn borrow_mut(&mut self) -> &mut Expr_identContextExt {
        &mut self.base
    }
}

impl Expr_identContextAttrs for ExprIdent_IdentContext {}

impl ExprIdent_IdentContextExt {
    fn new(ctx: &dyn Expr_identContextAttrs) -> Rc<Expr_identContextAll> {
        //let base = (cast::<_,Expr_identContext>(&ctx));
        Rc::new(Expr_identContextAll::ExprIdent_IdentContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprIdent_IdentContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type ExprIdent_StrLiteralContext = BaseParserRuleContext<ExprIdent_StrLiteralContextExt>;

pub trait ExprIdent_StrLiteralContextAttrs: ParserRuleContext {
    /// Retrieves first TerminalNode corresponding to token LIT_STRING
    /// Returns `None` if there is no child corresponding to token LIT_STRING
    fn LIT_STRING(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(LIT_STRING, 0)
    }
}

impl ExprIdent_StrLiteralContextAttrs for ExprIdent_StrLiteralContext {}

pub struct ExprIdent_StrLiteralContextExt {
    base: Expr_identContextExt,
}

impl CustomRuleContext for ExprIdent_StrLiteralContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_ident
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprIdent_StrLiteral(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprIdent_StrLiteral(ctx));
    }
}

impl Borrow<Expr_identContextExt> for ExprIdent_StrLiteralContext {
    fn borrow(&self) -> &Expr_identContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_identContextExt> for ExprIdent_StrLiteralContext {
    fn borrow_mut(&mut self) -> &mut Expr_identContextExt {
        &mut self.base
    }
}

impl Expr_identContextAttrs for ExprIdent_StrLiteralContext {}

impl ExprIdent_StrLiteralContextExt {
    fn new(ctx: &dyn Expr_identContextAttrs) -> Rc<Expr_identContextAll> {
        //let base = (cast::<_,Expr_identContext>(&ctx));
        Rc::new(Expr_identContextAll::ExprIdent_StrLiteralContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprIdent_StrLiteralContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn expr_ident(&mut self) -> Result<Rc<Expr_identContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Expr_identContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 22, RULE_expr_ident);
        let mut _localctx: Rc<Expr_identContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(126);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(5, &mut recog.base)? {
                1 => {
                    let tmp = ExprIdent_IdentContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(116);
                        recog.base.match_token(IDENT, recog.err_handler.as_mut())?;
                    }
                }
                2 => {
                    let tmp = ExprIdent_StrLiteralContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(117);
                        recog
                            .base
                            .match_token(LIT_STRING, recog.err_handler.as_mut())?;
                    }
                }
                3 => {
                    let tmp = ExprIdent_NumLiteralContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        recog.base.set_state(118);
                        recog
                            .base
                            .match_token(LIT_NUMBER, recog.err_handler.as_mut())?;
                    }
                }
                4 => {
                    let tmp = ExprIdent_ParenContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        recog.base.set_state(119);
                        recog
                            .base
                            .match_token(SYM_LPAREN, recog.err_handler.as_mut())?;

                        /*InvokeRule expr*/
                        recog.base.set_state(120);
                        recog.expr()?;

                        recog.base.set_state(121);
                        recog
                            .base
                            .match_token(SYM_RPAREN, recog.err_handler.as_mut())?;
                    }
                }
                5 => {
                    let tmp = ExprIdent_IdentContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 5);
                    _localctx = tmp;
                    {
                        /*InvokeRule if_expr*/
                        recog.base.set_state(123);
                        recog.if_expr()?;
                    }
                }
                6 => {
                    let tmp = ExprIdent_IdentContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 6);
                    _localctx = tmp;
                    {
                        /*InvokeRule while_expr*/
                        recog.base.set_state(124);
                        recog.while_expr()?;
                    }
                }
                7 => {
                    let tmp = ExprIdent_IdentContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 7);
                    _localctx = tmp;
                    {
                        /*InvokeRule fn_call*/
                        recog.base.set_state(125);
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
#[derive(Debug)]
pub enum Expr_unaryContextAll {
    ExprUnary_MinusContext(ExprUnary_MinusContext),
    ExprUnary_Ident_Context(ExprUnary_Ident_Context),
    Error(Expr_unaryContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Expr_unaryContextAll {}

impl Deref for Expr_unaryContextAll {
    type Target = dyn Expr_unaryContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Expr_unaryContextAll::*;
        match self {
            ExprUnary_MinusContext(inner) => inner,
            ExprUnary_Ident_Context(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Expr_unaryContext = BaseParserRuleContext<Expr_unaryContextExt>;

#[derive(Clone)]
pub struct Expr_unaryContextExt {}
impl CustomRuleContext for Expr_unaryContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_unary
    }
}

impl Expr_unaryContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Expr_unaryContextAll> {
        Rc::new(Expr_unaryContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Expr_unaryContextExt {}),
        ))
    }
}

pub trait Expr_unaryContextAttrs: ParserRuleContext + BorrowMut<Expr_unaryContextExt> {}

impl Expr_unaryContextAttrs for Expr_unaryContext {}

//impl Expr_unaryContext{

//public Expr_unaryContext() { }
//pub fn copy_into(&self, ctx: Expr_unaryContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type ExprUnary_MinusContext = BaseParserRuleContext<ExprUnary_MinusContextExt>;

pub trait ExprUnary_MinusContextAttrs: ParserRuleContext {
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

impl ExprUnary_MinusContextAttrs for ExprUnary_MinusContext {}

pub struct ExprUnary_MinusContextExt {
    base: Expr_unaryContextExt,
}

impl CustomRuleContext for ExprUnary_MinusContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_unary
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprUnary_Minus(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprUnary_Minus(ctx));
    }
}

impl Borrow<Expr_unaryContextExt> for ExprUnary_MinusContext {
    fn borrow(&self) -> &Expr_unaryContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_unaryContextExt> for ExprUnary_MinusContext {
    fn borrow_mut(&mut self) -> &mut Expr_unaryContextExt {
        &mut self.base
    }
}

impl Expr_unaryContextAttrs for ExprUnary_MinusContext {}

impl ExprUnary_MinusContextExt {
    fn new(ctx: &dyn Expr_unaryContextAttrs) -> Rc<Expr_unaryContextAll> {
        //let base = (cast::<_,Expr_unaryContext>(&ctx));
        Rc::new(Expr_unaryContextAll::ExprUnary_MinusContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprUnary_MinusContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type ExprUnary_Ident_Context = BaseParserRuleContext<ExprUnary_Ident_ContextExt>;

pub trait ExprUnary_Ident_ContextAttrs: ParserRuleContext {
    fn expr_ident(&self) -> Option<Rc<Expr_identContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprUnary_Ident_ContextAttrs for ExprUnary_Ident_Context {}

pub struct ExprUnary_Ident_ContextExt {
    base: Expr_unaryContextExt,
}

impl CustomRuleContext for ExprUnary_Ident_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_unary
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprUnary_Ident_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprUnary_Ident_(ctx));
    }
}

impl Borrow<Expr_unaryContextExt> for ExprUnary_Ident_Context {
    fn borrow(&self) -> &Expr_unaryContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_unaryContextExt> for ExprUnary_Ident_Context {
    fn borrow_mut(&mut self) -> &mut Expr_unaryContextExt {
        &mut self.base
    }
}

impl Expr_unaryContextAttrs for ExprUnary_Ident_Context {}

impl ExprUnary_Ident_ContextExt {
    fn new(ctx: &dyn Expr_unaryContextAttrs) -> Rc<Expr_unaryContextAll> {
        //let base = (cast::<_,Expr_unaryContext>(&ctx));
        Rc::new(Expr_unaryContextAll::ExprUnary_Ident_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprUnary_Ident_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn expr_unary(&mut self) -> Result<Rc<Expr_unaryContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Expr_unaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 24, RULE_expr_unary);
        let mut _localctx: Rc<Expr_unaryContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(131);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                SYM_SUB => {
                    let tmp = ExprUnary_MinusContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(128);
                        recog
                            .base
                            .match_token(SYM_SUB, recog.err_handler.as_mut())?;

                        /*InvokeRule expr_ident*/
                        recog.base.set_state(129);
                        recog.expr_ident()?;
                    }
                }

                IF | WHILE | SYM_LPAREN | IDENT | LIT_STRING | LIT_NUMBER => {
                    let tmp = ExprUnary_Ident_ContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        /*InvokeRule expr_ident*/
                        recog.base.set_state(130);
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
#[derive(Debug)]
pub enum Expr_mulContextAll {
    ExprMul_Unary_Context(ExprMul_Unary_Context),
    ExprMul_MulContext(ExprMul_MulContext),
    ExprMul_DivContext(ExprMul_DivContext),
    ExprMul_ModContext(ExprMul_ModContext),
    Error(Expr_mulContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Expr_mulContextAll {}

impl Deref for Expr_mulContextAll {
    type Target = dyn Expr_mulContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Expr_mulContextAll::*;
        match self {
            ExprMul_Unary_Context(inner) => inner,
            ExprMul_MulContext(inner) => inner,
            ExprMul_DivContext(inner) => inner,
            ExprMul_ModContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Expr_mulContext = BaseParserRuleContext<Expr_mulContextExt>;

#[derive(Clone)]
pub struct Expr_mulContextExt {}
impl CustomRuleContext for Expr_mulContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_mul
    }
}

impl Expr_mulContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Expr_mulContextAll> {
        Rc::new(Expr_mulContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Expr_mulContextExt {}),
        ))
    }
}

pub trait Expr_mulContextAttrs: ParserRuleContext + BorrowMut<Expr_mulContextExt> {}

impl Expr_mulContextAttrs for Expr_mulContext {}

//impl Expr_mulContext{

//public Expr_mulContext() { }
//pub fn copy_into(&self, ctx: Expr_mulContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type ExprMul_Unary_Context = BaseParserRuleContext<ExprMul_Unary_ContextExt>;

pub trait ExprMul_Unary_ContextAttrs: ParserRuleContext {
    fn expr_unary(&self) -> Option<Rc<Expr_unaryContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprMul_Unary_ContextAttrs for ExprMul_Unary_Context {}

pub struct ExprMul_Unary_ContextExt {
    base: Expr_mulContextExt,
}

impl CustomRuleContext for ExprMul_Unary_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_mul
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprMul_Unary_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprMul_Unary_(ctx));
    }
}

impl Borrow<Expr_mulContextExt> for ExprMul_Unary_Context {
    fn borrow(&self) -> &Expr_mulContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_mulContextExt> for ExprMul_Unary_Context {
    fn borrow_mut(&mut self) -> &mut Expr_mulContextExt {
        &mut self.base
    }
}

impl Expr_mulContextAttrs for ExprMul_Unary_Context {}

impl ExprMul_Unary_ContextExt {
    fn new(ctx: &dyn Expr_mulContextAttrs) -> Rc<Expr_mulContextAll> {
        //let base = (cast::<_,Expr_mulContext>(&ctx));
        Rc::new(Expr_mulContextAll::ExprMul_Unary_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprMul_Unary_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type ExprMul_MulContext = BaseParserRuleContext<ExprMul_MulContextExt>;

pub trait ExprMul_MulContextAttrs: ParserRuleContext {
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
    fn expr_unary(&self) -> Option<Rc<Expr_unaryContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprMul_MulContextAttrs for ExprMul_MulContext {}

pub struct ExprMul_MulContextExt {
    base: Expr_mulContextExt,
}

impl CustomRuleContext for ExprMul_MulContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_mul
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprMul_Mul(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprMul_Mul(ctx));
    }
}

impl Borrow<Expr_mulContextExt> for ExprMul_MulContext {
    fn borrow(&self) -> &Expr_mulContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_mulContextExt> for ExprMul_MulContext {
    fn borrow_mut(&mut self) -> &mut Expr_mulContextExt {
        &mut self.base
    }
}

impl Expr_mulContextAttrs for ExprMul_MulContext {}

impl ExprMul_MulContextExt {
    fn new(ctx: &dyn Expr_mulContextAttrs) -> Rc<Expr_mulContextAll> {
        //let base = (cast::<_,Expr_mulContext>(&ctx));
        Rc::new(Expr_mulContextAll::ExprMul_MulContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprMul_MulContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type ExprMul_DivContext = BaseParserRuleContext<ExprMul_DivContextExt>;

pub trait ExprMul_DivContextAttrs: ParserRuleContext {
    fn expr_mul(&self) -> Option<Rc<Expr_mulContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_DIV
    /// Returns `None` if there is no child corresponding to token SYM_DIV
    fn SYM_DIV(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_DIV, 0)
    }
    fn expr_unary(&self) -> Option<Rc<Expr_unaryContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprMul_DivContextAttrs for ExprMul_DivContext {}

pub struct ExprMul_DivContextExt {
    base: Expr_mulContextExt,
}

impl CustomRuleContext for ExprMul_DivContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_mul
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprMul_Div(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprMul_Div(ctx));
    }
}

impl Borrow<Expr_mulContextExt> for ExprMul_DivContext {
    fn borrow(&self) -> &Expr_mulContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_mulContextExt> for ExprMul_DivContext {
    fn borrow_mut(&mut self) -> &mut Expr_mulContextExt {
        &mut self.base
    }
}

impl Expr_mulContextAttrs for ExprMul_DivContext {}

impl ExprMul_DivContextExt {
    fn new(ctx: &dyn Expr_mulContextAttrs) -> Rc<Expr_mulContextAll> {
        //let base = (cast::<_,Expr_mulContext>(&ctx));
        Rc::new(Expr_mulContextAll::ExprMul_DivContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprMul_DivContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type ExprMul_ModContext = BaseParserRuleContext<ExprMul_ModContextExt>;

pub trait ExprMul_ModContextAttrs: ParserRuleContext {
    fn expr_mul(&self) -> Option<Rc<Expr_mulContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_MOD
    /// Returns `None` if there is no child corresponding to token SYM_MOD
    fn SYM_MOD(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_MOD, 0)
    }
    fn expr_unary(&self) -> Option<Rc<Expr_unaryContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprMul_ModContextAttrs for ExprMul_ModContext {}

pub struct ExprMul_ModContextExt {
    base: Expr_mulContextExt,
}

impl CustomRuleContext for ExprMul_ModContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_mul
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprMul_Mod(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprMul_Mod(ctx));
    }
}

impl Borrow<Expr_mulContextExt> for ExprMul_ModContext {
    fn borrow(&self) -> &Expr_mulContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_mulContextExt> for ExprMul_ModContext {
    fn borrow_mut(&mut self) -> &mut Expr_mulContextExt {
        &mut self.base
    }
}

impl Expr_mulContextAttrs for ExprMul_ModContext {}

impl ExprMul_ModContextExt {
    fn new(ctx: &dyn Expr_mulContextAttrs) -> Rc<Expr_mulContextAll> {
        //let base = (cast::<_,Expr_mulContext>(&ctx));
        Rc::new(Expr_mulContextAll::ExprMul_ModContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprMul_ModContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

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
            .enter_recursion_rule(_localctx.clone(), 26, RULE_expr_mul, _p);
        let mut _localctx: Rc<Expr_mulContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 26;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    let mut tmp = ExprMul_Unary_ContextExt::new(&**_localctx);
                    recog.ctx = Some(tmp.clone());
                    _localctx = tmp;
                    _prevctx = _localctx.clone();

                    /*InvokeRule expr_unary*/
                    recog.base.set_state(134);
                    recog.expr_unary()?;
                }
                let tmp = recog.input.lt(-1).map(Token::to_owned);
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(147);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(8, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(145);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(7, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            ExprMul_MulContextExt::new(&**Expr_mulContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr_mul,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(136);
                                        if !({ recog.precpred(None, 3) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 3)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(137);
                                        recog
                                            .base
                                            .match_token(SYM_MUL, recog.err_handler.as_mut())?;

                                        /*InvokeRule expr_unary*/
                                        recog.base.set_state(138);
                                        recog.expr_unary()?;
                                    }
                                }
                                2 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            ExprMul_DivContextExt::new(&**Expr_mulContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr_mul,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(139);
                                        if !({ recog.precpred(None, 2) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 2)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(140);
                                        recog
                                            .base
                                            .match_token(SYM_DIV, recog.err_handler.as_mut())?;

                                        /*InvokeRule expr_unary*/
                                        recog.base.set_state(141);
                                        recog.expr_unary()?;
                                    }
                                }
                                3 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            ExprMul_ModContextExt::new(&**Expr_mulContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr_mul,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(142);
                                        if !({ recog.precpred(None, 1) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 1)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(143);
                                        recog
                                            .base
                                            .match_token(SYM_MOD, recog.err_handler.as_mut())?;

                                        /*InvokeRule expr_unary*/
                                        recog.base.set_state(144);
                                        recog.expr_unary()?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(149);
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
#[derive(Debug)]
pub enum Expr_addContextAll {
    ExprAdd_AddContext(ExprAdd_AddContext),
    ExprAdd_Mul_Context(ExprAdd_Mul_Context),
    ExprAdd_SubContext(ExprAdd_SubContext),
    Error(Expr_addContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Expr_addContextAll {}

impl Deref for Expr_addContextAll {
    type Target = dyn Expr_addContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Expr_addContextAll::*;
        match self {
            ExprAdd_AddContext(inner) => inner,
            ExprAdd_Mul_Context(inner) => inner,
            ExprAdd_SubContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Expr_addContext = BaseParserRuleContext<Expr_addContextExt>;

#[derive(Clone)]
pub struct Expr_addContextExt {}
impl CustomRuleContext for Expr_addContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_add
    }
}

impl Expr_addContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Expr_addContextAll> {
        Rc::new(Expr_addContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Expr_addContextExt {}),
        ))
    }
}

pub trait Expr_addContextAttrs: ParserRuleContext + BorrowMut<Expr_addContextExt> {}

impl Expr_addContextAttrs for Expr_addContext {}

//impl Expr_addContext{

//public Expr_addContext() { }
//pub fn copy_into(&self, ctx: Expr_addContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type ExprAdd_AddContext = BaseParserRuleContext<ExprAdd_AddContextExt>;

pub trait ExprAdd_AddContextAttrs: ParserRuleContext {
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
    fn expr_mul(&self) -> Option<Rc<Expr_mulContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprAdd_AddContextAttrs for ExprAdd_AddContext {}

pub struct ExprAdd_AddContextExt {
    base: Expr_addContextExt,
}

impl CustomRuleContext for ExprAdd_AddContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_add
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprAdd_Add(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprAdd_Add(ctx));
    }
}

impl Borrow<Expr_addContextExt> for ExprAdd_AddContext {
    fn borrow(&self) -> &Expr_addContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_addContextExt> for ExprAdd_AddContext {
    fn borrow_mut(&mut self) -> &mut Expr_addContextExt {
        &mut self.base
    }
}

impl Expr_addContextAttrs for ExprAdd_AddContext {}

impl ExprAdd_AddContextExt {
    fn new(ctx: &dyn Expr_addContextAttrs) -> Rc<Expr_addContextAll> {
        //let base = (cast::<_,Expr_addContext>(&ctx));
        Rc::new(Expr_addContextAll::ExprAdd_AddContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprAdd_AddContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type ExprAdd_Mul_Context = BaseParserRuleContext<ExprAdd_Mul_ContextExt>;

pub trait ExprAdd_Mul_ContextAttrs: ParserRuleContext {
    fn expr_mul(&self) -> Option<Rc<Expr_mulContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprAdd_Mul_ContextAttrs for ExprAdd_Mul_Context {}

pub struct ExprAdd_Mul_ContextExt {
    base: Expr_addContextExt,
}

impl CustomRuleContext for ExprAdd_Mul_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_add
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprAdd_Mul_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprAdd_Mul_(ctx));
    }
}

impl Borrow<Expr_addContextExt> for ExprAdd_Mul_Context {
    fn borrow(&self) -> &Expr_addContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_addContextExt> for ExprAdd_Mul_Context {
    fn borrow_mut(&mut self) -> &mut Expr_addContextExt {
        &mut self.base
    }
}

impl Expr_addContextAttrs for ExprAdd_Mul_Context {}

impl ExprAdd_Mul_ContextExt {
    fn new(ctx: &dyn Expr_addContextAttrs) -> Rc<Expr_addContextAll> {
        //let base = (cast::<_,Expr_addContext>(&ctx));
        Rc::new(Expr_addContextAll::ExprAdd_Mul_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprAdd_Mul_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type ExprAdd_SubContext = BaseParserRuleContext<ExprAdd_SubContextExt>;

pub trait ExprAdd_SubContextAttrs: ParserRuleContext {
    fn expr_add(&self) -> Option<Rc<Expr_addContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SYM_SUB
    /// Returns `None` if there is no child corresponding to token SYM_SUB
    fn SYM_SUB(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_SUB, 0)
    }
    fn expr_mul(&self) -> Option<Rc<Expr_mulContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprAdd_SubContextAttrs for ExprAdd_SubContext {}

pub struct ExprAdd_SubContextExt {
    base: Expr_addContextExt,
}

impl CustomRuleContext for ExprAdd_SubContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_add
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprAdd_Sub(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprAdd_Sub(ctx));
    }
}

impl Borrow<Expr_addContextExt> for ExprAdd_SubContext {
    fn borrow(&self) -> &Expr_addContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_addContextExt> for ExprAdd_SubContext {
    fn borrow_mut(&mut self) -> &mut Expr_addContextExt {
        &mut self.base
    }
}

impl Expr_addContextAttrs for ExprAdd_SubContext {}

impl ExprAdd_SubContextExt {
    fn new(ctx: &dyn Expr_addContextAttrs) -> Rc<Expr_addContextAll> {
        //let base = (cast::<_,Expr_addContext>(&ctx));
        Rc::new(Expr_addContextAll::ExprAdd_SubContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprAdd_SubContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

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
            .enter_recursion_rule(_localctx.clone(), 28, RULE_expr_add, _p);
        let mut _localctx: Rc<Expr_addContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 28;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    let mut tmp = ExprAdd_Mul_ContextExt::new(&**_localctx);
                    recog.ctx = Some(tmp.clone());
                    _localctx = tmp;
                    _prevctx = _localctx.clone();

                    /*InvokeRule expr_mul*/
                    recog.base.set_state(151);
                    recog.expr_mul_rec(0)?;
                }
                let tmp = recog.input.lt(-1).map(Token::to_owned);
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(161);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(10, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(159);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(9, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            ExprAdd_AddContextExt::new(&**Expr_addContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr_add,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(153);
                                        if !({ recog.precpred(None, 2) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 2)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(154);
                                        recog
                                            .base
                                            .match_token(SYM_ADD, recog.err_handler.as_mut())?;

                                        /*InvokeRule expr_mul*/
                                        recog.base.set_state(155);
                                        recog.expr_mul_rec(0)?;
                                    }
                                }
                                2 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            ExprAdd_SubContextExt::new(&**Expr_addContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expr_add,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(156);
                                        if !({ recog.precpred(None, 1) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 1)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(157);
                                        recog
                                            .base
                                            .match_token(SYM_SUB, recog.err_handler.as_mut())?;

                                        /*InvokeRule expr_mul*/
                                        recog.base.set_state(158);
                                        recog.expr_mul_rec(0)?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(163);
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
#[derive(Debug)]
pub enum Expr_andContextAll {
    ExprAnd_Add_Context(ExprAnd_Add_Context),
    ExprAnd_AndContext(ExprAnd_AndContext),
    Error(Expr_andContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Expr_andContextAll {}

impl Deref for Expr_andContextAll {
    type Target = dyn Expr_andContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Expr_andContextAll::*;
        match self {
            ExprAnd_Add_Context(inner) => inner,
            ExprAnd_AndContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Expr_andContext = BaseParserRuleContext<Expr_andContextExt>;

#[derive(Clone)]
pub struct Expr_andContextExt {}
impl CustomRuleContext for Expr_andContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_and
    }
}

impl Expr_andContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Expr_andContextAll> {
        Rc::new(Expr_andContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Expr_andContextExt {}),
        ))
    }
}

pub trait Expr_andContextAttrs: ParserRuleContext + BorrowMut<Expr_andContextExt> {}

impl Expr_andContextAttrs for Expr_andContext {}

//impl Expr_andContext{

//public Expr_andContext() { }
//pub fn copy_into(&self, ctx: Expr_andContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type ExprAnd_Add_Context = BaseParserRuleContext<ExprAnd_Add_ContextExt>;

pub trait ExprAnd_Add_ContextAttrs: ParserRuleContext {
    fn expr_add(&self) -> Option<Rc<Expr_addContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprAnd_Add_ContextAttrs for ExprAnd_Add_Context {}

pub struct ExprAnd_Add_ContextExt {
    base: Expr_andContextExt,
}

impl CustomRuleContext for ExprAnd_Add_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_and
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprAnd_Add_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprAnd_Add_(ctx));
    }
}

impl Borrow<Expr_andContextExt> for ExprAnd_Add_Context {
    fn borrow(&self) -> &Expr_andContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_andContextExt> for ExprAnd_Add_Context {
    fn borrow_mut(&mut self) -> &mut Expr_andContextExt {
        &mut self.base
    }
}

impl Expr_andContextAttrs for ExprAnd_Add_Context {}

impl ExprAnd_Add_ContextExt {
    fn new(ctx: &dyn Expr_andContextAttrs) -> Rc<Expr_andContextAll> {
        //let base = (cast::<_,Expr_andContext>(&ctx));
        Rc::new(Expr_andContextAll::ExprAnd_Add_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprAnd_Add_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type ExprAnd_AndContext = BaseParserRuleContext<ExprAnd_AndContextExt>;

pub trait ExprAnd_AndContextAttrs: ParserRuleContext {
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
    fn expr_add(&self) -> Option<Rc<Expr_addContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprAnd_AndContextAttrs for ExprAnd_AndContext {}

pub struct ExprAnd_AndContextExt {
    base: Expr_andContextExt,
}

impl CustomRuleContext for ExprAnd_AndContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_and
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprAnd_And(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprAnd_And(ctx));
    }
}

impl Borrow<Expr_andContextExt> for ExprAnd_AndContext {
    fn borrow(&self) -> &Expr_andContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_andContextExt> for ExprAnd_AndContext {
    fn borrow_mut(&mut self) -> &mut Expr_andContextExt {
        &mut self.base
    }
}

impl Expr_andContextAttrs for ExprAnd_AndContext {}

impl ExprAnd_AndContextExt {
    fn new(ctx: &dyn Expr_andContextAttrs) -> Rc<Expr_andContextAll> {
        //let base = (cast::<_,Expr_andContext>(&ctx));
        Rc::new(Expr_andContextAll::ExprAnd_AndContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprAnd_AndContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

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
            .enter_recursion_rule(_localctx.clone(), 30, RULE_expr_and, _p);
        let mut _localctx: Rc<Expr_andContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 30;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    let mut tmp = ExprAnd_Add_ContextExt::new(&**_localctx);
                    recog.ctx = Some(tmp.clone());
                    _localctx = tmp;
                    _prevctx = _localctx.clone();

                    /*InvokeRule expr_add*/
                    recog.base.set_state(165);
                    recog.expr_add_rec(0)?;
                }
                let tmp = recog.input.lt(-1).map(Token::to_owned);
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(172);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(11, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            {
                                /*recRuleLabeledAltStartAction*/
                                let mut tmp = ExprAnd_AndContextExt::new(
                                    &**Expr_andContextExt::new(_parentctx.clone(), _parentState),
                                );
                                recog.push_new_recursion_context(
                                    tmp.clone(),
                                    _startState,
                                    RULE_expr_and,
                                );
                                _localctx = tmp;
                                recog.base.set_state(167);
                                if !({ recog.precpred(None, 1) }) {
                                    Err(FailedPredicateError::new(
                                        &mut recog.base,
                                        Some("recog.precpred(None, 1)".to_owned()),
                                        None,
                                    ))?;
                                }
                                recog.base.set_state(168);
                                recog
                                    .base
                                    .match_token(SYM_AND, recog.err_handler.as_mut())?;

                                /*InvokeRule expr_add*/
                                recog.base.set_state(169);
                                recog.expr_add_rec(0)?;
                            }
                        }
                    }
                    recog.base.set_state(174);
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
#[derive(Debug)]
pub enum Expr_orContextAll {
    ExprOr_OrContext(ExprOr_OrContext),
    ExprOr_And_Context(ExprOr_And_Context),
    Error(Expr_orContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Expr_orContextAll {}

impl Deref for Expr_orContextAll {
    type Target = dyn Expr_orContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Expr_orContextAll::*;
        match self {
            ExprOr_OrContext(inner) => inner,
            ExprOr_And_Context(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Expr_orContext = BaseParserRuleContext<Expr_orContextExt>;

#[derive(Clone)]
pub struct Expr_orContextExt {}
impl CustomRuleContext for Expr_orContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_or
    }
}

impl Expr_orContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<Expr_orContextAll> {
        Rc::new(Expr_orContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Expr_orContextExt {}),
        ))
    }
}

pub trait Expr_orContextAttrs: ParserRuleContext + BorrowMut<Expr_orContextExt> {}

impl Expr_orContextAttrs for Expr_orContext {}

//impl Expr_orContext{

//public Expr_orContext() { }
//pub fn copy_into(&self, ctx: Expr_orContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type ExprOr_OrContext = BaseParserRuleContext<ExprOr_OrContextExt>;

pub trait ExprOr_OrContextAttrs: ParserRuleContext {
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
    fn expr_and(&self) -> Option<Rc<Expr_andContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprOr_OrContextAttrs for ExprOr_OrContext {}

pub struct ExprOr_OrContextExt {
    base: Expr_orContextExt,
}

impl CustomRuleContext for ExprOr_OrContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_or
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprOr_Or(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprOr_Or(ctx));
    }
}

impl Borrow<Expr_orContextExt> for ExprOr_OrContext {
    fn borrow(&self) -> &Expr_orContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_orContextExt> for ExprOr_OrContext {
    fn borrow_mut(&mut self) -> &mut Expr_orContextExt {
        &mut self.base
    }
}

impl Expr_orContextAttrs for ExprOr_OrContext {}

impl ExprOr_OrContextExt {
    fn new(ctx: &dyn Expr_orContextAttrs) -> Rc<Expr_orContextAll> {
        //let base = (cast::<_,Expr_orContext>(&ctx));
        Rc::new(Expr_orContextAll::ExprOr_OrContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprOr_OrContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type ExprOr_And_Context = BaseParserRuleContext<ExprOr_And_ContextExt>;

pub trait ExprOr_And_ContextAttrs: ParserRuleContext {
    fn expr_and(&self) -> Option<Rc<Expr_andContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ExprOr_And_ContextAttrs for ExprOr_And_Context {}

pub struct ExprOr_And_ContextExt {
    base: Expr_orContextExt,
}

impl CustomRuleContext for ExprOr_And_ContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_expr_or
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ExprOr_And_(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ExprOr_And_(ctx));
    }
}

impl Borrow<Expr_orContextExt> for ExprOr_And_Context {
    fn borrow(&self) -> &Expr_orContextExt {
        &self.base
    }
}
impl BorrowMut<Expr_orContextExt> for ExprOr_And_Context {
    fn borrow_mut(&mut self) -> &mut Expr_orContextExt {
        &mut self.base
    }
}

impl Expr_orContextAttrs for ExprOr_And_Context {}

impl ExprOr_And_ContextExt {
    fn new(ctx: &dyn Expr_orContextAttrs) -> Rc<Expr_orContextAll> {
        //let base = (cast::<_,Expr_orContext>(&ctx));
        Rc::new(Expr_orContextAll::ExprOr_And_Context(
            BaseParserRuleContext::copy_from(
                ctx,
                ExprOr_And_ContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

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
            .enter_recursion_rule(_localctx.clone(), 32, RULE_expr_or, _p);
        let mut _localctx: Rc<Expr_orContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 32;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    let mut tmp = ExprOr_And_ContextExt::new(&**_localctx);
                    recog.ctx = Some(tmp.clone());
                    _localctx = tmp;
                    _prevctx = _localctx.clone();

                    /*InvokeRule expr_and*/
                    recog.base.set_state(176);
                    recog.expr_and_rec(0)?;
                }
                let tmp = recog.input.lt(-1).map(Token::to_owned);
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(183);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(12, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            {
                                /*recRuleLabeledAltStartAction*/
                                let mut tmp = ExprOr_OrContextExt::new(&**Expr_orContextExt::new(
                                    _parentctx.clone(),
                                    _parentState,
                                ));
                                recog.push_new_recursion_context(
                                    tmp.clone(),
                                    _startState,
                                    RULE_expr_or,
                                );
                                _localctx = tmp;
                                recog.base.set_state(178);
                                if !({ recog.precpred(None, 1) }) {
                                    Err(FailedPredicateError::new(
                                        &mut recog.base,
                                        Some("recog.precpred(None, 1)".to_owned()),
                                        None,
                                    ))?;
                                }
                                recog.base.set_state(179);
                                recog.base.match_token(SYM_OR, recog.err_handler.as_mut())?;

                                /*InvokeRule expr_and*/
                                recog.base.set_state(180);
                                recog.expr_and_rec(0)?;
                            }
                        }
                    }
                    recog.base.set_state(185);
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
#[derive(Debug)]
pub enum BlockContextAll {
    Block_NonReturnContext(Block_NonReturnContext),
    Block_WithReturnContext(Block_WithReturnContext),
    Error(BlockContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for BlockContextAll {}

impl Deref for BlockContextAll {
    type Target = dyn BlockContextAttrs;
    fn deref(&self) -> &Self::Target {
        use BlockContextAll::*;
        match self {
            Block_NonReturnContext(inner) => inner,
            Block_WithReturnContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type BlockContext = BaseParserRuleContext<BlockContextExt>;

#[derive(Clone)]
pub struct BlockContextExt {}
impl CustomRuleContext for BlockContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_block
    }
}

impl BlockContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<BlockContextAll> {
        Rc::new(BlockContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, BlockContextExt {}),
        ))
    }
}

pub trait BlockContextAttrs: ParserRuleContext + BorrowMut<BlockContextExt> {}

impl BlockContextAttrs for BlockContext {}

//impl BlockContext{

//public BlockContext() { }
//pub fn copy_into(&self, ctx: BlockContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type Block_NonReturnContext = BaseParserRuleContext<Block_NonReturnContextExt>;

pub trait Block_NonReturnContextAttrs: ParserRuleContext {
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
}

impl Block_NonReturnContextAttrs for Block_NonReturnContext {}

pub struct Block_NonReturnContextExt {
    base: BlockContextExt,
}

impl CustomRuleContext for Block_NonReturnContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_block
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_Block_NonReturn(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_Block_NonReturn(ctx));
    }
}

impl Borrow<BlockContextExt> for Block_NonReturnContext {
    fn borrow(&self) -> &BlockContextExt {
        &self.base
    }
}
impl BorrowMut<BlockContextExt> for Block_NonReturnContext {
    fn borrow_mut(&mut self) -> &mut BlockContextExt {
        &mut self.base
    }
}

impl BlockContextAttrs for Block_NonReturnContext {}

impl Block_NonReturnContextExt {
    fn new(ctx: &dyn BlockContextAttrs) -> Rc<BlockContextAll> {
        //let base = (cast::<_,BlockContext>(&ctx));
        Rc::new(BlockContextAll::Block_NonReturnContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Block_NonReturnContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

pub type Block_WithReturnContext = BaseParserRuleContext<Block_WithReturnContextExt>;

pub trait Block_WithReturnContextAttrs: ParserRuleContext {
    /// Retrieves first TerminalNode corresponding to token SYM_LCURLY
    /// Returns `None` if there is no child corresponding to token SYM_LCURLY
    fn SYM_LCURLY(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(SYM_LCURLY, 0)
    }
    fn expr(&self) -> Option<Rc<ExprContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
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
}

impl Block_WithReturnContextAttrs for Block_WithReturnContext {}

pub struct Block_WithReturnContextExt {
    base: BlockContextExt,
}

impl CustomRuleContext for Block_WithReturnContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_block
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_Block_WithReturn(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_Block_WithReturn(ctx));
    }
}

impl Borrow<BlockContextExt> for Block_WithReturnContext {
    fn borrow(&self) -> &BlockContextExt {
        &self.base
    }
}
impl BorrowMut<BlockContextExt> for Block_WithReturnContext {
    fn borrow_mut(&mut self) -> &mut BlockContextExt {
        &mut self.base
    }
}

impl BlockContextAttrs for Block_WithReturnContext {}

impl Block_WithReturnContextExt {
    fn new(ctx: &dyn BlockContextAttrs) -> Rc<BlockContextAll> {
        //let base = (cast::<_,BlockContext>(&ctx));
        Rc::new(BlockContextAll::Block_WithReturnContext(
            BaseParserRuleContext::copy_from(
                ctx,
                Block_WithReturnContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn block(&mut self) -> Result<Rc<BlockContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = BlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            recog.base.set_state(204);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(15, &mut recog.base)? {
                1 => {
                    let tmp = Block_NonReturnContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(186);
                        recog
                            .base
                            .match_token(SYM_LCURLY, recog.err_handler.as_mut())?;

                        recog.base.set_state(190);
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
                                    recog.base.set_state(187);
                                    recog.stmt()?;
                                }
                            }
                            recog.base.set_state(192);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(193);
                        recog
                            .base
                            .match_token(SYM_RCURLY, recog.err_handler.as_mut())?;
                    }
                }
                2 => {
                    let tmp = Block_WithReturnContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(194);
                        recog
                            .base
                            .match_token(SYM_LCURLY, recog.err_handler.as_mut())?;

                        recog.base.set_state(198);
                        recog.err_handler.sync(&mut recog.base)?;
                        _alt = recog.interpreter.adaptive_predict(14, &mut recog.base)?;
                        while { _alt != 2 && _alt != INVALID_ALT } {
                            if _alt == 1 {
                                {
                                    {
                                        /*InvokeRule stmt*/
                                        recog.base.set_state(195);
                                        recog.stmt()?;
                                    }
                                }
                            }
                            recog.base.set_state(200);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(14, &mut recog.base)?;
                        }
                        /*InvokeRule expr*/
                        recog.base.set_state(201);
                        recog.expr()?;

                        recog.base.set_state(202);
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
#[derive(Debug)]
pub enum If_exprContextAll {
    IfExprContext(IfExprContext),
    Error(If_exprContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for If_exprContextAll {}

impl Deref for If_exprContextAll {
    type Target = dyn If_exprContextAttrs;
    fn deref(&self) -> &Self::Target {
        use If_exprContextAll::*;
        match self {
            IfExprContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type If_exprContext = BaseParserRuleContext<If_exprContextExt>;

#[derive(Clone)]
pub struct If_exprContextExt {}
impl CustomRuleContext for If_exprContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_if_expr
    }
}

impl If_exprContextExt {
    fn new(parent: Option<ParserRuleContextType>, invoking_state: isize) -> Rc<If_exprContextAll> {
        Rc::new(If_exprContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, If_exprContextExt {}),
        ))
    }
}

pub trait If_exprContextAttrs: ParserRuleContext + BorrowMut<If_exprContextExt> {}

impl If_exprContextAttrs for If_exprContext {}

//impl If_exprContext{

//public If_exprContext() { }
//pub fn copy_into(&self, ctx: If_exprContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type IfExprContext = BaseParserRuleContext<IfExprContextExt>;

pub trait IfExprContextAttrs: ParserRuleContext {
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

impl IfExprContextAttrs for IfExprContext {}

pub struct IfExprContextExt {
    base: If_exprContextExt,
}

impl CustomRuleContext for IfExprContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_if_expr
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_IfExpr(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_IfExpr(ctx));
    }
}

impl Borrow<If_exprContextExt> for IfExprContext {
    fn borrow(&self) -> &If_exprContextExt {
        &self.base
    }
}
impl BorrowMut<If_exprContextExt> for IfExprContext {
    fn borrow_mut(&mut self) -> &mut If_exprContextExt {
        &mut self.base
    }
}

impl If_exprContextAttrs for IfExprContext {}

impl IfExprContextExt {
    fn new(ctx: &dyn If_exprContextAttrs) -> Rc<If_exprContextAll> {
        //let base = (cast::<_,If_exprContext>(&ctx));
        Rc::new(If_exprContextAll::IfExprContext(
            BaseParserRuleContext::copy_from(
                ctx,
                IfExprContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn if_expr(&mut self) -> Result<Rc<If_exprContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = If_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_if_expr);
        let mut _localctx: Rc<If_exprContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let tmp = IfExprContextExt::new(&**_localctx);
            recog.base.enter_outer_alt(Some(tmp.clone()), 1);
            _localctx = tmp;
            {
                recog.base.set_state(206);
                recog.base.match_token(IF, recog.err_handler.as_mut())?;

                recog.base.set_state(207);
                recog
                    .base
                    .match_token(SYM_LPAREN, recog.err_handler.as_mut())?;

                /*InvokeRule expr*/
                recog.base.set_state(208);
                recog.expr()?;

                recog.base.set_state(209);
                recog
                    .base
                    .match_token(SYM_RPAREN, recog.err_handler.as_mut())?;

                /*InvokeRule block*/
                recog.base.set_state(210);
                recog.block()?;

                recog.base.set_state(212);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(16, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule else_stmt*/
                            recog.base.set_state(211);
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
#[derive(Debug)]
pub enum Else_stmtContextAll {
    ElseStmtContext(ElseStmtContext),
    Error(Else_stmtContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for Else_stmtContextAll {}

impl Deref for Else_stmtContextAll {
    type Target = dyn Else_stmtContextAttrs;
    fn deref(&self) -> &Self::Target {
        use Else_stmtContextAll::*;
        match self {
            ElseStmtContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type Else_stmtContext = BaseParserRuleContext<Else_stmtContextExt>;

#[derive(Clone)]
pub struct Else_stmtContextExt {}
impl CustomRuleContext for Else_stmtContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_else_stmt
    }
}

impl Else_stmtContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<Else_stmtContextAll> {
        Rc::new(Else_stmtContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, Else_stmtContextExt {}),
        ))
    }
}

pub trait Else_stmtContextAttrs: ParserRuleContext + BorrowMut<Else_stmtContextExt> {}

impl Else_stmtContextAttrs for Else_stmtContext {}

//impl Else_stmtContext{

//public Else_stmtContext() { }
//pub fn copy_into(&self, ctx: Else_stmtContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type ElseStmtContext = BaseParserRuleContext<ElseStmtContextExt>;

pub trait ElseStmtContextAttrs: ParserRuleContext {
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<Rc<TerminalNode>>
    where
        Self: Sized,
    {
        self.get_token(ELSE, 0)
    }
    fn block(&self) -> Option<Rc<BlockContextAll>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl ElseStmtContextAttrs for ElseStmtContext {}

pub struct ElseStmtContextExt {
    base: Else_stmtContextExt,
}

impl CustomRuleContext for ElseStmtContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_else_stmt
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_ElseStmt(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_ElseStmt(ctx));
    }
}

impl Borrow<Else_stmtContextExt> for ElseStmtContext {
    fn borrow(&self) -> &Else_stmtContextExt {
        &self.base
    }
}
impl BorrowMut<Else_stmtContextExt> for ElseStmtContext {
    fn borrow_mut(&mut self) -> &mut Else_stmtContextExt {
        &mut self.base
    }
}

impl Else_stmtContextAttrs for ElseStmtContext {}

impl ElseStmtContextExt {
    fn new(ctx: &dyn Else_stmtContextAttrs) -> Rc<Else_stmtContextAll> {
        //let base = (cast::<_,Else_stmtContext>(&ctx));
        Rc::new(Else_stmtContextAll::ElseStmtContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ElseStmtContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn else_stmt(&mut self) -> Result<Rc<Else_stmtContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Else_stmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_else_stmt);
        let mut _localctx: Rc<Else_stmtContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let tmp = ElseStmtContextExt::new(&**_localctx);
            recog.base.enter_outer_alt(Some(tmp.clone()), 1);
            _localctx = tmp;
            {
                recog.base.set_state(214);
                recog.base.match_token(ELSE, recog.err_handler.as_mut())?;

                /*InvokeRule block*/
                recog.base.set_state(215);
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
//------------------- while_expr ----------------
#[derive(Debug)]
pub enum While_exprContextAll {
    WhileExprContext(WhileExprContext),
    Error(While_exprContext),
}

impl antlr_rust::parser_rule_context::DerefSeal for While_exprContextAll {}

impl Deref for While_exprContextAll {
    type Target = dyn While_exprContextAttrs;
    fn deref(&self) -> &Self::Target {
        use While_exprContextAll::*;
        match self {
            WhileExprContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}

pub type While_exprContext = BaseParserRuleContext<While_exprContextExt>;

#[derive(Clone)]
pub struct While_exprContextExt {}
impl CustomRuleContext for While_exprContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_while_expr
    }
}

impl While_exprContextExt {
    fn new(
        parent: Option<ParserRuleContextType>,
        invoking_state: isize,
    ) -> Rc<While_exprContextAll> {
        Rc::new(While_exprContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(parent, invoking_state, While_exprContextExt {}),
        ))
    }
}

pub trait While_exprContextAttrs: ParserRuleContext + BorrowMut<While_exprContextExt> {}

impl While_exprContextAttrs for While_exprContext {}

//impl While_exprContext{

//public While_exprContext() { }
//pub fn copy_into(&self, ctx: While_exprContext) {
//	//self.base.copyFrom(ctx);
//
//}
//}

pub type WhileExprContext = BaseParserRuleContext<WhileExprContextExt>;

pub trait WhileExprContextAttrs: ParserRuleContext {
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

impl WhileExprContextAttrs for WhileExprContext {}

pub struct WhileExprContextExt {
    base: While_exprContextExt,
}

impl CustomRuleContext for WhileExprContextExt {
    fn get_rule_index(&self) -> usize {
        RULE_while_expr
    }
    fn enter(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.enter_WhileExpr(ctx));
    }
    fn exit(ctx: &BaseParserRuleContext<Self>, listener: &mut dyn Any)
    where
        Self: Sized,
    {
        listener
            .downcast_mut::<Box<dyn MaysickListener>>()
            .map(|it| it.exit_WhileExpr(ctx));
    }
}

impl Borrow<While_exprContextExt> for WhileExprContext {
    fn borrow(&self) -> &While_exprContextExt {
        &self.base
    }
}
impl BorrowMut<While_exprContextExt> for WhileExprContext {
    fn borrow_mut(&mut self) -> &mut While_exprContextExt {
        &mut self.base
    }
}

impl While_exprContextAttrs for WhileExprContext {}

impl WhileExprContextExt {
    fn new(ctx: &dyn While_exprContextAttrs) -> Rc<While_exprContextAll> {
        //let base = (cast::<_,While_exprContext>(&ctx));
        Rc::new(While_exprContextAll::WhileExprContext(
            BaseParserRuleContext::copy_from(
                ctx,
                WhileExprContextExt {
                    base: ctx.borrow().clone(),
                },
            ),
        ))
    }
}

impl MaysickParser {
    pub fn while_expr(&mut self) -> Result<Rc<While_exprContextAll>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = While_exprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 40, RULE_while_expr);
        let mut _localctx: Rc<While_exprContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let tmp = WhileExprContextExt::new(&**_localctx);
            recog.base.enter_outer_alt(Some(tmp.clone()), 1);
            _localctx = tmp;
            {
                recog.base.set_state(217);
                recog.base.match_token(WHILE, recog.err_handler.as_mut())?;

                recog.base.set_state(218);
                recog
                    .base
                    .match_token(SYM_LPAREN, recog.err_handler.as_mut())?;

                /*InvokeRule expr*/
                recog.base.set_state(219);
                recog.expr()?;

                recog.base.set_state(220);
                recog
                    .base
                    .match_token(SYM_RPAREN, recog.err_handler.as_mut())?;

                /*InvokeRule block*/
                recog.base.set_state(221);
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
	\x1f\u{e2}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\x13\
	\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x03\x02\x07\x02\
	\x2e\x0a\x02\x0c\x02\x0e\x02\x31\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x05\x03\
	\x3f\x0a\x03\x03\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\
	\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x07\x03\
	\x07\x05\x07\x52\x0a\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\x08\x03\x08\
	\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\
	\x07\x0a\x63\x0a\x0a\x0c\x0a\x0e\x0a\x66\x0b\x0a\x03\x0a\x03\x0a\x03\x0a\
	\x03\x0b\x03\x0b\x03\x0b\x07\x0b\x6e\x0a\x0b\x0c\x0b\x0e\x0b\x71\x0b\x0b\
	\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\
	\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{81}\x0a\x0d\x03\x0e\x03\
	\x0e\x03\x0e\x05\x0e\u{86}\x0a\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\
	\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x07\x0f\u{94}\x0a\
	\x0f\x0c\x0f\x0e\x0f\u{97}\x0b\x0f\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\
	\x03\x10\x03\x10\x03\x10\x03\x10\x07\x10\u{a2}\x0a\x10\x0c\x10\x0e\x10\u{a5}\
	\x0b\x10\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x07\x11\u{ad}\x0a\
	\x11\x0c\x11\x0e\x11\u{b0}\x0b\x11\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\
	\x03\x12\x07\x12\u{b8}\x0a\x12\x0c\x12\x0e\x12\u{bb}\x0b\x12\x03\x13\x03\
	\x13\x07\x13\u{bf}\x0a\x13\x0c\x13\x0e\x13\u{c2}\x0b\x13\x03\x13\x03\x13\
	\x03\x13\x07\x13\u{c7}\x0a\x13\x0c\x13\x0e\x13\u{ca}\x0b\x13\x03\x13\x03\
	\x13\x03\x13\x05\x13\u{cf}\x0a\x13\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\
	\x03\x14\x05\x14\u{d7}\x0a\x14\x03\x15\x03\x15\x03\x15\x03\x16\x03\x16\x03\
	\x16\x03\x16\x03\x16\x03\x16\x03\x16\x02\x06\x1c\x1e\x20\x22\x17\x02\x04\
	\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\
	\x2a\x02\x02\x02\u{eb}\x02\x2f\x03\x02\x02\x02\x04\x3e\x03\x02\x02\x02\x06\
	\x40\x03\x02\x02\x02\x08\x43\x03\x02\x02\x02\x0a\x49\x03\x02\x02\x02\x0c\
	\x4f\x03\x02\x02\x02\x0e\x55\x03\x02\x02\x02\x10\x59\x03\x02\x02\x02\x12\
	\x5e\x03\x02\x02\x02\x14\x6a\x03\x02\x02\x02\x16\x74\x03\x02\x02\x02\x18\
	\u{80}\x03\x02\x02\x02\x1a\u{85}\x03\x02\x02\x02\x1c\u{87}\x03\x02\x02\x02\
	\x1e\u{98}\x03\x02\x02\x02\x20\u{a6}\x03\x02\x02\x02\x22\u{b1}\x03\x02\x02\
	\x02\x24\u{ce}\x03\x02\x02\x02\x26\u{d0}\x03\x02\x02\x02\x28\u{d8}\x03\x02\
	\x02\x02\x2a\u{db}\x03\x02\x02\x02\x2c\x2e\x05\x04\x03\x02\x2d\x2c\x03\x02\
	\x02\x02\x2e\x31\x03\x02\x02\x02\x2f\x2d\x03\x02\x02\x02\x2f\x30\x03\x02\
	\x02\x02\x30\x32\x03\x02\x02\x02\x31\x2f\x03\x02\x02\x02\x32\x33\x07\x02\
	\x02\x03\x33\x03\x03\x02\x02\x02\x34\x3f\x05\x06\x04\x02\x35\x3f\x05\x0a\
	\x06\x02\x36\x3f\x05\x08\x05\x02\x37\x3f\x05\x0c\x07\x02\x38\x3f\x05\x0e\
	\x08\x02\x39\x3f\x05\x10\x09\x02\x3a\x3f\x05\x12\x0a\x02\x3b\x3f\x05\x24\
	\x13\x02\x3c\x3f\x05\x26\x14\x02\x3d\x3f\x05\x2a\x16\x02\x3e\x34\x03\x02\
	\x02\x02\x3e\x35\x03\x02\x02\x02\x3e\x36\x03\x02\x02\x02\x3e\x37\x03\x02\
	\x02\x02\x3e\x38\x03\x02\x02\x02\x3e\x39\x03\x02\x02\x02\x3e\x3a\x03\x02\
	\x02\x02\x3e\x3b\x03\x02\x02\x02\x3e\x3c\x03\x02\x02\x02\x3e\x3d\x03\x02\
	\x02\x02\x3f\x05\x03\x02\x02\x02\x40\x41\x05\x16\x0c\x02\x41\x42\x07\x1b\
	\x02\x02\x42\x07\x03\x02\x02\x02\x43\x44\x07\x07\x02\x02\x44\x45\x07\x1c\
	\x02\x02\x45\x46\x07\x11\x02\x02\x46\x47\x05\x16\x0c\x02\x47\x48\x07\x1b\
	\x02\x02\x48\x09\x03\x02\x02\x02\x49\x4a\x07\x08\x02\x02\x4a\x4b\x07\x1c\
	\x02\x02\x4b\x4c\x07\x11\x02\x02\x4c\x4d\x05\x16\x0c\x02\x4d\x4e\x07\x1b\
	\x02\x02\x4e\x0b\x03\x02\x02\x02\x4f\x51\x07\x09\x02\x02\x50\x52\x05\x16\
	\x0c\x02\x51\x50\x03\x02\x02\x02\x51\x52\x03\x02\x02\x02\x52\x53\x03\x02\
	\x02\x02\x53\x54\x07\x1b\x02\x02\x54\x0d\x03\x02\x02\x02\x55\x56\x07\x0a\
	\x02\x02\x56\x57\x07\x1d\x02\x02\x57\x58\x07\x1b\x02\x02\x58\x0f\x03\x02\
	\x02\x02\x59\x5a\x07\x1c\x02\x02\x5a\x5b\x07\x11\x02\x02\x5b\x5c\x05\x16\
	\x0c\x02\x5c\x5d\x07\x1b\x02\x02\x5d\x11\x03\x02\x02\x02\x5e\x5f\x07\x06\
	\x02\x02\x5f\x60\x07\x1c\x02\x02\x60\x64\x07\x0b\x02\x02\x61\x63\x07\x1c\
	\x02\x02\x62\x61\x03\x02\x02\x02\x63\x66\x03\x02\x02\x02\x64\x62\x03\x02\
	\x02\x02\x64\x65\x03\x02\x02\x02\x65\x67\x03\x02\x02\x02\x66\x64\x03\x02\
	\x02\x02\x67\x68\x07\x0c\x02\x02\x68\x69\x05\x24\x13\x02\x69\x13\x03\x02\
	\x02\x02\x6a\x6b\x07\x1c\x02\x02\x6b\x6f\x07\x0b\x02\x02\x6c\x6e\x05\x16\
	\x0c\x02\x6d\x6c\x03\x02\x02\x02\x6e\x71\x03\x02\x02\x02\x6f\x6d\x03\x02\
	\x02\x02\x6f\x70\x03\x02\x02\x02\x70\x72\x03\x02\x02\x02\x71\x6f\x03\x02\
	\x02\x02\x72\x73\x07\x0c\x02\x02\x73\x15\x03\x02\x02\x02\x74\x75\x05\x22\
	\x12\x02\x75\x17\x03\x02\x02\x02\x76\u{81}\x07\x1c\x02\x02\x77\u{81}\x07\
	\x1d\x02\x02\x78\u{81}\x07\x1e\x02\x02\x79\x7a\x07\x0b\x02\x02\x7a\x7b\x05\
	\x16\x0c\x02\x7b\x7c\x07\x0c\x02\x02\x7c\u{81}\x03\x02\x02\x02\x7d\u{81}\
	\x05\x26\x14\x02\x7e\u{81}\x05\x2a\x16\x02\x7f\u{81}\x05\x14\x0b\x02\u{80}\
	\x76\x03\x02\x02\x02\u{80}\x77\x03\x02\x02\x02\u{80}\x78\x03\x02\x02\x02\
	\u{80}\x79\x03\x02\x02\x02\u{80}\x7d\x03\x02\x02\x02\u{80}\x7e\x03\x02\x02\
	\x02\u{80}\x7f\x03\x02\x02\x02\u{81}\x19\x03\x02\x02\x02\u{82}\u{83}\x07\
	\x19\x02\x02\u{83}\u{86}\x05\x18\x0d\x02\u{84}\u{86}\x05\x18\x0d\x02\u{85}\
	\u{82}\x03\x02\x02\x02\u{85}\u{84}\x03\x02\x02\x02\u{86}\x1b\x03\x02\x02\
	\x02\u{87}\u{88}\x08\x0f\x01\x02\u{88}\u{89}\x05\x1a\x0e\x02\u{89}\u{95}\
	\x03\x02\x02\x02\u{8a}\u{8b}\x0c\x05\x02\x02\u{8b}\u{8c}\x07\x17\x02\x02\
	\u{8c}\u{94}\x05\x1a\x0e\x02\u{8d}\u{8e}\x0c\x04\x02\x02\u{8e}\u{8f}\x07\
	\x16\x02\x02\u{8f}\u{94}\x05\x1a\x0e\x02\u{90}\u{91}\x0c\x03\x02\x02\u{91}\
	\u{92}\x07\x13\x02\x02\u{92}\u{94}\x05\x1a\x0e\x02\u{93}\u{8a}\x03\x02\x02\
	\x02\u{93}\u{8d}\x03\x02\x02\x02\u{93}\u{90}\x03\x02\x02\x02\u{94}\u{97}\
	\x03\x02\x02\x02\u{95}\u{93}\x03\x02\x02\x02\u{95}\u{96}\x03\x02\x02\x02\
	\u{96}\x1d\x03\x02\x02\x02\u{97}\u{95}\x03\x02\x02\x02\u{98}\u{99}\x08\x10\
	\x01\x02\u{99}\u{9a}\x05\x1c\x0f\x02\u{9a}\u{a3}\x03\x02\x02\x02\u{9b}\u{9c}\
	\x0c\x04\x02\x02\u{9c}\u{9d}\x07\x18\x02\x02\u{9d}\u{a2}\x05\x1c\x0f\x02\
	\u{9e}\u{9f}\x0c\x03\x02\x02\u{9f}\u{a0}\x07\x19\x02\x02\u{a0}\u{a2}\x05\
	\x1c\x0f\x02\u{a1}\u{9b}\x03\x02\x02\x02\u{a1}\u{9e}\x03\x02\x02\x02\u{a2}\
	\u{a5}\x03\x02\x02\x02\u{a3}\u{a1}\x03\x02\x02\x02\u{a3}\u{a4}\x03\x02\x02\
	\x02\u{a4}\x1f\x03\x02\x02\x02\u{a5}\u{a3}\x03\x02\x02\x02\u{a6}\u{a7}\x08\
	\x11\x01\x02\u{a7}\u{a8}\x05\x1e\x10\x02\u{a8}\u{ae}\x03\x02\x02\x02\u{a9}\
	\u{aa}\x0c\x03\x02\x02\u{aa}\u{ab}\x07\x14\x02\x02\u{ab}\u{ad}\x05\x1e\x10\
	\x02\u{ac}\u{a9}\x03\x02\x02\x02\u{ad}\u{b0}\x03\x02\x02\x02\u{ae}\u{ac}\
	\x03\x02\x02\x02\u{ae}\u{af}\x03\x02\x02\x02\u{af}\x21\x03\x02\x02\x02\u{b0}\
	\u{ae}\x03\x02\x02\x02\u{b1}\u{b2}\x08\x12\x01\x02\u{b2}\u{b3}\x05\x20\x11\
	\x02\u{b3}\u{b9}\x03\x02\x02\x02\u{b4}\u{b5}\x0c\x03\x02\x02\u{b5}\u{b6}\
	\x07\x15\x02\x02\u{b6}\u{b8}\x05\x20\x11\x02\u{b7}\u{b4}\x03\x02\x02\x02\
	\u{b8}\u{bb}\x03\x02\x02\x02\u{b9}\u{b7}\x03\x02\x02\x02\u{b9}\u{ba}\x03\
	\x02\x02\x02\u{ba}\x23\x03\x02\x02\x02\u{bb}\u{b9}\x03\x02\x02\x02\u{bc}\
	\u{c0}\x07\x0f\x02\x02\u{bd}\u{bf}\x05\x04\x03\x02\u{be}\u{bd}\x03\x02\x02\
	\x02\u{bf}\u{c2}\x03\x02\x02\x02\u{c0}\u{be}\x03\x02\x02\x02\u{c0}\u{c1}\
	\x03\x02\x02\x02\u{c1}\u{c3}\x03\x02\x02\x02\u{c2}\u{c0}\x03\x02\x02\x02\
	\u{c3}\u{cf}\x07\x10\x02\x02\u{c4}\u{c8}\x07\x0f\x02\x02\u{c5}\u{c7}\x05\
	\x04\x03\x02\u{c6}\u{c5}\x03\x02\x02\x02\u{c7}\u{ca}\x03\x02\x02\x02\u{c8}\
	\u{c6}\x03\x02\x02\x02\u{c8}\u{c9}\x03\x02\x02\x02\u{c9}\u{cb}\x03\x02\x02\
	\x02\u{ca}\u{c8}\x03\x02\x02\x02\u{cb}\u{cc}\x05\x16\x0c\x02\u{cc}\u{cd}\
	\x07\x10\x02\x02\u{cd}\u{cf}\x03\x02\x02\x02\u{ce}\u{bc}\x03\x02\x02\x02\
	\u{ce}\u{c4}\x03\x02\x02\x02\u{cf}\x25\x03\x02\x02\x02\u{d0}\u{d1}\x07\x03\
	\x02\x02\u{d1}\u{d2}\x07\x0b\x02\x02\u{d2}\u{d3}\x05\x16\x0c\x02\u{d3}\u{d4}\
	\x07\x0c\x02\x02\u{d4}\u{d6}\x05\x24\x13\x02\u{d5}\u{d7}\x05\x28\x15\x02\
	\u{d6}\u{d5}\x03\x02\x02\x02\u{d6}\u{d7}\x03\x02\x02\x02\u{d7}\x27\x03\x02\
	\x02\x02\u{d8}\u{d9}\x07\x04\x02\x02\u{d9}\u{da}\x05\x24\x13\x02\u{da}\x29\
	\x03\x02\x02\x02\u{db}\u{dc}\x07\x05\x02\x02\u{dc}\u{dd}\x07\x0b\x02\x02\
	\u{dd}\u{de}\x05\x16\x0c\x02\u{de}\u{df}\x07\x0c\x02\x02\u{df}\u{e0}\x05\
	\x24\x13\x02\u{e0}\x2b\x03\x02\x02\x02\x13\x2f\x3e\x51\x64\x6f\u{80}\u{85}\
	\u{93}\u{95}\u{a1}\u{a3}\u{ae}\u{b9}\u{c0}\u{c8}\u{ce}\u{d6}";
