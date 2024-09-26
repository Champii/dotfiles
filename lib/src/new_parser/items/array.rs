use crate::{
    lexer::TokenType,
    new_parser::{engine::*, Array, Expression},
};

use super::{empty_lines, expression, indent};

pub fn multiline_array(stream: Input) -> IResult<Vec<Expression>> {
    indented(preceded(
        TokenType::Eol,
        preceded(
            empty_lines,
            separated_trailing(
                preceded(indent, separated1(expression, TokenType::Coma)),
                (
                    TokenType::Coma.opt(),
                    TokenType::Eol.followed_by(empty_lines),
                ),
            ),
        )
        .map(|elements| elements.into_iter().flatten().collect::<Vec<_>>()),
    ))
    .followed_by(indent)
    .process(stream)
}

pub fn monoline_array(stream: Input) -> IResult<Vec<Expression>> {
    separated_trailing(expression, TokenType::Coma).process(stream)
}

pub fn array(stream: Input) -> IResult<Array> {
    delimited(
        TokenType::OpenBracket,
        multiline_array.or(monoline_array),
        TokenType::CloseBracket,
    )
    .map(|elements| Array { elements })
    .process(stream)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{new_parser::lex_test, Config};

    #[test]
    fn test_parse_empty_array() {
        let input = "[]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array.elements.len(), 0);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_single_element_array() {
        let input = "[42]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array.elements.len(), 1);
        assert_eq!(rest.len(), 0);
        // Vous pouvez ajouter des assertions supplémentaires pour vérifier la valeur de l'élément
    }

    #[test]
    fn test_parse_multiple_elements_array() {
        let input = "[1, 2, 3]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array.elements.len(), 3);
        assert_eq!(rest.len(), 0);
        // Vérifiez les valeurs des éléments si nécessaire
    }

    #[test]
    fn test_parse_array_with_expressions() {
        let input = "[a + b, foo!]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array.elements.len(), 2);
        assert_eq!(rest.len(), 0);
        // Vérifiez les expressions des éléments si nécessaire
    }

    #[test]
    fn test_parse_nested_arrays() {
        let input = "[[1, 2], [3, 4]]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array_outer) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array_outer.elements.len(), 2);
        assert_eq!(rest.len(), 0);
        // Vous pouvez descendre dans les éléments pour vérifier les tableaux imbriqués
    }

    #[test]
    fn test_parse_array_with_trailing_comma() {
        let input = "[1, 2, 3,]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array.elements.len(), 3);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_array_with_whitespace() {
        let input = "[ 1 , 2 , 3 ]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array.elements.len(), 3);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_array_with_comments() {
        let input = "[1, /* comment */ 2, 3]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array.elements.len(), 3);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_array_with_errors() {
        let input = "[1, 2,";
        let tokens = lex_test(input);
        let config = Config::default();

        let result = array.process(ParseCtx::from(&tokens, &config));

        assert!(result.is_err());
    }

    // While this is parsed correctly, this should fail later during typechecking
    #[test]
    fn test_parse_array_with_mixed_types() {
        let input = "[1, \"string\", true]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array.elements.len(), 3);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_large_array() {
        let input = "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array.elements.len(), 10);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_multiline_array() {
        let input = "[\n    1,\n    2,\n    3\n]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array.elements.len(), 3);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_mixed_multiline_array() {
        let input = "[\n    1, 2,\n    3, 4\n]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array.elements.len(), 4);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_multiline_array_with_empty_lines() {
        let input = "[\n    1,\n\n    2,\n    \n    3\n]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, array) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(array.elements.len(), 3);
        assert_eq!(rest.len(), 0);
    }
}
