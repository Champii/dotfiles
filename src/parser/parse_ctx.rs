use crate::lexer::{Token, TokenType};

use super::ParseError;

#[derive(Debug, Clone)]
pub struct ParseCtx {
    indent_level: u8,
    indent_step: u8,
    _diagnostics: Vec<String>,
}

impl ParseCtx {
    pub fn new() -> Self {
        ParseCtx {
            indent_step: 2,
            indent_level: 0,
            _diagnostics: Vec::new(),
        }
    }

    pub fn indent(&mut self) {
        self.indent_level += self.indent_step;
    }

    pub fn dedent(&mut self) {
        self.indent_level -= self.indent_step;
    }

    pub fn consume_indent<'a>(&self, tokens: &'a [Token]) -> Result<&'a [Token], ParseError> {
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
}
