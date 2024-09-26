use crate::new_parser::*;

pub fn instance(stream: Input) -> IResult<Instance> {
    (
        type_path,
        preceded(
            TokenType::Eol.followed_by(empty_lines),
            indented(separated1(
                preceded(indent, (followed(ident, TokenType::Colon), expression)),
                TokenType::Eol.followed_by(empty_lines),
            )),
        )
        .or(separated1(
            (followed(ident, TokenType::Colon), expression),
            TokenType::Coma,
        ))
        .opt(),
    )
        .map(|(type_path, fields)| Instance {
            name: type_path,
            fields: fields.unwrap_or_default().into_iter().collect(),
        })
        .process(stream)
}

#[cfg(test)]
mod instance {
    use super::*;
    use crate::{new_parser::lex_test, Config};

    #[test]
    fn test_parse_enum_instance() {
        let input = "Type::Variant1";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, enum_instance) = instance.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(enum_instance.name.to_string(), "Type::Variant1");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_instance_inline() {
        let input = "Test a: 1, b: 2, c: a + 4";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, struct_instance) = instance.process(ParseCtx::from(&tokens, &config)).unwrap();
        assert_eq!(struct_instance.name.to_string(), "Test");
        assert_eq!(struct_instance.fields.len(), 3);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_instance_multiline() {
        let input = "Test\n    a: 1\n    b: 2\n    c: a + 4";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, struct_instance) = instance.process(ParseCtx::from(&tokens, &config)).unwrap();
        assert_eq!(struct_instance.name.to_string(), "Test");
        assert_eq!(struct_instance.fields.len(), 3);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_instance_empty_args() {
        let input = "Test";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, struct_instance) = instance.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(struct_instance.name.to_string(), "Test");
        assert_eq!(struct_instance.fields.len(), 0);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_instance_empty_lines() {
        let input = "Test\n\n    a: 1\n\n    b: 2\n\n    c: a + 4";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, struct_instance) = instance.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(struct_instance.name.to_string(), "Test");
        assert_eq!(struct_instance.fields.len(), 3);
        assert_eq!(rest.len(), 0);
    }
}
