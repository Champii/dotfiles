use crate::new_parser::*;

pub fn parse_condition(stream: Input) -> IResult<Condition> {
    disallow_multiline_fn_call((pattern.followed_by(TokenType::Equal).opt(), expression))
        .map(|(pattern, expression)| Condition {
            pattern,
            expression,
        })
        .process(stream)
}

pub fn parse_if(stream: Input) -> IResult<If> {
    (
        TokenType::Keyword("if".to_string()),
        parse_condition,
        (
            TokenType::Eol.opt(),
            indent.opt(),
            TokenType::Keyword("then".to_string()),
        )
            .opt(),
        block,
        (TokenType::Eol, indent.opt()).opt(),
        parse_else.opt(),
    )
        .map(|(_, condition, _, then, _, else_)| If {
            condition,
            then,
            else_,
        })
        .process(stream)
}

pub fn parse_else(stream: Input) -> IResult<Else> {
    preceded(
        TokenType::Keyword("else".to_string()),
        parse_if
            .map(Box::new)
            .map(Else::If)
            .or(block.map(Else::Block)),
    )
    .process(stream)
}

#[cfg(test)]
mod test_if {
    use super::*;
    use crate::{new_parser::lex_test, Config};

    #[test]
    fn test_parse_if_monoline() {
        let input = "if a then 1";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, _if_) = parse_if.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_if_else_monoline() {
        let input = "if true then 1 else z";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, _if_) = parse_if.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_if_else_if_else_monoline() {
        let input = "if true then 1 else if false then 2 else 3";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, _if_) = parse_if.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_if_else_multiline_1() {
        let input = "if true\nthen 1\nelse 2";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, _if_) = parse_if.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_if_else_multiline_2() {
        let input = "if true then\n    1\nelse if false then\n    2\nelse 3";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, _if_) = parse_if.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_if_else_multiline_3() {
        let input = "if true\n    1\nelse\n    2";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, _if_) = parse_if.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn multiline_if() {
        let input = "if true\n    1";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, _if_) = parse_if.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn pattern_condition_if() {
        let input = "if a = 1 then 1";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, _if_) = parse_if.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(rest.len(), 0);
    }
}
