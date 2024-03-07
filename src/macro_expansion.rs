use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{
    ast::{Ident, MacroDecl, MacroFragment, Program, TopLevel, TopLevelKind},
    lexer::{Span, Token, TokenType},
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

/* #[derive(Debug, Clone)]
enum CorrespondanceKind {
    Direct(Token),
    Nested(BTreeMap<String, CorrespondanceKind>),
} */

#[derive(Debug, Clone)]
struct Correspondance {
    pub entries: BTreeMap<String, Vec<Token>>,
    nested_corresp_keys: Vec<Vec<String>>,
    nested_corresp: Vec<Correspondance>,
}

impl Correspondance {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            nested_corresp_keys: vec![],
            nested_corresp: vec![],
        }
    }

    pub fn insert_direct(&mut self, name: String, token: Token) {
        let entry = self.entries.entry(name).or_insert(vec![token.clone()]);

        if !entry.contains(&token) {
            entry.push(token);
        }
    }

    pub fn insert_nested(&mut self, correspondance: Correspondance) {
        if self
            .nested_corresp_keys
            .contains(&correspondance.keys().clone())
        {
            let nested_idx = self
                .nested_corresp_keys
                .iter()
                .enumerate()
                .find(|(i, names)| **names == correspondance.keys())
                .map(|(i, _)| i);

            if let Some(i) = nested_idx {
                self.nested_corresp[i].merge(&correspondance);
            }
        } else {
            self.nested_corresp_keys.push(correspondance.keys());
            self.nested_corresp.push(correspondance);
        };
    }

    /* pub fn get_nested_scope_of(&self, name: &str) -> Option<BTreeMap<String, CorrespondanceKind>> {
        match self.correspondances.get(name) {
            Some(CorrespondanceKind::Direct(token)) => Some(self.correspondances.clone()),
            Some(CorrespondanceKind::Nested(correspondance)) => Some(correspondance.clone()),
            _ => None,
        }
    } */

    /* pub fn get(&self, name: &str) -> Option<Token> {
        match self.correspondances.get(name) {
            Some(CorrespondanceKind::Direct(token)) => Some(token.clone()),
            _ => None,
        }
    } */

    pub fn keys(&self) -> Vec<String> {
        self.entries
            .keys()
            .cloned()
            .chain(
                self.nested_corresp
                    .iter()
                    .map(|corresp| corresp.keys())
                    .flatten(),
            )
            .collect()
    }

    pub fn get(&self, name: &str, max_level: usize) -> Option<Vec<Token>> {
        if max_level == 0 {
            return self.entries.get(name).cloned();
        } else {
            if let Some(tokens) = self.entries.get(name) {
                return Some(tokens.clone());
            }

            for (i, inner_name) in self.nested_corresp_keys.iter().enumerate() {
                if inner_name.contains(&name.to_string()) {
                    return self.nested_corresp[i].get(name, max_level - 1);
                }
            }
            None
        }
    }

    pub fn merge(&mut self, other: &Self) {
        for (name, tokens) in other.entries.clone() {
            let entry = self.entries.entry(name).or_insert(vec![]);
            entry.extend(tokens);
        }

        for (i, nested_corresp) in other.nested_corresp.iter().enumerate() {
            if self.nested_corresp_keys.contains(&nested_corresp.keys()) {
                let idx = self
                    .nested_corresp_keys
                    .iter()
                    .position(|keys| *keys == nested_corresp.keys())
                    .unwrap();
                self.nested_corresp[idx].merge(&nested_corresp.clone());
            } else {
                self.nested_corresp_keys.push(nested_corresp.keys());
                self.nested_corresp.push(nested_corresp.clone());
            }
        }
    }
}

#[derive(Clone, Debug)]
struct MacroThread<'a> {
    pub args: &'a [Token],
    pub tokens: Vec<MacroFragment>,
    pub correspondances: Correspondance,
}

#[derive(Debug)]
struct MacroArgMatcher<'a> {
    threads: Vec<MacroThread<'a>>,
    must_be_completed: bool,
}

impl<'a> MacroArgMatcher<'a> {
    fn new(args: &'a [Token], thread: Vec<MacroFragment>) -> Self {
        Self {
            threads: vec![MacroThread {
                args,
                tokens: thread,
                correspondances: Correspondance::new(),
            }],
            must_be_completed: false,
        }
    }

    pub fn run(&mut self) -> Result<Correspondance, ParseError> {
        self.must_be_completed = true;
        let (correspondances, _) = self.match_threads()?;

        Ok(correspondances)
    }

