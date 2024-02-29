use std::collections::HashMap;

use crate::{
    ast::{MacroDecl, MacroFragment, Program, TopLevel, TopLevelKind},
    lexer::{Token, TokenType},
    parser::{Parsable, ParseCtx},
};

pub fn expand_macros(mut program: Program) -> Program {
    let top_levels = program
        .top_levels
        .iter()
        .enumerate()
        .filter_map(|(i, item)| match &item.kind {
            TopLevelKind::MacroInvoc(invocation) => {
                let TopLevelKind::MacroDecl(ref decl) = program
                    .top_level_from_ident(&invocation.name.name)
                    .unwrap()
                    .kind
                else {
                    panic!("Macro not found")
                };

                let new_top_level = expand_top_level(decl, invocation.args.clone());

                Some((i, new_top_level))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    for (i, new_top_level) in top_levels {
        program.top_levels[i] = new_top_level;
    }

    program
}

fn expand_top_level(macro_decl: &MacroDecl, args: Vec<Token>) -> TopLevel {
    let defs = &macro_decl.defs;

    let mut correspondances = HashMap::new();

    for (i, def) in defs.iter().enumerate() {
        if let MacroFragment::Ident(ident) = def {
            correspondances.insert(ident.name.clone(), args[i].clone());
        }
    }

    // replace in the body
    let body = macro_decl
        .block
        .iter()
        .map(|token| match &token.token_type {
            TokenType::MacroInvoc(name) => correspondances.get(name).unwrap().clone(),
            _ => token.clone(),
        })
        .collect::<Vec<_>>();

    //parse the top_level
    TopLevel::parse(&body, &mut ParseCtx::new()).unwrap().0
}
