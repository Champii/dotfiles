use std::collections::HashMap;

use crate::{
    ast::{MacroDecl, MacroFragment, Program, TopLevel, TopLevelKind},
    lexer::{Token, TokenType},
    parser::{Parsable, ParseCtx, ParseError},
};

pub fn expand_macros(mut program: Program) -> Result<Program, ParseError> {
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
                        // It might be defined later
                        continue;
                    };

                    decls.insert(i, (decl.clone(), invocation.args.clone()));
                }
                _ => (),
            }
        }
        program = expand_macros_once(program, &decls)?;

        depth += 1;

        if depth > 100 {
            panic!("Macro expansion depth exceeded (>100)");
        }
    }

    Ok(program)
}

fn expand_macros_once(
    mut program: Program,
    decls: &HashMap<usize, (MacroDecl, Vec<Token>)>,
) -> Result<Program, ParseError> {
    let results = program
        .top_levels
        .into_iter()
        .enumerate()
        .map(|(i, top_level)| match top_level.kind {
            TopLevelKind::MacroInvoc(_) => {
                let Some((decl, args)) = decls.get(&i) else {
                    return Ok(vec![top_level]);
                };
                expand_top_level(decl, args.clone())
            }
            _ => Ok(vec![top_level]),
        })
        .collect::<Vec<_>>();

    program.top_levels = vec![];

    for result in results {
        program.top_levels.extend(result?);
    }

    Ok(program)
}

fn expand_top_level(macro_decl: &MacroDecl, args: Vec<Token>) -> Result<Vec<TopLevel>, ParseError> {
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
                _ => unimplemented!(),
            }
        }

        let body = entry
            .block
            .iter()
            .map(|token| match &token.token_type {
                TokenType::MacroVar(name) => {
                    if let Some(corresp) = correspondances.get(name) {
                        corresp.clone()
                    } else {
                        token.clone()
                    }
                }
                _ => token.clone(),
            })
            .collect::<Vec<_>>();

        let (program, _) = Program::parse(&body, &mut ParseCtx::new())?;

        top_levels.extend(program.top_levels);
    }

    if top_levels.is_empty() {
        return Err(ParseError::MacroNoCorrespondance(macro_decl.name.clone()));
    }

    Ok(top_levels)
}
