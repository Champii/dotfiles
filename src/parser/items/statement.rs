use crate::{
    ast::{Expression, Ident, Literal, Number, Operator, PrimaryExpr, Statement, UnaryExpr},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        util::{expect_token, ParseError},
    },
};

impl Parsable for Statement {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let (expression, new_tokens) = Expression::parse(tokens)?;
        let new_tokens = expect_token(new_tokens, TokenType::Eol)?;

        Ok((Statement::Expression(expression), new_tokens))
    }
}

impl Parsable for Expression {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let (unary_expr, remaining_tokens) = UnaryExpr::parse(tokens)?;

        let token = remaining_tokens.get(0).ok_or(ParseError::UnexpectedEof)?;
        if let TokenType::Operator(_) = token.token_type {
            let (operator, remaining_tokens) = Operator::parse(remaining_tokens)?;
            let (expression, remaining_tokens) = Expression::parse(remaining_tokens)?;

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
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;

        if let TokenType::Operator(_) = token.token_type {
            let (operator, remaining_tokens) = Operator::parse(tokens)?;
            let (unary_expr, remaining_tokens) = UnaryExpr::parse(remaining_tokens)?;

            Ok((
                UnaryExpr::UnaryExpr(operator, Box::new(unary_expr)),
                remaining_tokens,
            ))
        } else {
            let (primary_expr, remaining_tokens) = PrimaryExpr::parse(tokens)?;

            Ok((UnaryExpr::PrimaryExpr(primary_expr), remaining_tokens))
        }
    }
}

impl Parsable for PrimaryExpr {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;

        match &token.token_type {
            TokenType::Ident(name) => Ok((
                PrimaryExpr::Ident(Ident {
                    name: name.clone(),
                    span: token.span.clone(),
                }),
                &tokens[1..],
            )),
            TokenType::Number(value) => Ok((
                PrimaryExpr::Literal(Literal::Number(Number {
                    value: value.clone(),
                    span: token.span.clone(),
                })),
                &tokens[1..],
            )),
            _ => Err(ParseError::UnexpectedToken(token.clone())),
        }
    }
}

impl Parsable for Number {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;

        match &token.token_type {
            TokenType::Number(value) => Ok((
                Number {
                    value: value.clone(),
                    span: token.span.clone(),
                },
                &tokens[1..],
            )),
            _ => Err(ParseError::UnexpectedToken(token.clone())),
        }
    }
}

impl Parsable for Operator {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;

        match &token.token_type {
            TokenType::Operator(value) => Ok((
                Operator {
                    value: value.clone(),
                    span: token.span.clone(),
                },
                &tokens[1..],
            )),
            _ => Err(ParseError::UnexpectedToken(token.clone())),
        }
    }
}
