use crate::new_parser::*;
use crate::lexer::TokenType;

fn token_type_description(tt: &TokenType) -> String {
    match tt {
        TokenType::Ident(_) => "identifier".to_string(),
        TokenType::Type(_) => "type".to_string(),
        TokenType::Number(_) => "number".to_string(),
        TokenType::Float(_) => "float".to_string(),
        TokenType::Operator(s) => format!("operator '{}'", s),
        TokenType::Comment(_) => "comment".to_string(),
        TokenType::StuckOperator(s) => format!("operator '{}'", s),
        TokenType::NativeOperator(s) => format!("native operator '{}'", s),
        TokenType::Keyword(s) => format!("keyword '{}'", s),
        TokenType::MacroVar(_) => "macro variable".to_string(),
        TokenType::MacroInvoc(_) => "macro invocation".to_string(),
        TokenType::MacroRepeatOpen => "'$('".to_string(),
        TokenType::MacroRepeatClose => "')'".to_string(),
        TokenType::Equal => "'='".to_string(),
        TokenType::OpenParen => "'('".to_string(),
        TokenType::CloseParen => "')'".to_string(),
        TokenType::OpenBracket => "'['".to_string(),
        TokenType::CloseBracket => "']'".to_string(),
        TokenType::Char(_) => "character literal".to_string(),
        TokenType::String(_) => "string literal".to_string(),
        TokenType::Arrow => "'->'".to_string(),
        TokenType::FatArrow => "'=>'".to_string(),
        TokenType::Coma => "','".to_string(),
        TokenType::Colon => "':'".to_string(),
        TokenType::DoubleColon => "'::'".to_string(),
        TokenType::Dot => "'.'".to_string(),
        TokenType::DoubleDot => "'..'".to_string(),
        TokenType::SpacedDot => "spaced dot".to_string(),
        TokenType::Arobase => "'@'".to_string(),
        TokenType::Interogation => "'?'".to_string(),
        TokenType::Indent(level) => format!("indent level {}", level),
        TokenType::Underscore => "'_'".to_string(),
        TokenType::Eol => "newline".to_string(),
        TokenType::Eof => "end of file".to_string(),
    }
}

macro_rules! token {
    ($stream:ident, $pat:pat => $result:expr) => {{
        let ($stream, token) = $stream.consume()?;

        if let $pat = &token.token_type {
            Ok(($stream, $result))
        } else {
            Err(ParseError::UnexpectedToken(
                token_type_description(&token.token_type),
                token.clone(),
            ))
        }
    }};
}

macro_rules! token_with_span {
    ($stream:ident, $span:ident, $pat:pat => $result:expr) => {{
        let ($stream, token) = $stream.consume()?;

        let $span = token.span.clone();

        if let $pat = token.token_type {
            Ok(($stream, $result))
        } else {
            Err(ParseError::UnexpectedToken(
                token_type_description(&token.token_type),
                token,
            ))
        }
    }};
}

pub fn ident_token(stream: Input) -> IResult<Ident> {
    token_with_span!(stream, span, TokenType::Ident(name) =>
        Ident { name, span }
    )
}

pub fn indent(stream: Input) -> IResult<()> {
    token!(stream, TokenType::Indent(level) => {
        if *level as usize != stream.indent_level {
            return Err(ParseError::UnexpectedIndent(*level));
        }


    })
}

// used to get any indent level
pub fn indent_token(stream: Input) -> IResult<u8> {
    token!(stream, TokenType::Indent(level) => {
        *level
    })
}

pub fn boolean(stream: Input) -> IResult<bool> {
    TokenType::Keyword("true".to_string())
        .map(|_| true)
        .or(TokenType::Keyword("false".to_string()).map(|_| false))
        .process(stream)
}

pub fn int(stream: Input) -> IResult<u64> {
    token!(stream, TokenType::Number(value) => {
        value.parse().unwrap()
    })
}

pub fn float(stream: Input) -> IResult<f64> {
    token!(stream, TokenType::Float(value) => {
        value.parse().unwrap()
    })
}

pub fn string(stream: Input) -> IResult<String> {
    token!(stream, TokenType::String(value) => {
        value.clone()
    })
}

pub fn char(stream: Input) -> IResult<String> {
    token!(stream, TokenType::Char(value) => {
        value.clone()
    })
}

pub fn macro_invoc_token(stream: Input) -> IResult<String> {
    token!(stream, TokenType::MacroInvoc(name) => {
        name.to_string()
    })
}

pub fn operator_token(stream: Input) -> IResult<Operator> {
    token_with_span!(stream, span, TokenType::Operator(name) => {
        Operator {
            value: name.clone(),
            span,
        }
    })
}

pub fn stuck_operator_token(stream: Input) -> IResult<Operator> {
    token_with_span!(stream, span, TokenType::StuckOperator(name) => {
        Operator {
            value: name.clone(),
            span,
        }
    })
}

pub fn operator(stream: Input) -> IResult<Operator> {
    operator_token.or(stuck_operator_token).process(stream)
}

pub fn type_token(stream: Input) -> IResult<String> {
    token!(stream, TokenType::Type(name) => {
        name.clone()
    })
}

pub fn native_operator(stream: Input) -> IResult<NativeOperator> {
    token_with_span!(stream, span, TokenType::NativeOperator(name) => {
        NativeOperator {
            name: name.clone(),
            span,
        }
    })
}
