use std::collections::HashMap;

use crate::{
    ast::{MacroDecl, MacroFragment, Program, TopLevel, TopLevelKind},
    lexer::{Token, TokenType},
    parser::{Parsable, ParseCtx},
};

pub fn expand_macros(mut program: Program) -> Program {
    let mut decls = HashMap::new();

    for (i, top_level) in program.top_levels.iter().enumerate() {
        match &top_level.kind {
            TopLevelKind::MacroInvoc(invocation) => {
                let TopLevelKind::MacroDecl(ref decl) = program
                    .top_level_from_ident(&invocation.name.name)
                    .unwrap()
                    .kind
                else {
                    panic!("Macro not found")
                };

                decls.insert(i, (decl.clone(), invocation.args.clone()));
            }
            _ => (),
        }
    }

    program.top_levels = program
        .top_levels
        .into_iter()
        .enumerate()
        .map(|(i, top_level)| match top_level.kind {
            TopLevelKind::MacroInvoc(_) => {
                let (decl, args) = decls.get(&i).unwrap();
                expand_top_level(decl, args.clone())
            }
            _ => vec![top_level],
        })
        .flatten()
        .collect();

    program
}

fn expand_top_level(macro_decl: &MacroDecl, args: Vec<Token>) -> Vec<TopLevel> {
    let entries = &macro_decl.entries;
    let mut top_levels = vec![];

    for entry in entries {
        let defs = &entry.defs;
        let mut correspondances = HashMap::new();

        if defs.len() != args.len() {
            continue;
        }

        for (i, def) in defs.iter().enumerate() {
            if let MacroFragment::Ident(ident) = def {
                correspondances.insert(ident.name.clone(), args[i].clone());
            }
        }

        // replace in the body
        let body = entry
            .block
            .iter()
            .map(|token| match &token.token_type {
                TokenType::MacroInvoc(name) => correspondances.get(name).unwrap().clone(),
                _ => token.clone(),
            })
            .collect::<Vec<_>>();

        top_levels.push(TopLevel::parse(&body, &mut ParseCtx::new()).unwrap().0);
    }

    top_levels
    //parse the top_level
}
