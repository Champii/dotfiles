use crate::lexer::{Span, Token};

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken(String, Token), // expected, got
    UnexpectedEOF,
    UnknownFile(String),
    Lexer(crate::lexer::LexerError),
    UnexpectedIndent(u8),
    ExpectedOneOrMore,
    MacroNoCorrespondance {
        macro_name: Span,
        invoc_name: Span,
        invoc_arg: Option<Span>,
    },
    // used to short-circuit the parser
    Fail,
    ShortCircuit, // should not be bubbled up to the user
    AssertFailed,
    // Wraps an error with context about what was being parsed
    WithContext {
        context: String,
        error: Box<ParseError>,
    },
}

impl ParseError {
    pub fn discriminant(&self) -> &'static str {
        match self {
            ParseError::UnexpectedToken(_, _) => "UnexpectedToken",
            ParseError::UnexpectedEOF => "UnexpectedEOF",
            ParseError::UnknownFile(_) => "UnknownFile",
            ParseError::Lexer(_) => "Lexer",
            ParseError::UnexpectedIndent(_) => "UnexpectedIndent",
            ParseError::ExpectedOneOrMore => "ExpectedOneOrMore",
            ParseError::MacroNoCorrespondance { .. } => "MacroNoCorrespondance",
            ParseError::Fail => "Fail",
            ParseError::ShortCircuit => "ShortCircuit",
            ParseError::AssertFailed => "AssertFailed",
            ParseError::WithContext { .. } => "WithContext",
        }
    }

    /// Get the position (span) of this error for comparison purposes.
    /// Returns the start position of the error's span.
    /// Errors that occurred later in the input are considered "better" to report.
    pub fn position(&self) -> usize {
        match self {
            ParseError::UnexpectedToken(_, token) => token.span.start,
            ParseError::UnexpectedEOF => usize::MAX, // EOF errors are always at the end
            ParseError::MacroNoCorrespondance { invoc_name, .. } => invoc_name.start,
            ParseError::UnexpectedIndent(_) => 0,
            ParseError::ExpectedOneOrMore => 0,
            ParseError::UnknownFile(_) => 0,
            ParseError::Lexer(_) => 0,
            ParseError::Fail => 0,
            ParseError::ShortCircuit => 0,
            ParseError::AssertFailed => 0,
            ParseError::WithContext { error, .. } => error.position(),
        }
    }

    /// Add context to this error about what was being parsed
    pub fn with_context(self, context: impl Into<String>) -> ParseError {
        ParseError::WithContext {
            context: context.into(),
            error: Box::new(self),
        }
    }

    /// Get all context strings from this error and its nested errors
    pub fn get_context_chain(&self) -> Vec<String> {
        match self {
            ParseError::WithContext { context, error } => {
                let mut chain = vec![context.clone()];
                chain.extend(error.get_context_chain());
                chain
            }
            _ => vec![],
        }
    }

    /// Get the depth of context (number of WithContext wrappers)
    fn context_depth(&self) -> usize {
        match self {
            ParseError::WithContext { error, .. } => 1 + error.context_depth(),
            _ => 0,
        }
    }

    /// Choose the "better" error to report between two errors.
    /// Prefers errors that occurred later in the input, but if positions are equal,
    /// prefers errors with more context (deeper in the parsing tree).
    pub fn choose_better(self, other: ParseError) -> ParseError {
        // Special cases: ShortCircuit and Fail should never be reported
        match (&self, &other) {
            (ParseError::ShortCircuit, _) => return other,
            (_, ParseError::ShortCircuit) => return self,
            (ParseError::Fail, _) => return other,
            (_, ParseError::Fail) => return self,
            _ => {}
        }

        let self_pos = self.position();
        let other_pos = other.position();

        // Compare positions first - prefer the error that occurred later
        if self_pos > other_pos {
            return self;
        } else if other_pos > self_pos {
            return other;
        }

        // Positions are equal - prefer the error with more context
        let self_depth = self.context_depth();
        let other_depth = other.context_depth();

        if self_depth >= other_depth {
            self
        } else {
            other
        }
    }
}
