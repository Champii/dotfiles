use crate::new_parser::{engine::*, Ident};

use super::{ident_token, operator};

pub fn ident(stream: Input) -> IResult<Ident> {
    ident_token
        .or(operator.map(|op| Ident {
            name: op.to_string(),
            span: op.span.clone(),
        }))
        .process(stream)
}

#[cfg(test)]
mod tests {

    use crate::{new_parser::lex_test, Config};

    use super::*;

    #[test]
    fn test_parse_ident() {
        let input = "ident";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, ident) = ident.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(ident.name, "ident");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_ident_error() {
        let input = "123";
        let tokens = lex_test(input);
        let config = Config::default();

        let result = ident.process(ParseCtx::from(&tokens, &config));

        assert!(result.is_err());
    }
}
