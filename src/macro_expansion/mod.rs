use std::collections::HashMap;

use crate::{
    ast::{MacroDecl, MacroFragment, Program, TopLevel, TopLevelKind},
    lexer::{Token, TokenType},
    parser::{Parsable, ParseCtx, ParseError},
};

use self::{correspondances::Correspondance, macro_arg_matcher::MacroArgMatcher};

mod correspondances;
mod macro_arg_matcher;

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

    for entry in entries {
        let defs = &entry.defs;
        let mut macro_matcher = MacroArgMatcher::new(&args, defs.clone());
        let Ok(correspondances) = macro_matcher.run() else {
            continue;
        };

        let body = replace_body_variables(entry.body.clone(), &correspondances, 0);

        let (program, _) = Program::parse(&body, &mut ParseCtx::new())?;

        top_levels.extend(program.top_levels);

        break;
    }

    if top_levels.is_empty() {
        return Err(ParseError::MacroNoCorrespondance(macro_decl.name.clone()));
    }

    Ok(top_levels)
}

fn replace_body_variables(
    body: Vec<MacroFragment>,
    correspondances: &Correspondance,
    correspondance_level: usize,
) -> Vec<Token> {
    body.iter()
        .map(|fragment| match &fragment {
            MacroFragment::Ident(name) => {
                if let Some(corresp) = correspondances.get(&name.name.clone(), correspondance_level)
                {
                    corresp.clone()
                } else {
                    vec![Token {
                        token_type: TokenType::MacroVar(name.name.clone()),
                        span: name.span.clone(),
                    }]
                }
            }
            MacroFragment::Token(token) => vec![token.clone()],
            MacroFragment::Repetition(repetition) => {
                let repetition_names = repetition
                    .iter()
                    .filter_map(|fragment| match fragment {
                        MacroFragment::Ident(ident) => Some(ident.name.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                // find nested correspondance from names
                let Some((index, nested_correspondance)) = correspondances
                    .nested_corresp
                    .iter()
                    .enumerate()
                    .find(|(_i, corresp)| corresp.keys() == repetition_names)
                else {
                    return vec![];
                };

                let mut repetitions = vec![];

                for (i, _ident) in nested_correspondance
                    .entries
                    .iter()
                    .enumerate()
                    .nth(0)
                    .unwrap()
                    .1
                     .1
                    .iter()
                    .enumerate()
                {
                    let mut new_correspondances = correspondances.clone();
                    new_correspondances.nested_corresp[index].entries = new_correspondances
                        .nested_corresp[index]
                        .entries
                        .iter()
                        .map(|(name, tokens)| {
                            (name.clone(), vec![tokens.iter().nth(i).unwrap().clone()])
                        })
                        .collect();

                    repetitions.push(replace_body_variables(
                        repetition.clone(),
                        &new_correspondances,
                        correspondance_level + 1,
                    ));
                }

                repetitions.into_iter().flatten().collect::<Vec<_>>()
            }
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_string;

    use super::*;

    #[test]
    fn simple_macro_expand() {
        let input = r#"macro mymacro
  a b c =>
    main = -> 1
%mymacro a b c"#;

        let expected = r#"macro mymacro
  a b c =>
    main = -> 1
main = -> 1"#;

        let input_program = parse_string(input).unwrap();
        let expanded = expand_macros(input_program).unwrap();

        let expected_program = parse_string(expected).unwrap();

        assert_eq!(expanded, expected_program);
    }

    #[test]
    fn simple_macro_expand_fail() {
        let input = r#"macro mymacro
  a b c =>
    main = -> 1
%mymacro a c b"#;

        let input_program = parse_string(input).unwrap();
        let expanded = expand_macros(input_program);

        assert!(expanded.is_err());
    }

    #[test]
    fn argument_matching() {
        let input = r#"macro mymacro
  $a:ident $b:ident $c:ident =>
    $a = $b -> $c
%mymacro x y z "#;

        let expected = r#"macro mymacro
  $a:ident $b:ident $c:ident =>
    $a = $b -> $c
x = y -> z"#;

        let input_program = parse_string(input).unwrap();
        let expanded = expand_macros(input_program).unwrap();

        let expected_program = parse_string(expected).unwrap();

        assert_eq!(expanded, expected_program);
    }

    #[test]
    fn argument_repetition() {
        let input = r#"macro mymacro
  $a:ident $($b:ident)* $c:ident =>
    $a = $($b,)* -> $c
%mymacro a b c d e"#;

        let expected = r#"macro mymacro
  $a:ident $($b:ident)* $c:ident =>
    $a = $($b,)* -> $c
a = b, c, d, -> e"#;

        let input_program = parse_string(input).unwrap();
        let expanded = expand_macros(input_program).unwrap();

        let expected_program = parse_string(expected).unwrap();

        assert_eq!(expanded, expected_program);
    }

    #[test]
    fn multi_entries_macro() {
        let input = r#"macro mymacro
  $a:ident $b:ident $c:ident =>
    $a = $b -> $c
  $a:ident =>
    $a = -> 1
%mymacro x y z
%mymacro x"#;

        let expected = r#"macro mymacro
  $a:ident $b:ident $c:ident =>
    $a = $b -> $c
  $a:ident =>
    $a = -> 1
x = y -> z
x = -> 1"#;

        let input_program = parse_string(input).unwrap();
        let expanded = expand_macros(input_program).unwrap();

        let expected_program = parse_string(expected).unwrap();

        assert_eq!(expanded, expected_program);
    }

    #[test]
    fn no_repetition() {
        let input = r#"macro mymacro
  $a:ident $($b:ident)* $c:ident =>
    $a = $($b,)* -> $c
%mymacro a c"#;
        let expected = r#"macro mymacro
  $a:ident $($b:ident)* $c:ident =>
    $a = $($b,)* -> $c
a = -> c"#;

        let input_program = parse_string(input).unwrap();
        let expanded = expand_macros(input_program).unwrap();

        let expected_program = parse_string(expected).unwrap();

        assert_eq!(expanded, expected_program);
    }

    #[test]
    fn empty_repetition_matches_less_args() {
        let input = r#"macro mymacro
  $name:ident $($args:ident)* =>
    $name = $($args,)* -> 1
%mymacro a"#;

        let expected = r#"macro mymacro
  $name:ident $($args:ident)* =>
    $name = $($args,)* -> 1
a = -> 1"#;

        let input_program = parse_string(input).unwrap();
        let expanded = expand_macros(input_program).unwrap();

        let expected_program = parse_string(expected).unwrap();

        assert_eq!(expanded, expected_program);
    }
}
