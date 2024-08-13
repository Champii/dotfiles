use crate::new_parser::{engine::*, Impl};

pub fn r#impl(stream: Input) -> IResult<Impl> {
    unimplemented!()
}

#[cfg(test)]
mod parse_impl {
    use crate::{
        new_parser::{lex_test, ParseCtx},
        Config,
    };

    use super::*;

    #[test]
    fn test_parse_impl() {
        let input = "impl Test\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, r#impl) = r#impl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(r#impl.name.to_string(), "Test");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_impl_with_methods() {
        let input = "impl Test\n    new = -> lol\n    @add = -> a\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, r#impl) = r#impl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(r#impl.name.to_string(), "Test");
        assert_eq!(r#impl.methods.len(), 2);
        assert_eq!(
            r#impl
                .methods
                .iter()
                .find(|(k, _v)| k.name == "new")
                .unwrap()
                .1
                .name
                .name,
            "new"
        );
        assert_eq!(rest.len(), 0);
    }
}
