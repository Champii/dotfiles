use std::path::PathBuf;

use crate::lexer::{span::Span, Token, TokenType};

pub const KEYWORDS: [&str; 4] = ["if", "macro", "true", "false"];
pub const OPERATORS_CHARS: [char; 9] = ['+', '-', '*', '/', '=', '!', '<', '>', '$'];

#[derive(Debug)]
pub enum LexerError {
    UnknownToken(char, Span),
}

pub struct Lexer {
    file_path: PathBuf,
    input: String,
    position: usize,
    last_token: Option<Token>,
    /// true by default, is turned off for the parser unit tests
    add_empty_newline_at_end: bool,
}

impl Lexer {
    pub fn new(file_path: PathBuf, input: &str) -> Result<Self, LexerError> {
        Ok(Lexer {
            file_path,
            input: input.to_string(),
            position: 0,
            last_token: None,
            add_empty_newline_at_end: true,
        })
    }

    #[cfg(test)]
    pub fn with_newline_at_end(mut self, add_empty_newline_at_end: bool) -> Self {
        self.add_empty_newline_at_end = add_empty_newline_at_end;
        self
    }

    fn span(&self, len: usize) -> Span {
        Span {
            file_path: self.file_path.clone(),
            start: self.position,
            end: self.position + len,
        }
    }

    fn token(&self, token_type: TokenType, len: usize) -> Token {
        Token {
            token_type,
            span: self.span(len),
        }
    }

    pub fn next(&mut self) -> Result<Token, LexerError> {
        if self.prev_char() != '\n' && self.position != 0 || self.position == self.input.len() {
            self.skip_whitespace();
        } else {
            if let Some(token) = &self.last_token {
                match token.token_type {
                    TokenType::Indent(_) => (),
                    _ => {
                        let token = self.indent();

                        self.position = token.span.end;

                        self.last_token = Some(token.clone());

                        return Ok(token);
                    }
                }
            } else {
                let token = self.indent();

                self.position = token.span.end;

                self.last_token = Some(token.clone());

                return Ok(token);
            }
        }

        let token = match self.current_char() {
            '\n' => self.token(TokenType::Eol, 1),
            '-' if self.peek(1) == '>' => self.token(TokenType::Arrow, 2),
            '=' if self.peek(1) == '>' => self.token(TokenType::FatArrow, 2),
            '$' if self.peek(1).is_alphabetic() => self.macro_var(),
            '$' if self.peek(1) == '(' => self.token(TokenType::MacroRepeatOpen, 2),
            ')' if self.peek(1) == '*' => self.token(TokenType::MacroRepeatClose, 2),
            '%' if self.peek(1).is_alphabetic() => self.macro_invoc(),
            c if OPERATORS_CHARS.contains(&c) => self.operator(),
            '(' => self.token(TokenType::OpenParen, 1),
            ')' => self.token(TokenType::CloseParen, 1),
            ',' => self.token(TokenType::Coma, 1),
            ':' if self.peek(1) == ':' => self.token(TokenType::DoubleColon, 2),
            ':' => self.token(TokenType::Colon, 1),
            '\'' => self.token(TokenType::SimpleQuote, 1),
            '"' => self.token(TokenType::DoubleQuote, 1),
            c if c.is_alphabetic() => self.ident_or_keyword(),
            c if c.is_digit(10) => self.number(),
            '\0' => self.token(TokenType::Eof, 1),
            c => return Err(LexerError::UnknownToken(c, self.span(1))),
        };

        self.position = token.span.end;

        self.last_token = Some(token.clone());

        Ok(token)
    }

    pub fn collect(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next()?;

            if token.token_type == TokenType::Eof {
                break;
            }

            tokens.push(token);
        }

        if self.add_empty_newline_at_end {
            if let Some(token) = tokens.last() {
                if token.token_type != TokenType::Eol {
                    // Finish the current line
                    tokens.push(self.token(TokenType::Eol, 1));
                }
            }
            // add an empty line at the end of the file
            tokens.push(self.token(TokenType::Indent(0), 0));
            tokens.push(self.token(TokenType::Eol, 1));
        }

        tokens.push(self.token(TokenType::Eof, 0));

        Ok(tokens)
    }

    fn operator(&mut self) -> Token {
        let start = self.position;
        let mut end = self.position;

        while OPERATORS_CHARS.contains(&self.peek(end - start)) {
            end += 1;
        }

        if self.input[start..end] == *"=" {
            self.token(TokenType::Equal, 1)
        } else {
            self.token(
                TokenType::Operator(self.input[start..end].to_string()),
                end - start,
            )
        }
    }

    fn ident_or_keyword(&mut self) -> Token {
        let start = self.position;
        let mut end = self.position;

        while self.peek(end - start).is_alphanumeric() {
            end += 1;
        }

        let ident = self.input[start..end].to_string();

        if KEYWORDS.contains(&ident.as_str()) {
            self.token(TokenType::Keyword(ident), end - start)
        } else {
            self.token(TokenType::Ident(ident), end - start)
        }
    }

    fn macro_var(&mut self) -> Token {
        self.position += 1;
        let start = self.position;
        let mut end = self.position;

        while self.peek(end - start).is_alphanumeric() {
            end += 1;
        }

        self.token(
            TokenType::MacroVar(self.input[start..end].to_string()),
            end - start,
        )
    }

    fn macro_invoc(&mut self) -> Token {
        self.position += 1;
        let start = self.position;
        let mut end = self.position;

        while self.peek(end - start).is_alphanumeric() {
            end += 1;
        }

        self.token(
            TokenType::MacroInvoc(self.input[start..end].to_string()),
            end - start,
        )
    }

    fn number(&mut self) -> Token {
        let start = self.position;
        let mut end = self.position;

        while self.peek(end - start).is_digit(10) {
            end += 1;
        }

        self.token(
            TokenType::Number(self.input[start..end].to_string()),
            end - start,
        )
    }

    fn indent(&mut self) -> Token {
        let start = self.position;
        let mut end = self.position;

        while self.peek(end - start).is_whitespace() && self.peek(end - start) != '\n' {
            end += 1;
        }

        self.token(TokenType::Indent((end - start) as u8), end - start)
    }

    fn skip_whitespace(&mut self) {
        while self.current_char().is_whitespace() && self.current_char() != '\n' {
            self.position += 1;
        }
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }

    fn prev_char(&self) -> char {
        self.input
            .chars()
            .nth(self.position.saturating_sub(1))
            .unwrap_or('\0')
    }

    fn peek(&self, n: usize) -> char {
        self.input.chars().nth(self.position + n).unwrap_or('\0')
    }
}
