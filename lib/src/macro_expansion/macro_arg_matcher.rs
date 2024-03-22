use crate::{
    ast::{Ident, MacroFragment},
    lexer::{Span, Token, TokenType},
    parser::ParseError,
};

use super::correspondances::Correspondance;

#[derive(Clone, Debug)]
struct MacroThread<'a> {
    pub args: &'a [Token],
    pub tokens: Vec<MacroFragment>,
    pub correspondances: Correspondance,
}

#[derive(Debug)]
pub struct MacroArgMatcher<'a> {
    threads: Vec<MacroThread<'a>>,
    must_be_completed: bool,
}

impl<'a> MacroArgMatcher<'a> {
    pub fn new(args: &'a [Token], thread: Vec<MacroFragment>) -> Self {
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
        let mut new_threads = self.threads.clone();

        while !has_one_solution(new_threads.clone(), self.must_be_completed)
            && !new_threads.is_empty()
        {
            let mut new_new_threads = vec![];

            for thread in &mut new_threads {
                if let Some(arg) = thread.args.get(0) {
                    let tokens = &thread.tokens;
                    if tokens.is_empty() {
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
                        return Ok((thread.correspondances.clone(), thread.args));
                    }
                }
            }
            self.threads = new_new_threads.clone();
            new_threads = new_new_threads.clone();
        }

        if let Some(thread) = get_correspondances_thread(new_threads.clone()) {
            Ok((thread.correspondances.clone(), thread.args))
        } else {
            Err(ParseError::MacroNoCorrespondance(Ident {
                name: "macro".to_string(),
                span: Span::default(),
            }))
        }
    }
}

fn remaining_is_all_repetition_or_empty(tokens: &[MacroFragment]) -> bool {
    if tokens.is_empty() {
        return true;
    }

    tokens.iter().all(|token| {
        if let MacroFragment::Repetition(_) = token {
            true
        } else {
            false
        }
    })
}

fn has_one_solution(threads: Vec<MacroThread>, must_be_completed: bool) -> bool {
    threads.iter().any(|thread| {
        (must_be_completed
            && (remaining_is_all_repetition_or_empty(&thread.tokens) && thread.args.is_empty()))
            || (!must_be_completed
                && (remaining_is_all_repetition_or_empty(&thread.tokens) || thread.args.is_empty()))
    })
}

fn get_correspondances_thread<'a>(threads: Vec<MacroThread<'a>>) -> Option<MacroThread<'a>> {
    if let Some(found) = threads.iter().find(|thread| {
        remaining_is_all_repetition_or_empty(&thread.tokens) && thread.args.is_empty()
    }) {
        return Some(found.clone());
    } else if let Some(found) = threads.iter().find(|thread| thread.args.is_empty()) {
        return Some(found.clone());
    } else if let Some(found) = threads
        .iter()
        .find(|thread| remaining_is_all_repetition_or_empty(&thread.tokens))
    {
        return Some(found.clone());
    } else {
        None
    }
}
