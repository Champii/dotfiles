use crate::{
    ast::*,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEof,
    LeftoverTokens(Vec<Token>),
}

pub fn expect_token(tokens: &[Token], token_type: TokenType) -> Result<&[Token], ParseError> {
    let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;
    if token.token_type != token_type {
        return Err(ParseError::UnexpectedToken(token.clone()));
    }
    Ok(tokens[1..].into())
}

pub trait Parsable: Sized {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError>;
}

impl Parsable for Program {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let mut statements = Vec::new();
        let mut tokens = tokens;

        loop {
            if tokens.is_empty() {
                break;
            }

            while tokens
                .get(0)
                .map(|t| t.token_type == TokenType::Eol)
                .unwrap_or(false)
            {
                tokens = &tokens[1..];
            }

            let Ok((statement, new_tokens)) = TopLevel::parse(tokens) else {
                break;
            };

            tokens = new_tokens;

            statements.push(statement);
        }

        let remaining_tokens = expect_token(tokens, TokenType::Eof)?;

        if !remaining_tokens.is_empty() {
            return Err(ParseError::LeftoverTokens(remaining_tokens.to_vec()));
        }

        Ok((Program { top_levels: statements }, tokens))
    }
}

impl Parsable for TopLevel {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let (statement, new_tokens) = FunctionDecl::parse(tokens)?;

        Ok((TopLevel::FunctionDecl(statement), new_tokens))
    }
}

impl Parsable for FunctionDecl {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let (name, mut remaining_tokens) = Ident::parse(tokens)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Equal)?;

        let (parameters, mut remaining_tokens) = Vec::<Ident>::parse(remaining_tokens)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Arrow)?;

        let (body, remaining_tokens) = Block::parse(remaining_tokens)?;

        Ok((
            FunctionDecl {
                name,
                parameters,
                body,
            },
            remaining_tokens,
        ))
    }
}

impl Parsable for Ident {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;

        match &token.token_type {
            TokenType::Ident(name) => Ok((
                Ident {
                    name: name.clone(),
                    span: token.span.clone(),
                },
                &tokens[1..],
            )),
            _ => Err(ParseError::UnexpectedToken(token.clone())),
        }
    }
}

impl Parsable for Vec<Ident> {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let mut remaining_tokens = tokens;
        let mut items = Vec::new();

        loop {
            if remaining_tokens.is_empty() {
                break;
            }

            let Ok((item, new_remaining_tokens)) = Ident::parse(remaining_tokens) else {
                break;
            };

            remaining_tokens = new_remaining_tokens;

            items.push(item);

            let Ok(new_remaining_tokens) = expect_token(remaining_tokens, TokenType::Coma) else {
                break;
            };

            remaining_tokens = new_remaining_tokens;
        }

        Ok((items, remaining_tokens))
    }
}

impl Parsable for Block {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let (statements, new_tokens) = Vec::<Statement>::parse(tokens)?;

        Ok((
            Block {
                expressions: statements,
            },
            new_tokens,
        ))
    }
}

impl Parsable for Vec<Statement> {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let mut remaining_tokens = tokens;
        let mut items = Vec::new();

        loop {
            if remaining_tokens.is_empty() {
                break;
            }

            let Ok((item, new_remaining_tokens)) = Statement::parse(remaining_tokens) else {
                break;
            };

            remaining_tokens = new_remaining_tokens;

            items.push(item);
        }

        Ok((items, remaining_tokens))
    }
}

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
