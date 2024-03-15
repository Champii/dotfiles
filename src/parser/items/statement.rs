use crate::{
    ast::{
        Expression, IdentifierPath, Literal, LiteralKind, Operand, Operator, PrimaryExpr,
        Statement, UnaryExpr,
    },
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{consume_tokens_until, expect_token, ParseError},
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

        Ok((Statement::Expression(expression), remaining_tokens))
    }
}

impl Parsable for Expression {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let (unary_expr, remaining_tokens) = UnaryExpr::parse(tokens, parse_ctx)?;

        let token =
            remaining_tokens
                .get(0)
                .ok_or(ParseError::UnexpectedEof(TokenType::Operator(
                    "".to_string(),
                )))?;
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
        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Operator(
                "".to_string(),
            )))?;

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
        let (operand, remaining_tokens) = Operand::parse(tokens, parse_ctx)?;

        let primary_expr = PrimaryExpr {
            operand,
            secondaries: None,
        };

        Ok((primary_expr, remaining_tokens))
    }
}

impl Parsable for Operand {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Ident("".to_string())))?;

        match &token.token_type {
            TokenType::Ident(_) => {
                let (identifier_path, remaining_tokens) = IdentifierPath::parse(tokens, parse_ctx)?;
                Ok((Operand::Ident(identifier_path), remaining_tokens))
            }
            TokenType::Number(value) => Ok((
                Operand::Literal(Literal {
                    kind: LiteralKind::Number(value.parse().unwrap()),
                    span: token.span.clone(),
                }),
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

impl Parsable for Literal {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let mut tokens = tokens;

        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Number("".to_string())))?;

        let literal_kind = match &token.token_type {
            TokenType::Ident(value) if value == "true" || value == "false" => {
                tokens = &tokens[1..];
                LiteralKind::Bool(value.parse().unwrap())
            }
            TokenType::Number(value) => {
                tokens = &tokens[1..];
                LiteralKind::Number(value.parse().unwrap())
            }
            TokenType::DoubleQuote => {
                let (string, remaining_tokens) = String::parse(tokens, parse_ctx)?;
                tokens = remaining_tokens;
                LiteralKind::String(string)
            }
            TokenType::SimpleQuote => {
                let (char, remaining_tokens) = char::parse(tokens, parse_ctx)?;
                tokens = remaining_tokens;
                LiteralKind::Char(char)
            }
            // TokenType::Float(value) => LiteralKind::Float(value.parse().unwrap()),
            // TokenType::Array(_) => {
            // let (array, remaining_tokens) = Array::parse(tokens, parse_ctx)?;
            // LiteralKind::Array(array)
            // }
            _ => {
                return Err(ParseError::UnexpectedToken(
                    token.clone(),
                    vec![TokenType::Number("".to_string())],
                ))
            }
        };

        Ok((
            Literal {
                kind: literal_kind,
                span: token.span.clone(),
            },
            &tokens,
        ))
    }
}

impl Parsable for String {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let remaining_tokens = expect_token(tokens, TokenType::DoubleQuote)?;

        let (inner_tokens, remaining_tokens) =
            consume_tokens_until(remaining_tokens, TokenType::DoubleQuote);

        let remaining_tokens = expect_token(remaining_tokens, TokenType::DoubleQuote)?;

        let string = inner_tokens
            .iter()
            .map(|token| token.token_type.to_string())
            .collect();

        Ok((string, remaining_tokens))
    }
}

impl Parsable for char {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let mut remaining_tokens = expect_token(tokens, TokenType::SimpleQuote)?;

        let inner_token = remaining_tokens[0].clone();

        if format!("{}", inner_token.token_type.to_string()).len() > 1 {
            return Err(ParseError::UnexpectedToken(
                inner_token,
                vec![TokenType::SimpleQuote],
            ));
        }
        remaining_tokens = &remaining_tokens[1..];

        let remaining_tokens = expect_token(remaining_tokens, TokenType::SimpleQuote)?;

        Ok((
            inner_token.token_type.to_string().chars().next().unwrap(),
            remaining_tokens,
        ))
    }
}

impl Parsable for Operator {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Operator(
                "".to_string(),
            )))?;

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

#[cfg(test)]
mod literals {
    use std::path::PathBuf;

    use super::*;
    use crate::lexer::Lexer;

    fn lex(input: &str) -> Vec<Token> {
        let mut tokens = Lexer::new(PathBuf::new(), input)
            .unwrap()
            .with_newline_at_end(false)
            .collect()
            .unwrap();

        // ignoring indent
        tokens.remove(0);
        // ignoring EOF
        tokens.pop();

        tokens
    }

    fn parse_literal(input: &str) -> Literal {
        let tokens = lex(input);
        let (literal, remaining_tokens) = Literal::parse(&tokens, &mut ParseCtx::new()).unwrap();

        println!("{:#?}", literal);
        println!("{:#?}", remaining_tokens);

        assert_eq!(remaining_tokens.len(), 0);

        literal
    }

    #[test]
    fn test_parse_number() {
        let input = "123";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::Number(123));
    }

    #[test]
    fn test_parse_string() {
        let input = "\"hello\"";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::String("hello".to_string()));
    }

    #[test]
    fn test_parse_char() {
        let input = "'a'";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::Char('a'));
    }
}
