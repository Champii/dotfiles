use std::collections::HashMap;

use crate::{
    ast::{MacroDecl, MacroFragment, Program, TopLevel, TopLevelKind},
    lexer::{Token, TokenType},
    parser::{Parsable, ParseCtx},
};

pub fn expand_macros(mut program: Program) -> Program {
    let mut depth = 0;

    while program.has_macro_invoc() {
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
        program = expand_macros_once(program, &decls);

        depth += 1;

        if depth > 100 {
            panic!("Macro expansion depth exceeded (>100)");
        }
    }

    program
}

fn expand_macros_once(
    mut program: Program,
    decls: &HashMap<usize, (MacroDecl, Vec<Token>)>,
) -> Program {
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

    'first_loop: for entry in entries {
        let defs = &entry.defs;
        let mut correspondances = HashMap::new();

        if defs.len() != args.len() {
            continue;
        }

        for (i, def) in defs.iter().enumerate() {
            match def {
                MacroFragment::Ident(ident) => {
                    if let TokenType::Ident(_) = args[i].token_type {
                        correspondances.insert(ident.name.clone(), args[i].clone());
                    } else {
                        continue 'first_loop;
                    }
                }
                MacroFragment::Token(token) => {
                    if token.token_type != args[i].token_type {
                        continue 'first_loop;
                    }
                }
            }
        }

        let body = entry
            .block
            .iter()
            .map(|token| match &token.token_type {
                TokenType::MacroVar(name) => correspondances.get(name).unwrap().clone(),
                _ => token.clone(),
            })
            .collect::<Vec<_>>();

        if let Ok((top_level, _)) = TopLevel::parse(&body, &mut ParseCtx::new()) {
            top_levels.push(top_level);
        }
    }

    if top_levels.is_empty() {
        panic!("No correspondance found for macro invocation");
    }

    top_levels
}
