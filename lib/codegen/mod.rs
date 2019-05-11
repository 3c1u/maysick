/*
 * maysick
 *
 * 2018 - murueka
 */

use std::collections::*;
use std::iter::*;

use crate::ast::*;

pub mod symbol;
pub mod types;

use symbol::*;
use types::*;

trait Codegen {
    fn generate(
        &self,
        global: &mut GlobalCodegen,
        block: &mut BlockCodegen,
    ) -> (String, ObjectType);
}

pub struct Variable {
    pub o_type: ObjectType,
    pub immutable: bool,
}

pub fn cast_code(c: &String, ta: ObjectType, tb: ObjectType) -> String {
    let mut code = String::new();

    if ta == tb {
        return c.to_owned();
    }

    code.push_str(match tb {
        ObjectType::Integer => "m_to_integer",
        ObjectType::String => "m_to_string",
        ObjectType::Bool => "0 != ",
        _ => "",
    });
    code.push('(');
    code.push_str(match ta {
        ObjectType::Integer => "m_any_int",
        ObjectType::String => "m_any_string",
        ObjectType::Bool => "m_any_bool",
        ObjectType::Nil => "m_any_nil",
    });
    code.push('(');
    code.push_str(&c);
    code.push(')');
    code.push(')');

    code
}

impl Codegen for Expr {
    fn generate(
        &self,
        global: &mut GlobalCodegen,
        block: &mut BlockCodegen,
    ) -> (String, ObjectType) {
        let mut code: String = String::new();

        match self {
            Expr::Ident(s) => {
                code.push_str(s);
                let v = block.variable_defs.get(s);
                return if let Some(v) = v {
                    (s.clone(), v.o_type)
                } else {
                    (s.clone(), ObjectType::Nil)
                };
            }
            Expr::Literal(l) => match l {
                Literal::Integer(i) => {
                    let f = format!("{}", i);
                    code.push_str(&f);
                    return (code, ObjectType::Integer);
                }
                Literal::String(s) => {
                    code.push_str("m_string_from_cstr(\"");
                    code.push_str(&s.escape_debug().to_string());
                    code.push_str("\")");

                    return (code, ObjectType::String);
                }
                Literal::Bool(b) => {
                    if *b {
                        code.push_str("true");
                    } else {
                        code.push_str("false");
                    }

                    return (code, ObjectType::Bool);
                }
            },
            Expr::Infix(i, a, b) => {
                let a = a.generate(global, block);
                let b = b.generate(global, block);

                if a.1 == ObjectType::Integer && a.1 == ObjectType::Integer {
                    code.push('(');
                    code.push_str(&a.0);
                    code.push(' ');
                    code.push(match i {
                        Infix::EqualOp => '=',
                        Infix::AddOp => '+',
                        Infix::SubOp => '-',
                        Infix::ModOp => '%',
                        Infix::DivOp => '/',
                        Infix::MulOp => '*',
                        Infix::AndOp => '&',
                        Infix::OrOp => '|',
                        _ => '#',
                    });
                    code.push(' ');
                    code.push_str(&b.0);
                    code.push(')');

                    return (code, ObjectType::Integer);
                } else {
                    code.push_str("m_string_concat(");
                    code.push_str(&cast_code(&a.0, a.1, ObjectType::String));
                    code.push(',');
                    code.push_str(&cast_code(&b.0, b.1, ObjectType::String));
                    code.push(')');
                    return (code, ObjectType::String);
                }
            }
            Expr::FnCall(f, args) => {
                let args: Vec<_> = args
                    .into_iter()
                    .map(|v| v.generate(global, block))
                    .collect();

                let types: Vec<_> = args.clone().into_iter().map(|v| v.1).collect();

                let sym = global.lookup_fn(f, &types).unwrap();

                code.push_str(&mangle(&sym));
                code.push('(');

                for i in 0..args.len() {
                    if i != 0 {
                        code.push_str(", ");
                    }
                    if types[i] == sym.arguments[i] {
                        code.push_str(&args[i].0);
                    } else {
                        code.push_str(&cast_code(&args[i].0, types[i], sym.arguments[i]));
                    }
                }

                code.push(')');

                return (code, sym.retval);
            }
            Expr::If(c, b) => {}
            Expr::While(c, b) => {}
            Expr::Prefix(_, _) => {}
        }

        (code, ObjectType::Nil)
    }
}

