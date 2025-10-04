use ariadne::{Color, ColorGenerator, Label, Report, ReportKind, Source};

use crate::lexer::{LexerError, Span};
use crate::new_parser::ParseError;

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
            ParseError::UnexpectedToken(_expected_desc, got_token) => {
                let got_display = if got_token.token_type.to_string().is_empty() {
                    format!("end of file")
                } else {
                    format!("'{}'", got_token.token_type.to_string())
                };

                Diagnostic {
                    message: format!("Unexpected token: {}", got_display),
                    labels: vec![(
                        format!("Unexpected {}", got_display),
                        got_token.span.clone()
                    )],
                    span: got_token.span,
                    kind: DiagnosticType::Error,
                }
            },
            /* ParseError::UnexpectedKeyword(token, expected) => Diagnostic {
                message: format!("Unexpected keyword: {:?}", token.token_type,),
                labels: vec![(
                    format!("Expected one of {:?}", expected),
                    token.span.clone(),
                )],
                span: token.span,
                kind: DiagnosticType::Error,
            }, */
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
                    ("For this macro".to_string(), macro_name.clone()),
                    ("In this macro invocation".to_string(), invoc_name.clone()),
                ];

                if let Some(invoc_arg) = invoc_arg {
                    labels.push(("With this token".to_string(), invoc_arg.clone()));
                }

                Diagnostic {
                    message: "Macro: Nothing expected this token".to_string(),
                    labels,
                    span: macro_name.clone(),
                    kind: DiagnosticType::Error,
                }
            }
            /* ParseError::InvalidPrecedence(precedence, token) => Diagnostic {
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
            }, */
            ParseError::UnexpectedEOF => Diagnostic {
                message: "Unexpected end of file".to_string(),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            /* ParseError::LeftoverTokens(tokens) => Diagnostic {
                message: format!("Leftover tokens"),
                labels: vec![(format!("Expected end of file"), tokens[0].span.clone())],
                span: tokens[0].span.clone(),
                kind: DiagnosticType::Error,
            }, */
            ParseError::UnknownFile(file) => Diagnostic {
                message: format!("Unknown file: {:?}", file),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            ParseError::UnexpectedIndent(level) => Diagnostic {
                message: format!("Unexpected indent level: {level}"),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            ParseError::ExpectedOneOrMore => Diagnostic {
                message: "Expected one or more".to_string(),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            ParseError::Fail => Diagnostic {
                message: "Fail".to_string(),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            ParseError::ShortCircuit => Diagnostic {
                message: "Short circuit, should never be printed !".to_string(),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            ParseError::AssertFailed => Diagnostic {
                message: "Assert failed".to_string(),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            ParseError::WithContext { context, error } => {
                // Build the full context chain by collecting all contexts
                // and find the innermost non-context error
                let mut context_chain = vec![context.clone()];
                let mut current_error = error.as_ref();

                // Unwrap all nested WithContext layers
                loop {
                    match current_error {
                        ParseError::WithContext { context: inner_context, error: inner_error } => {
                            context_chain.push(inner_context.clone());
                            current_error = inner_error.as_ref();
                        }
                        _ => break,
                    }
                }

                // Now current_error is the innermost non-context error
                // Get its base message and span
                let (base_message, span) = match current_error {
                    ParseError::UnexpectedToken(_expected_desc, got_token) => {
                        let got_display = if got_token.token_type.to_string().is_empty() {
                            format!("end of file")
                        } else {
                            format!("'{}'", got_token.token_type.to_string())
                        };
                        (
                            format!("Unexpected token: {}", got_display),
                            got_token.span.clone(),
                        )
                    },
                    ParseError::UnexpectedEOF => {
                        ("Unexpected end of file".to_string(), Span::default())
                    },
                    ParseError::UnexpectedIndent(level) => {
                        (format!("Unexpected indent level: {}", level), Span::default())
                    },
                    _ => ("Parse error".to_string(), Span::default()),
                };

                // Create diagnostic with context
                let message = format!(
                    "{}\n\nParsing context (innermost first):\n{}",
                    base_message,
                    context_chain.iter()
                        .enumerate()
                        .map(|(i, ctx)| format!("  {}. {}", i + 1, ctx))
                        .collect::<Vec<_>>()
                        .join("\n")
                );

                // Get the label from the innermost error
                let labels = match current_error {
                    ParseError::UnexpectedToken(_expected_desc, got_token) => {
                        let got_display = if got_token.token_type.to_string().is_empty() {
                            format!("end of file")
                        } else {
                            format!("'{}'", got_token.token_type.to_string())
                        };
                        vec![(
                            format!("Unexpected {}", got_display),
                            got_token.span.clone()
                        )]
                    },
                    _ => vec![],
                };

                Diagnostic {
                    message,
                    labels,
                    span,
                    kind: DiagnosticType::Error,
                }
            },
            /* ParseError::InternalError(message) => Diagnostic {
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
            ParseError::ExpectedType(span) => Diagnostic {
                message: format!("Expected a type"),
                labels: vec![(format!("But got"), span.clone())],
                span,
                kind: DiagnosticType::Error,
            },
            ParseError::InvalidLHS(span) => Diagnostic {
                message: format!("Malformed assignment"),
                labels: vec![(format!("Invalid left-hand side"), span.clone())],
                span,
                kind: DiagnosticType::Error,
            },
            ParseError::InvalidVariant(enum_name, variant) => Diagnostic {
                message: format!("Invalid variant"),
                labels: vec![
                    (format!("In this enum"), enum_name.clone()),
                    (format!("Expected a variant"), variant.clone()),
                ],
                span: enum_name,
                kind: DiagnosticType::Error,
            },
            ParseError::InvalidType(span) => Diagnostic {
                message: format!("Invalid type"),
                labels: vec![(format!("Expected a type"), span.clone())],
                span,
                kind: DiagnosticType::Error,
            }, */
        }
    }
}

impl Diagnostic {
    pub fn report(&self) {
        let mut colors = ColorGenerator::new();

        // Generate & choose some colours for each of our elements
        let red = Color::Fixed(9);

        let colors = [red, colors.next(), colors.next()];

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

        println!();
    }
}

#[derive(Debug, Default, Clone)]
pub struct Diagnostics(pub Vec<Diagnostic>);

impl Diagnostics {
    pub fn push<T: Into<Diagnostic>>(&mut self, diagnostic: T) -> Self {
        let diagnostic = diagnostic.into();

        self.0.push(diagnostic);

        self.clone()
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

/* // Old
impl From<crate::parser::ParseError> for Diagnostics {
    fn from(err: crate::parser::ParseError) -> Self {
        let mut diagnostics = Diagnostics::default();

        diagnostics.push(Diagnostic::from(err));

        diagnostics
    }
}

impl From<crate::parser::ParseError> for Diagnostic {
    fn from(err: crate::parser::ParseError) -> Self {
        match err {
            crate::parser::ParseError::UnexpectedToken(got, expected) => Diagnostic {
                message: format!("Unexpected token: {:?}", got),
                labels: vec![(format!("Expected {:?}", expected), got.span.clone())],
                span: got.span,
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::UnexpectedKeyword(token, expected) => Diagnostic {
                message: format!("Unexpected keyword: {:?}", token.token_type,),
                labels: vec![(
                    format!("Expected one of {:?}", expected),
                    token.span.clone(),
                )],
                span: token.span,
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::Lexer(LexerError::UnknownToken(c, span)) => Diagnostic {
                message: format!("Lexer: Unknown token: {:?}", c),
                labels: vec![],
                span,
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::MacroNoCorrespondance {
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
            crate::parser::ParseError::InvalidPrecedence(precedence, token) => Diagnostic {
                message: format!("Invalid precedence: {:?}", precedence),
                labels: vec![(
                    format!("Precedence must be between 0 and 9"),
                    token.span.clone(),
                )],
                span: token.span,
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::IndentMismatch(got, expected, span) => Diagnostic {
                message: format!("Indent mismatch: got {}, expected {}", got, expected),
                labels: vec![
                    (format!("Expected indent level: {}", expected), span.clone()),
                    (format!("Got indent level: {}", got), span.clone()),
                ],
                span,
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::UnexpectedEof(_token) => Diagnostic {
                message: format!("Unexpected end of file"),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::LeftoverTokens(tokens) => Diagnostic {
                message: format!("Leftover tokens"),
                labels: vec![(format!("Expected end of file"), tokens[0].span.clone())],
                span: tokens[0].span.clone(),
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::UnknownFile(file) => Diagnostic {
                message: format!("Unknown file: {:?}", file),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::InternalError(message) => Diagnostic {
                message: format!("Internal error: {:?}", message),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::ShortCircuit => Diagnostic {
                message: format!("Short circuit, should never be printed !"),
                labels: vec![],
                span: Span::default(),
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::ExpectedType(span) => Diagnostic {
                message: format!("Expected a type"),
                labels: vec![(format!("But got"), span.clone())],
                span,
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::InvalidLHS(span) => Diagnostic {
                message: format!("Malformed assignment"),
                labels: vec![(format!("Invalid left-hand side"), span.clone())],
                span,
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::InvalidVariant(enum_name, variant) => Diagnostic {
                message: format!("Invalid variant"),
                labels: vec![
                    (format!("In this enum"), enum_name.clone()),
                    (format!("Expected a variant"), variant.clone()),
                ],
                span: enum_name,
                kind: DiagnosticType::Error,
            },
            crate::parser::ParseError::InvalidType(span) => Diagnostic {
                message: format!("Invalid type"),
                labels: vec![(format!("Expected a type"), span.clone())],
                span,
                kind: DiagnosticType::Error,
            },
        }
    }
} */
