use std::path::PathBuf;

use crate::lexer::{span::Span, Token, TokenType};

pub const KEYWORDS: [&str; 23] = [
    "struct", "enum", "trait", "impl", "if", "then", "else", "for", "in", "while", "loop", "macro",
    "true", "false", "return", "continue", "break", "infix", "mod", "extern", "match", "unsafe",
    "type",
];
pub const OPERATORS_CHARS: [char; 10] = ['+', '-', '*', '/', '=', '!', '<', '>', '$', '|'];

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

    pub fn next(&mut self) -> Result<Token, LexerError> {
        let token = self.match_current_char()?;

        self.position = token.span.end;

        self.last_token = Some(token.clone());

        if let TokenType::Comment(_) = token.token_type {
            return self.next();
        }

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

    fn match_current_char(&mut self) -> Result<Token, LexerError> {
        // Handle the space dot
        if self.current_char() == ' '
            && self.peek(1) == '.'
            && self.peek(2) != '.'
            // special case to differenciate the dot operator
            && self.peek(2) != ' '
        {
            return Ok(self.token(TokenType::SpacedDot, 2));
        }

        // Skip whitespace except when in the start of the file for indentation
        if self.prev_char() != '\n' && self.position != 0 || self.position == self.input.len() {
            self.skip_whitespace();
        } else if let Some(token) = &self.last_token {
            match token.token_type {
                TokenType::Indent(_) => (),
                _ => {
                    return Ok(self.indent());
                }
            }
        } else {
            return Ok(self.indent());
        }

        let token = match self.current_char() {
            '\n' => self.token(TokenType::Eol, 1),
            '-' if self.peek(1) == '>' => self.token(TokenType::Arrow, 2),
            '=' if self.peek(1) == '>' => self.token(TokenType::FatArrow, 2),
            '$' if self.peek(1).is_alphabetic() => self.macro_var(),
            '$' if self.peek(1) == '(' => self.token(TokenType::MacroRepeatOpen, 2),
            ')' if self.peek(1) == '*' => self.token(TokenType::MacroRepeatClose, 2),
            '%' if self.peek(1).is_alphabetic() && self.peek(1).is_lowercase() => {
                self.macro_invoc()
            }
            '%' if self.peek(1).is_alphabetic() && self.peek(1).is_uppercase() => {
                self.native_operator()
            }
            '/' if self.peek(1) == '/' => self.comment_eol(),
            '/' if self.peek(1) == '*' => self.comment(),
            c if OPERATORS_CHARS.contains(&c) => self.operator(),
            '(' => self.token(TokenType::OpenParen, 1),
            ')' => self.token(TokenType::CloseParen, 1),
            '[' => self.token(TokenType::OpenBracket, 1),
            ']' => self.token(TokenType::CloseBracket, 1),
            ',' => self.token(TokenType::Coma, 1),
            ':' if self.peek(1) == ':' => self.token(TokenType::DoubleColon, 2),
            ':' => self.token(TokenType::Colon, 1),
            '.' if self.peek(1) == ' ' => self.operator(),
            '.' if self.peek(1) == '.' => self.token(TokenType::DoubleDot, 2),
            '.' => self.token(TokenType::Dot, 1),
            '?' => self.token(TokenType::Interogation, 1),
            '\'' => self.char(),
            '"' => self.string(),
            '@' => self.token(TokenType::Arobase, 1),
            '_' => self.token(TokenType::Underscore, 1),
            c if c.is_alphabetic() => self.ident_or_keyword_or_type(),
            c if c.is_ascii_digit() => self.number(),
            '\0' => self.token(TokenType::Eof, 1),
            c => return Err(LexerError::UnknownToken(c, self.span(1))),
        };

        Ok(token)
    }

    fn operator(&self) -> Token {
        let start = self.position;
        let mut end = self.position;

        if self.peek(0) == '.' {
            end += 1;
        } else {
            while OPERATORS_CHARS.contains(&self.peek(end - start)) {
                end += 1;
            }
        }

        if self.input[start..end] == *"=" {
            self.token(TokenType::Equal, 1)
        } else if self.input.len() > 2 && self.input[0..2] == *". " {
            self.token(TokenType::Operator(self.input[0..1].to_string()), 2)
        } else if self.input.len() > end + 1
            && (self.input[end..end + 1] == *" " || self.input[end..end + 1] == *"\n")
        {
            self.token(
                TokenType::Operator(self.input[start..end].to_string()),
                end - start,
            )
        } else {
            self.token(
                TokenType::StuckOperator(self.input[start..end].to_string()),
                end - start,
            )
        }
    }

    fn ident_or_keyword_or_type(&self) -> Token {
        let start = self.position;
        let mut end = self.position;

        while self.peek(end - start).is_alphanumeric() || self.peek(end - start) == '_' {
            end += 1;
        }

        let ident = self.input[start..end].to_string();

        if self.current_char().is_uppercase() {
            self.token(TokenType::Type(ident), end - start)
        } else if KEYWORDS.contains(&ident.as_str()) {
            self.token(TokenType::Keyword(ident), end - start)
        } else {
            self.token(TokenType::Ident(ident), end - start)
        }
    }

    fn macro_var(&self) -> Token {
        let start = self.position;
        let mut end = self.position;

        while self.peek(end - start + 1).is_alphanumeric() || self.peek(end - start + 1) == '_' {
            end += 1;
        }

        self.token(
            TokenType::MacroVar(self.input[start + 1..end + 1].to_string()),
            end - start + 1,
        )
    }

    fn comment_eol(&self) -> Token {
        let start = self.position;
        let mut end = self.position;

        // consume the '//'
        end += 2;

        while self.peek(end - start) != '\n' {
            end += 1;
        }

        self.token(TokenType::Comment(String::new()), end - start)
    }

    fn comment(&self) -> Token {
        let start = self.position;
        let mut end = self.position;

        // consume the '/*'
        end += 2;

        while self.peek(end - start) != '*' && self.peek(end - start + 1) != '/' {
            end += 1;
        }

        // consume the '*/'
        end += 2;

        self.token(TokenType::Comment(String::new()), end - start)
    }

    fn macro_invoc(&self) -> Token {
        let start = self.position;
        let mut end = self.position;

        while self.peek(end - start + 1).is_alphanumeric() || self.peek(end - start + 1) == '_' {
            end += 1;
        }

        self.token(
            TokenType::MacroInvoc(self.input[start + 1..end + 1].to_string()),
            end - start + 1,
        )
    }

    fn native_operator(&self) -> Token {
        let start = self.position;
        let mut end = self.position;

        while self.peek(end - start + 1).is_alphabetic() || self.peek(end - start + 1) == '_' {
            end += 1;
        }

        self.token(
            TokenType::NativeOperator(self.input[start + 1..end + 1].to_string()),
            end - start + 1,
        )
    }

    fn number(&self) -> Token {
        let start = self.position;
        let mut end = self.position;

        while self.peek(end - start).is_ascii_digit() {
            end += 1;
        }

        if self.peek(end - start) == '.' {
            let old_end = end;
            end += 1;

            while self.peek(end - start).is_ascii_digit() {
                end += 1;
            }
            if end > old_end + 1 {
                return self.token(
                    TokenType::Float(self.input[start..end].to_string()),
                    end - start,
                );
            } else {
                end = old_end;
            }
        }

        self.token(
            TokenType::Number(self.input[start..end].to_string()),
            end - start,
        )
    }

    fn char(&self) -> Token {
        let start = self.position;
        let mut end = self.position;

        // We deliberately allow multi-character literals here to handle escaped chars
        // The other errors should be catched in the parser
        while self.peek(end - start + 1) != '\'' {
            end += 1;
        }

        self.token(
            TokenType::Char(self.input[start + 1..end + 1].to_owned()),
            end - start + 2,
        )
    }

    fn string(&self) -> Token {
        let start = self.position;
        let mut end = self.position;

        while self.peek(end - start + 1) != '"' {
            end += 1;
        }

        self.token(
            TokenType::String(self.input[start + 1..end + 1].to_string()),
            end - start + 2,
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