impl Codegen for Stmt {
    fn generate(
        &self,
        global: &mut GlobalCodegen,
        block: &mut BlockCodegen,
    ) -> (String, ObjectType) {
        let mut code = String::new();

        match self {
            Stmt::FnDef(i, a, b) => {}
            Stmt::Let(i, _, e) => {
                let (c, t) = e.generate(global, block);
                code.push_str(&i);
                code.push_str(" = ");
                code.push_str(&c);
                code.push_str(";\n");
                block.variable_defs.insert(
                    i.clone(),
                    Variable {
                        o_type: t,
                        immutable: true,
                    },
                );
            }
            Stmt::Var(i, _, e) => {
                let (c, t) = e.generate(global, block);
                code.push_str(&i);
                code.push_str(" = ");
                code.push_str(&c);
                code.push_str(";\n");
                block.variable_defs.insert(
                    i.clone(),
                    Variable {
                        o_type: t,
                        immutable: true,
                    },
                );
            }
            Stmt::Subst(i, e) => {
                let (c, t) = e.generate(global, block);
                code.push_str(&i);
                code.push_str(" = ");
                code.push_str(&c);
                code.push_str(";\n");
            }
            Stmt::Return(e) => {
                if let Some(e) = e {
                    let (c, t) = e.generate(global, block);
                    code.push_str("return ");
                    code.push_str(&c);
                    code.push_str(";\n");
                } else {
                    code.push_str("return;");
                }
            }
            Stmt::Expr(e) => {
                let (c, t) = e.generate(global, block);
                code.push_str(&c);
                code.push_str(";\n");
            }
            Stmt::Import(s) => {}
        }

        block.proc.push_str(&code);

        (String::new(), ObjectType::Nil)
    }
}

pub struct BlockCodegen {
    pub variable_defs: HashMap<String, Variable>,
    pub proc: String,
}

pub struct GlobalCodegen {
    pub fn_syms: HashMap<String, (bool, Vec<Symbol>)>,
    pub fn_defs: Vec<(Symbol, Block)>,
}

impl GlobalCodegen {
    pub fn register_builtin(&mut self) {
        self.fn_syms.insert(
            "println".to_owned(),
            (
                true,
                vec![Symbol {
                    name: "println".to_owned(),
                    arguments: vec![ObjectType::String],
                    retval: ObjectType::Nil,
                    is_maysick_symbol: true,
                }],
            ),
        );
        self.fn_syms.insert(
            "print".to_owned(),
            (
                true,
                vec![Symbol {
                    name: "print".to_owned(),
                    arguments: vec![ObjectType::String],
                    retval: ObjectType::Nil,
                    is_maysick_symbol: true,
                }],
            ),
        );
        self.fn_syms.insert(
            "getchar".to_owned(),
            (
                true,
                vec![Symbol {
                    name: "getchar".to_owned(),
                    arguments: vec![],
                    retval: ObjectType::String,
                    is_maysick_symbol: true,
                }],
            ),
        );
        self.fn_syms.insert(
            "random".to_owned(),
            (
                true,
                vec![Symbol {
                    name: "random".to_owned(),
                    arguments: vec![],
                    retval: ObjectType::Integer,
                    is_maysick_symbol: true,
                }],
            ),
        );
        self.fn_syms.insert(
            "char_at".to_owned(),
            (
                true,
                vec![Symbol {
                    name: "char_at".to_owned(),
                    arguments: vec![ObjectType::String, ObjectType::Integer],
                    retval: ObjectType::Integer,
                    is_maysick_symbol: true,
                }],
            ),
        );
        self.fn_syms.insert(
            "char_from".to_owned(),
            (
                true,
                vec![Symbol {
                    name: "char_from".to_owned(),
                    arguments: vec![ObjectType::Integer],
                    retval: ObjectType::String,
                    is_maysick_symbol: true,
                }],
            ),
        );
        self.fn_syms.insert(
            "integer_as_hex".to_owned(),
            (
                true,
                vec![Symbol {
                    name: "integer_as_hex".to_owned(),
                    arguments: vec![ObjectType::Integer],
                    retval: ObjectType::String,
                    is_maysick_symbol: true,
                }],
            ),
        );
    }
}

impl GlobalCodegen {
    pub fn generate(&self) -> (String, ObjectType) {
        let mut code: String = String::new();

        (code, ObjectType::Nil)
    }

    pub fn lookup_fn(&mut self, f: &String, a: &Vec<ObjectType>) -> Option<&Symbol> {
        let (builtin, sym) = self.fn_syms.get_mut(f)?;

        if *builtin {
            return Some(&sym[0]);
        }

        for s in sym {
            if &s.arguments == a {
                return Some(s);
            }
        }

        None
    }
}

impl BlockCodegen {
    pub fn generate(&self) -> (String, ObjectType) {
        let mut code: String = String::new();

        for (s, v) in &self.variable_defs {
            code.push_str(match v.o_type {
                ObjectType::Integer => "m_int      ",
                ObjectType::String => "m_string*  ",
                ObjectType::Bool => "bool       ",
                ObjectType::Nil => "void*      ",
            });
            code.push_str(&s);
            code.push_str(";\n");
        }

        code.push('\n');
        code.push_str(&self.proc);

        (code, ObjectType::Nil)
    }
}

pub fn prelude_export() {
    println!("{}", include_str!("prelude.c"));
}

pub fn generate_code(p: Program) {
    // export all prelude program
    prelude_export();

    // gen global
    let mut glbl = GlobalCodegen {
        fn_syms: HashMap::new(),
        fn_defs: Vec::new(),
    };

    let mut block = BlockCodegen {
        variable_defs: HashMap::new(),
        proc: String::new(),
    };

    // register built-in functions
    glbl.register_builtin();

    for stmt in p {
        stmt.generate(&mut glbl, &mut block);
    }

    println!("void m_entry() {{\n{}}}", block.generate().0);
}
