use crate::new_parser::{engine::*, Program};

use super::module_inline;

pub fn program(stream: Input) -> IResult<Program> {
    module_inline
        .map(|module| Program { module })
        .process(stream)
}

#[cfg(test)]
mod tests {
    use crate::{new_parser::parse_string, Config};

    #[test]
    fn program_with_newlines() {
        let input = r#"

main = -> 1


test = -> 2


"#;

        assert!(parse_string(input, &Config::default()).is_ok());
    }

    #[test]
    fn program_with_no_newlines() {
        let input = r#"main = -> 1
test = -> 2"#;

        assert!(parse_string(input, &Config::default()).is_ok());
    }

    #[test]
    fn program_with_good_2_indent() {
        let input = r#"main = ->
  a
  2"#;

        assert!(parse_string(input, &Config::default()).is_ok());
    }

    #[test]
    fn program_with_good_4_indent() {
        let input = r#"main = ->
    a
    2"#;

        assert!(parse_string(input, &Config::default()).is_ok());
    }

    #[test]
    fn program_with_bad_indent() {
        let input = r#"main = ->
    a
  2"#;

        assert!(parse_string(input, &Config::default()).is_err());
    }
}