    fn match_threads(&mut self) -> Result<(Correspondance, &'a [Token]), ParseError> {
        // let mut args = &self.args[..];

        /* while let Some(token) = args.get(0) {
        let mut new_threads = vec![]; */

        let mut new_threads = self.threads.clone();

        while !has_one_solution(new_threads.clone(), self.must_be_completed)
            && !new_threads.is_empty()
        {
            let mut new_new_threads = vec![];

            for thread in &mut new_threads {
                if let Some(arg) = thread.args.get(0) {
                    let tokens = &thread.tokens;
                    if tokens.is_empty() {
                        new_new_threads.push(thread.clone());
                        continue;
                    }

                    let fragment = &tokens[0];

                    match fragment {
                        MacroFragment::Ident(ident) => {
                            if let TokenType::Ident(_) = arg.token_type {
                                thread
                                    .correspondances
                                    .insert_direct(ident.name.clone(), arg.clone());
                                new_new_threads.push(MacroThread {
                                    args: &thread.args[1..],
                                    tokens: tokens[1..].to_vec(),
                                    correspondances: thread.correspondances.clone(),
                                });
                            }
                        }
                        MacroFragment::Token(t) => {
                            if t.token_type == arg.token_type {
                                new_new_threads.push(MacroThread {
                                    args: &thread.args[1..],
                                    tokens: tokens[1..].to_vec(),
                                    correspondances: thread.correspondances.clone(),
                                });
                            }
                        }
                        MacroFragment::Repetition(repetition) => {
                            // Case no repetition
                            new_new_threads.push(MacroThread {
                                args: &thread.args,
                                tokens: tokens[1..].to_vec(),
                                correspondances: thread.correspondances.clone(),
                            });

                            let mut matcher = MacroArgMatcher::new(thread.args, repetition.clone());
                            if let Ok((correspondances, new_args)) = matcher.match_threads() {
                                thread
                                    .correspondances
                                    .insert_nested(correspondances.clone());

                                println!("NEW ARGS {:#?}", new_args);

                                // Case repetition found and it continues
                                let new_thread_matched = MacroThread {
                                    args: new_args,
                                    tokens: tokens.to_vec(),
                                    correspondances: thread.correspondances.clone(),
                                };

                                new_new_threads.push(new_thread_matched);

                                // Case repetition found and it stops
                                let new_thread_matched = MacroThread {
                                    args: new_args,
                                    tokens: tokens[1..].to_vec(),
                                    correspondances: thread.correspondances.clone(),
                                };

                                new_new_threads.push(new_thread_matched);
                            }
                        }
                    }
                } else {
                    // FOUND IT
                    if (self.must_be_completed
                        && thread.tokens.is_empty()
                        && thread.args.is_empty())
                        || (!self.must_be_completed && (thread.tokens.is_empty()))
                    {
                        println!("FOUND IT {:#?}", thread);
                        return Ok((thread.correspondances.clone(), thread.args));
                    }
                    // return Ok((thread.correspondances.clone(), thread.args.clone()));
                }
            }
            self.threads = new_new_threads.clone();
            new_threads = new_new_threads.clone();
        }

        println!("THREADS {:#?}", self.threads);

        if let Some(thread) = get_correspondances_thread(new_threads.clone()) {
            Ok((thread.correspondances.clone(), thread.args.clone()))
        } else {
            Err(ParseError::MacroNoCorrespondance(Ident {
                name: "macro".to_string(),
                span: Span::default(),
            }))
        }
    }
}

fn has_one_solution(threads: Vec<MacroThread>, must_be_completed: bool) -> bool {
    threads.iter().any(|thread| {
        (must_be_completed && (thread.tokens.is_empty() && thread.args.is_empty()))
            || (!must_be_completed && (thread.tokens.is_empty() || thread.args.is_empty()))
    })
}

fn get_correspondances_thread<'a>(threads: Vec<MacroThread<'a>>) -> Option<MacroThread<'a>> {
    if let Some(found) = threads
        .iter()
        .find(|thread| thread.tokens.is_empty() && thread.args.is_empty())
    {
        return Some(found.clone());
    } else if let Some(found) = threads.iter().find(|thread| thread.args.is_empty()) {
        return Some(found.clone());
    } else if let Some(found) = threads.iter().find(|thread| thread.tokens.is_empty()) {
        return Some(found.clone());
    } else {
        None
    }
    // .map(|thread| thread.correspondances.clone())
}

fn expand_top_level(macro_decl: &MacroDecl, args: Vec<Token>) -> Result<Vec<TopLevel>, ParseError> {
    let entries = &macro_decl.entries;
    let mut top_levels = vec![];

    'first_loop: for entry in entries {
        let defs = &entry.defs;
        let mut macro_matcher = MacroArgMatcher::new(&args, defs.clone());
        let correspondances = macro_matcher.run()?;
        println!("{:#?}", correspondances);

        /* for (i, def) in defs.iter().enumerate() {
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
        } */

        /* let body = entry
        .body
        .iter()
        .map(|token| match &token.token_type {
            TokenType::MacroVar(name) => {
                if let Some(corresp) = correspondances.get(name) {
                    corresp.clone()
                } else {
                    // If no correspondance, it might be defined by a nested macro later
                    token.clone()
                }
            }
            _ => token.clone(),
        })
        .collect::<Vec<_>>(); */

        let body = replace_body_variables(entry.body.clone(), &correspondances, 0);

        println!("BODY VARIABLES: {:#?}", body);

        let (program, _) = Program::parse(&body, &mut ParseCtx::new())?;

        top_levels.extend(program.top_levels);
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
                    // } else {
                    // If no correspondance, it might be defined by a nested macro later
                    // fragment.clone()
                } else {
                    panic!("No correspondance for macro variable {:#?}", name);
                }

                /* Token {
                    token_type: TokenType::MacroVar(name.name.clone()),
                    span: name.span.clone(),
                } */
            }
            MacroFragment::Token(token) => vec![token.clone()],
            MacroFragment::Repetition(repetition) => {
                // find correspondance for repetition

                let repetition_names = repetition
                    .iter()
                    .filter_map(|fragment| match fragment {
                        MacroFragment::Ident(ident) => Some(ident.name.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                // find nested correspondance from names
                let (index, nested_correspondance) = correspondances
                    .nested_corresp
                    .iter()
                    .enumerate()
                    .find(|(i, corresp)| corresp.keys() == repetition_names)
                    .unwrap();

                // loop for each values of the repetition
                let mut repetitions = vec![];

                for (i, ident) in nested_correspondance
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
