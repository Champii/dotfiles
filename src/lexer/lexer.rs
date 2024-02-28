use crate::lexer::span::Span;
use crate::lexer::{Token, TokenType};

pub const KEYWORDS: [&str; 2] = ["if", "macro"];
pub const OPERATORS_CHARS: [char; 9] = ['+', '-', '*', '/', '=', '!', '<', '>', '$'];

#[derive(Debug)]
pub enum LexerError {
    UnknownToken(char),
}

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Result<Self, LexerError> {
        Ok(Lexer {
            input: input.to_string(),
            position: 0,
        })
    }

    fn token(&self, token_type: TokenType, len: usize) -> Token {
        Token {
            token_type,
            span: Span {
                start: self.position,
                end: self.position + len,
            },
        }
    }

    pub fn next(&mut self) -> Result<Token, LexerError> {
        if self.prev_char() != '\n' {
            self.skip_whitespace();
        }

        let token = match self.current_char() {
            '\n' => self.token(TokenType::Eol, 1),
            c if c.is_whitespace() && self.prev_char() == '\n' => self.indent(),
            '-' if self.peek(1) == '>' => self.token(TokenType::Arrow, 2),
            '=' if self.peek(1) == '>' => self.token(TokenType::FatArrow, 2),
            c if OPERATORS_CHARS.contains(&c) => self.operator(),
            '(' => self.token(TokenType::OpenParen, 1),
            ')' => self.token(TokenType::CloseParen, 1),
            ',' => self.token(TokenType::Coma, 1),
            ':' => self.token(TokenType::Colon, 1),
            c if c.is_alphabetic() => self.ident_or_keyword(),
            c if c.is_digit(10) => self.number(),
            '\0' => self.token(TokenType::Eof, 1),
            c => return Err(LexerError::UnknownToken(c)),
        };

        self.position = token.span.end;

        Ok(token)
    }

    pub fn collect(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next()?;

            if token.token_type == TokenType::Eof {
                tokens.push(token);

                break;
            }

            tokens.push(token);
        }

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

        while self.peek(end - start).is_whitespace() {
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
