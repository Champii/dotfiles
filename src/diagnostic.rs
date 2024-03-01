use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};

use crate::lexer::{LexerError, Span};
use crate::parser::ParseError;

pub enum DiagnosticType {
    Error,
    Warning,
    Note,
}

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
                message: format!("Unknown token: {:?}", c),
                labels: vec![],
                span,
                kind: DiagnosticType::Error,
            },
            _ => Diagnostic {
                message: format!("Unexpected error: {:?}", err),
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

        let file_name = self.span.file_path.file_name().unwrap().to_str().unwrap();

        let mut builder = Report::build(ReportKind::Error, file_name, self.span.start)
            .with_code(3)
            .with_message(self.message.clone());

        for (message, span) in &self.labels {
            let label_file_name = span.file_path.file_name().unwrap().to_str().unwrap();
            builder = builder.with_label(
                Label::new((label_file_name, span.start..span.end))
                    .with_message(message)
                    .with_color(colors.next()),
            );
        }

        builder
            .finish()
            .print((
                file_name,
                Source::from(std::fs::read_to_string(&self.span.file_path).unwrap()),
            ))
            .unwrap();
    }
}
