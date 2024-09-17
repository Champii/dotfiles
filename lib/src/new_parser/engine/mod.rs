use std::path::PathBuf;

use crate::{
    lexer::{Token, TokenType},
    Config,
};

mod and;
mod delimited;
mod fns;
mod followed;
mod indented;
mod iresult;
mod many;
mod map;
mod not;
mod opt;
mod or;
mod parse_error;
mod parser_trait;
mod preceded;
mod separated;
mod token_type;
mod tuples;

pub use and::*;
pub use delimited::*;
pub use followed::*;
pub use indented::*;
pub use iresult::*;
pub use many::*;
pub use map::*;
pub use not::*;
pub use opt::*;
pub use or::*;
pub use parse_error::*;
pub use parser_trait::*;
pub use preceded::*;
pub use separated::*;

pub type Input<'a> = ParseCtx<'a>;

#[derive(Clone, Debug)]
pub struct ParseCtx<'a> {
    pub tokens: &'a [Token],
    pub indent_level: usize,
    pub config: &'a Config,
    pub indent_step: usize,
    pub disallowed_multiline_fn_call: bool,
    pub inside_argument_list: bool,
    pub is_inside_fn_type_decl: bool,
}

impl ParseCtx<'_> {
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn consume(&self) -> Result<(Self, Token), ParseError> {
        if self.tokens.is_empty() {
            return Err(ParseError::UnexpectedEOF);
        }

        Ok((
            ParseCtx {
                tokens: &self.tokens[1..],
                ..*self
            },
            self.tokens[0].clone(),
        ))
    }

    pub fn indent(&self) -> Result<Self, ParseError> {
        Ok(ParseCtx {
            indent_level: self.indent_level + self.indent_step,
            ..*self
        })
    }

    pub fn dedent(&self) -> Result<Self, ParseError> {
        Ok(ParseCtx {
            indent_level: self.indent_level - self.indent_step,
            ..*self
        })
    }

    pub fn with_indent<'a, F, T>(self, f: F) -> IResult<'a, T>
    where
        F: FnOnce(Self) -> IResult<'a, T>,
    {
        let new_ctx = self.indent()?;
        let (new_ctx, res) = f(new_ctx)?;
        let new_ctx = new_ctx.dedent()?;

        Ok((new_ctx, res))
    }

    pub fn seek(&self) -> Result<Token, ParseError> {
        if self.tokens.is_empty() {
            return Err(ParseError::UnexpectedEOF);
        }

        Ok(self.tokens[0].clone())
    }

    pub fn seek_nth(&self, n: usize) -> Result<Token, ParseError> {
        if self.tokens.len() < n {
            return Err(ParseError::UnexpectedEOF);
        }

        Ok(self.tokens[n].clone())
    }

    pub fn from<'a>(tokens: &'a [Token], config: &'a Config) -> ParseCtx<'a> {
        let mut indent_step = Self::determine_indent_step(tokens, config);

        if indent_step == 0 {
            indent_step = 4;
        }

        ParseCtx {
            tokens,
            indent_level: 0,
            indent_step,
            config,
            disallowed_multiline_fn_call: false,
            inside_argument_list: false,
            is_inside_fn_type_decl: false,
        }
    }

    pub fn current_file_path(&self) -> PathBuf {
        self.tokens[0].span.file_path.clone()
    }

    pub fn sibling_module_filepath(&self, name: &str) -> Result<PathBuf, ParseError> {
        let mut base_path = self.current_file_path();
        base_path.pop(); // Remove the current file name to get the directory

        // First, check for "{}/{}.rk"
        let mut path = base_path.clone();
        path.push(format!("{}.rk", name));

        if path.exists() {
            return Ok(path);
        }

        // If not found, check for "{}/mod.rk" inside the "name" directory
        path = base_path.clone();
        path.push(name);
        path.push("mod.rk");

        if path.exists() {
            return Ok(path);
        }

        // If neither path exists, return an error
        Err(ParseError::UnknownFile(format!(
            "Neither '{}' nor '{}' exists",
            base_path.join(format!("{}.rk", name)).display(),
            base_path.join(name).join("mod.rk").display()
        )))
    }

    fn determine_indent_step(tokens: &[Token], _config: &Config) -> usize {
        let mut indent_step = 0;

        for token in tokens {
            if let TokenType::Indent(level) = token.token_type {
                if level > 0 {
                    if level % 2 != 0 {
                        panic!("Indentation level is not a multiple 2");
                    }

                    indent_step = level as usize;

                    break;
                }
            }
        }

        indent_step
    }

    pub fn disallow_multiline_fn_call(&self) -> Result<Self, ParseError> {
        Ok(ParseCtx {
            disallowed_multiline_fn_call: true,
            ..*self
        })
    }

    pub fn argument_list_short_circuit(&mut self) -> Result<(), ParseError> {
        if !self.inside_argument_list {
            return Ok(());
        }

        self.inside_argument_list = false;

        Err(ParseError::ShortCircuit)
    }
}

impl Copy for ParseCtx<'_> {}
