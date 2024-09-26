use crate::new_parser::*;

macro_rules! token {
    ($stream:ident, $pat:pat => $result:expr) => {{
        let ($stream, token) = $stream.consume()?;

        if let $pat = &token.token_type {
            Ok(($stream, $result))
        } else {
            Err(ParseError::UnexpectedToken(
                token.token_type.discriminant().to_string(),
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
                token.token_type.discriminant().to_string(),
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
