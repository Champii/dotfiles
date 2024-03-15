use crate::{
    ast::{Expression, Statement},
    lexer::{Token, TokenType},
    parser::{parsable::Parsable, parse_ctx::ParseCtx, util::ParseError},
};

impl Parsable for Statement {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let remaining_tokens = if let TokenType::Indent(level) = tokens[0].token_type {
            if level != parse_ctx.indent_level {
                return Err(ParseError::IndentMismatch(level, parse_ctx.indent_level));
            }
            &tokens[1..]
        } else {
            tokens
        };

        let (expression, remaining_tokens) = Expression::parse(remaining_tokens, parse_ctx)?;

        Ok((Statement::Expression(expression), remaining_tokens))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        ast::{Literal, Operand, PrimaryExpr, UnaryExpr},
        lexer::Span,
        parser::util::lex_test,
    };

    #[test]
    fn test_parse_statement() {
        let input = "1";
        let tokens = lex_test(input);
        let (statement, rest) = Statement::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            statement,
            Statement::Expression(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Literal(Literal {
                    kind: crate::ast::LiteralKind::Number(1),
                    span: Span::default(),
                }),
                secondaries: None,
            })))
        );

        assert_eq!(rest.len(), 0);
    }
}
