use std::{collections::HashSet, path::PathBuf};

use crate::{
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    Config,
};

use super::ParseError;

#[derive(Debug, Clone)]
pub struct ParseCtx {
    indent_level: u8,
    pub indent_step: u8,
    _diagnostics: Vec<String>,
    pub inside_argument_list: bool,
    pub files_map: HashSet<PathBuf>,
    pub current_file: Option<PathBuf>,
    pub config: Config,
    /// This is to handle nested fn type declarations
    pub is_inside_fn_type_decl: bool,
    pub disallowed_multiline_fn_call: bool,
}

impl ParseCtx {
    pub fn new(config: &Config) -> Self {
        ParseCtx {
            indent_step: 2,
            indent_level: 0,
            inside_argument_list: false,
            _diagnostics: Vec::new(),
            files_map: HashSet::new(),
            current_file: None,
            config: config.clone(),
            is_inside_fn_type_decl: false,
            disallowed_multiline_fn_call: false,
        }
    }

    pub fn indent(&mut self) {
        self.indent_level += self.indent_step;
    }

    pub fn indent_block<T, F>(&mut self, f: F) -> Result<T, Diagnostics>
    where
        F: FnOnce(&mut Self) -> Result<T, Diagnostics>,
    {
        self.indent();

        let res = f(self);

        self.dedent();

        res
    }

    pub fn dedent(&mut self) {
        self.indent_level -= self.indent_step;
    }

    pub fn consume_indent<'a>(&self, tokens: &'a [Token]) -> Result<&'a [Token], Diagnostics> {
        if tokens.is_empty() {
            return Err(ParseError::UnexpectedEof(TokenType::Indent(self.indent_level)).into());
        }

        if let TokenType::Indent(level) = tokens[0].token_type {
            if self.indent_level == level {
                return Ok(&tokens[1..]);
            } else {
                return Err(ParseError::IndentMismatch(
                    level,
                    self.indent_level,
                    tokens[0].span.clone(),
                )
                .into());
            }
        }

        return Err(ParseError::UnexpectedToken(
            tokens[0].clone(),
            vec![TokenType::Indent(self.indent_level)],
        )
        .into());
    }

    pub fn consume_indent_if_any<'a>(&self, tokens: &'a [Token]) -> (bool, &'a [Token]) {
        if let Ok(new_remaining_tokens) = self.consume_indent(tokens) {
            return (true, new_remaining_tokens);
        }

        (false, tokens)
    }

    pub fn deduce_indent_step(&mut self, tokens: &[Token]) {
        for token in tokens {
            match token.token_type {
                TokenType::Indent(level) => {
                    if level > 0 {
                        if level % 2 != 0 {
                            panic!("Indent level must be a multiple of 2");
                        }
                        self.indent_step = level;
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn indent_level(&self) -> u8 {
        self.indent_level
    }

    pub fn indent_step(&self) -> u8 {
        self.indent_step
    }

    pub fn add_file_relative(&mut self, name: String) {
        if let Some(current_path) = &self.current_file {
            let mut path = current_path.clone();

            path.pop();

            path.push(name + ".rk");

            self.files_map.insert(path.clone());

            self.current_file = Some(path);
        } else {
            self.files_map.insert(PathBuf::from(name.clone()));

            self.current_file = Some(PathBuf::from(name));
        }
    }

    /// This is to handle the spaced dot that closes argument list
    pub fn argument_list<'a, T, F>(&mut self, f: F) -> Result<(T, &'a [Token]), Diagnostics>
    where
        F: FnOnce(&mut Self) -> Result<(T, &'a [Token]), Diagnostics>,
    {
        let old_state = self.inside_argument_list;

        self.inside_argument_list = true;

        let res = f(self);

        self.inside_argument_list = old_state;
        res
    }

    pub fn argument_list_short_circuit(&mut self) -> Result<(), Diagnostics> {
        if !self.inside_argument_list {
            return Ok(());
        }

        self.inside_argument_list = false;

        Err(ParseError::ShortCircuit.into())
    }

    pub fn disallow_multiline_fn_call<T, F: FnOnce(&mut Self) -> Result<T, Diagnostics>>(
        &mut self,
        f: F,
    ) -> Result<T, Diagnostics> {
        let old_state = self.disallowed_multiline_fn_call;

        self.disallowed_multiline_fn_call = true;

        let res = f(self);

        self.disallowed_multiline_fn_call = old_state;
        res
    }
}
