use ariadne::{Color, ColorGenerator, Label, Report, ReportKind, Source};

use crate::lexer::{LexerError, Span};
use crate::parser::ParseError;

#[derive(Debug, Clone)]
pub enum DiagnosticType {
    Error,
    /* Warning,
    Note, */
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub message: String,
    pub labels: Vec<(String, Span)>,
    pub span: Span,
    pub kind: DiagnosticType,
}

impl From<ParseError> for Diagnostic {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::UnexpectedToken(token, expected) => Diagnostic {
                message: format!("Unexpected token: {:?}", token.token_type),
                labels: vec![(
                    format!("Expected one of {:?}", expected),
                    token.span.clone(),
                )],
                span: token.span,
                kind: DiagnosticType::Error,
            },
            ParseError::UnexpectedKeyword(token, expected) => Diagnostic {
                message: format!("Unexpected keyword: {:?}", token.token_type,),
                labels: vec![(
                    format!("Expected one of {:?}", expected),
                    token.span.clone(),
                )],
                span: token.span,
                kind: DiagnosticType::Error,
            },
            ParseError::Lexer(LexerError::UnknownToken(c, span)) => Diagnostic {
                message: format!("Lexer: Unknown token: {:?}", c),
                labels: vec![],
                span,
                kind: DiagnosticType::Error,
            },
            ParseError::MacroNoCorrespondance {
                macro_name,
                invoc_name,
                invoc_arg,
            } => {
                let mut labels = vec![
                    (format!("For this macro"), macro_name.clone()),
                    (format!("In this macro invocation"), invoc_name.clone()),
                ];

                if let Some(invoc_arg) = invoc_arg {
                    labels.push((format!("With this token"), invoc_arg.clone()));
                }

                Diagnostic {
                    message: format!("Macro: Nothing expected this token"),
                    labels,
                    span: macro_name.clone(),
                    kind: DiagnosticType::Error,
                }
            }
            ParseError::InvalidPrecedence(precedence, token) => Diagnostic {
                message: format!("Invalid precedence: {:?}", precedence),
                labels: vec![(
                    format!("Precedence must be between 0 and 9"),
                    token.span.clone(),
                )],
                span: token.span,
                kind: DiagnosticType::Error,
            },
            ParseError::IndentMismatch(got, expected, span) => Diagnostic {
                message: format!("Indent mismatch: got {}, expected {}", got, expected),
                labels: vec![
                    (format!("Expected indent level: {}", expected), span.clone()),
                    (format!("Got indent level: {}", got), span.clone()),
                ],
                span,
                kind: DiagnosticType::Error,
            },
            ParseError::UnexpectedEof(token_type) => Diagnostic {
                message: format!("Unexpected end of file"),
                labels: vec![(
                    format!("Expected token of type {:?}", token_type),
                    Span::default(),
                )],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            ParseError::LeftoverTokens(tokens) => Diagnostic {
                message: format!("Leftover tokens"),
                labels: vec![(format!("Expected end of file"), tokens[0].span.clone())],
                span: tokens[0].span.clone(),
                kind: DiagnosticType::Error,
            },
            ParseError::UnknownFile(file) => Diagnostic {
                message: format!("Unknown file: {:?}", file),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            ParseError::InternalError(message) => Diagnostic {
                message: format!("Internal error: {:?}", message),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            ParseError::ShortCircuit => Diagnostic {
                message: format!("Short circuit, should never be printed !"),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
        }
    }
}

impl Diagnostic {
    pub fn report(&self) {
        let mut colors = ColorGenerator::new();

        // Generate & choose some colours for each of our elements
        let red = Color::Fixed(9);

        let colors = vec![red, colors.next(), colors.next()];

        let file_name = self
            .span
            .file_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap();

        let mut builder = Report::build(ReportKind::Error, file_name, self.span.start)
            .with_message(self.message.clone());

        for (i, (message, span)) in self.labels.iter().enumerate() {
            let label_file_name = span.file_path.file_name().unwrap().to_str().unwrap();
            builder = builder.with_label(
                Label::new((label_file_name, span.start..span.end))
                    .with_message(message)
                    .with_color(colors[i]),
            );
        }

        builder
            .finish()
            .print((
                file_name,
                Source::from(std::fs::read_to_string(&self.span.file_path).unwrap_or_default()),
            ))
            .unwrap();

        println!("");
    }
}

#[derive(Debug, Default, Clone)]
pub struct Diagnostics(pub Vec<Diagnostic>);

impl Diagnostics {
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.0.push(diagnostic);
    }

    pub fn merge(&mut self, mut diagnostics: Diagnostics) {
        self.0.append(&mut diagnostics.0);
    }

    pub fn report(&self) {
        for diagnostic in &self.0 {
            diagnostic.report();
        }
    }

    pub fn return_if_err(&self) -> Result<(), Self> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(self.clone())
        }
    }
}

impl From<ParseError> for Diagnostics {
    fn from(err: ParseError) -> Self {
        let mut diagnostics = Diagnostics::default();
        diagnostics.push(Diagnostic::from(err));
        diagnostics
    }
}
