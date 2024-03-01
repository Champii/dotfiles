use crate::{
    ast::{
        Expression, Ident, Literal, MacroInvoc, Number, Operator, PrimaryExpr, Statement, UnaryExpr,
    },
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, ParseError},
    },
};

impl Parsable for Statement {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let remaining_tokens = if parse_ctx.indent_level > 0 {
            expect_token(tokens, TokenType::Indent(parse_ctx.indent_level))?
        } else {
            tokens
        };
        let (expression, remaining_tokens) = Expression::parse(remaining_tokens, parse_ctx)?;
        let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        Ok((Statement::Expression(expression), remaining_tokens))
    }
}

impl Parsable for Expression {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let (unary_expr, remaining_tokens) = UnaryExpr::parse(tokens, parse_ctx)?;

        let token = remaining_tokens.get(0).ok_or(ParseError::UnexpectedEof)?;
        if let TokenType::Operator(_) = token.token_type {
            let (operator, remaining_tokens) = Operator::parse(remaining_tokens, parse_ctx)?;
            let (expression, remaining_tokens) = Expression::parse(remaining_tokens, parse_ctx)?;

            Ok((
                Expression::BinopExpr(unary_expr, operator, Box::new(expression)),
                remaining_tokens,
            ))
        } else {
            Ok((Expression::UnaryExpr(unary_expr), remaining_tokens))
        }
    }
}

impl Parsable for UnaryExpr {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;

        if let TokenType::Operator(_) = token.token_type {
            let (operator, remaining_tokens) = Operator::parse(tokens, parse_ctx)?;
            let (unary_expr, remaining_tokens) = UnaryExpr::parse(remaining_tokens, parse_ctx)?;

            Ok((
                UnaryExpr::UnaryExpr(operator, Box::new(unary_expr)),
                remaining_tokens,
            ))
        } else {
            let (primary_expr, remaining_tokens) = PrimaryExpr::parse(tokens, parse_ctx)?;

            Ok((UnaryExpr::PrimaryExpr(primary_expr), remaining_tokens))
        }
    }
}

impl Parsable for PrimaryExpr {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;

        match &token.token_type {
            TokenType::Ident(name) => Ok((
                PrimaryExpr::Ident(Ident {
                    name: name.clone(),
                    span: token.span.clone(),
                }),
                &tokens[1..],
            )),
            TokenType::MacroInvoc(_) => {
                let (macro_invoc, remaining_tokens) = MacroInvoc::parse(tokens, parse_ctx)?;
                Ok((PrimaryExpr::MacroInvoc(macro_invoc), remaining_tokens))
            }
            TokenType::Number(value) => Ok((
                PrimaryExpr::Literal(Literal::Number(Number {
                    value: value.clone(),
                    span: token.span.clone(),
                })),
                &tokens[1..],
            )),
            _ => Err(ParseError::UnexpectedToken(
                token.clone(),
                vec![
                    TokenType::Ident("".to_string()),
                    TokenType::MacroInvoc("".to_string()),
                    TokenType::Number("".to_string()),
                ],
            )),
        }
    }
}

impl Parsable for Number {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;

        match &token.token_type {
            TokenType::Number(value) => Ok((
                Number {
                    value: value.clone(),
                    span: token.span.clone(),
                },
                &tokens[1..],
            )),
            _ => Err(ParseError::UnexpectedToken(
                token.clone(),
                vec![TokenType::Number("".to_string())],
            )),
        }
    }
}

impl Parsable for Operator {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;

        match &token.token_type {
            TokenType::Operator(value) => Ok((
                Operator {
                    value: value.clone(),
                    span: token.span.clone(),
                },
                &tokens[1..],
            )),
            _ => Err(ParseError::UnexpectedToken(
                token.clone(),
                vec![TokenType::Operator("".to_string())],
            )),
        }
    }
}
