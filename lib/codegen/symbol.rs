use crate::codegen::types::*;

#[derive(Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub arguments: Vec<ObjectType>,
    pub retval: ObjectType,
    pub is_maysick_symbol: bool,
}

pub fn mangle(symbol: &Symbol) -> String {
    let mut symname = String::new();

    if !symbol.is_maysick_symbol {
        symname.push_str(&symbol.name);
        return symname;
    }

    // add prefix
    symname.push_str("_m");

    // add argument types
    for ty in &symbol.arguments {
        let t = match ty {
            ObjectType::Integer => "i",
            ObjectType::String => "S",
            ObjectType::Bool => "b",
            _ => "A",
        };

        symname.push_str(t);
    }

    // add return type
    symname.push('_');
    symname.push_str(match &symbol.retval {
        ObjectType::Integer => "i",
        ObjectType::String => "S",
        ObjectType::Bool => "b",
        ObjectType::Any => "A",
        ObjectType::Nil => "",
    });

    // add name
    symname.push('_');
    symname.push_str(&symbol.name);

    symname
}
