use crate::lexer::{Span, Token};

#[derive(Debug)]
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
        }
    }

    /// Choose the "better" error to report between two errors.
    /// The error that occurred later in the input (higher position) is considered better.
    pub fn choose_better(self, other: ParseError) -> ParseError {
        // Special cases: ShortCircuit and Fail should never be reported
        match (&self, &other) {
            (ParseError::ShortCircuit, _) => return other,
            (_, ParseError::ShortCircuit) => return self,
            (ParseError::Fail, _) => return other,
            (_, ParseError::Fail) => return self,
            _ => {}
        }

        // Compare positions - prefer the error that occurred later
        if self.position() >= other.position() {
            self
        } else {
            other
        }
    }
}
