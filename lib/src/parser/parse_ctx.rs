use std::{collections::HashSet, path::PathBuf};

use crate::{
    lexer::{Token, TokenType},
    Config,
};

use super::ParseError;

#[derive(Debug, Clone)]
pub struct ParseCtx {
    indent_level: u8,
    indent_step: u8,
    _diagnostics: Vec<String>,
    pub inside_argument_list: Vec<bool>,
    pub files_map: HashSet<PathBuf>,
    pub current_file: Option<PathBuf>,
    pub config: Config,
    /// This is to handle nested fn type declarations
    pub is_inside_fn_type_decl: bool,
}

impl ParseCtx {
    pub fn new(config: &Config) -> Self {
        ParseCtx {
            indent_step: 2,
            indent_level: 0,
            inside_argument_list: Vec::new(),
            _diagnostics: Vec::new(),
            files_map: HashSet::new(),
            current_file: None,
            config: config.clone(),
            is_inside_fn_type_decl: false,
        }
    }

    pub fn indent(&mut self) {
        self.indent_level += self.indent_step;
    }

    pub fn dedent(&mut self) {
        self.indent_level -= self.indent_step;
    }

    pub fn consume_indent<'a>(&self, tokens: &'a [Token]) -> Result<&'a [Token], ParseError> {
        if tokens.is_empty() {
            return Err(ParseError::UnexpectedEof(TokenType::Indent(
                self.indent_level,
            )));
        }

        if let TokenType::Indent(level) = tokens[0].token_type {
            if self.indent_level == level {
                return Ok(&tokens[1..]);
            } else {
                return Err(ParseError::IndentMismatch(level, self.indent_level));
            }
        }

        return Err(ParseError::UnexpectedToken(
            tokens[0].clone(),
            vec![TokenType::Indent(self.indent_level)],
        ));
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
}
