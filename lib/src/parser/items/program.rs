use crate::{
    ast::{Ident, Module, ModuleInner, Program, TopLevel},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parse_ctx::ParseCtx,
        parse_file,
        util::{expect_token, ignore_empty_lines, look_ahead, ParseError},
        Parsable,
    },
};

impl Parsable for Program {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        parse_ctx.add_file_relative("./src/main.rk".to_string());

        let (module, tokens) = ModuleInner::parse(tokens, parse_ctx)?;
        let mut module: Module = module.into();
        module.filepath = Some(parse_ctx.current_file.clone().unwrap());
        module.name = Some(Ident {
            name: "main".to_string(),
            span: Default::default(),
        });

        let remaining_tokens = expect_token(tokens, TokenType::Eof)?;

        if !remaining_tokens.is_empty() {
            return Err(ParseError::LeftoverTokens(remaining_tokens.to_vec()).into());
        }

        Ok((Program { module }, tokens))
    }
}

impl Parsable for Module {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let tokens = expect_token(tokens, TokenType::Keyword("mod".to_string()))?;
        let (name, tokens) = Ident::parse(tokens, parse_ctx)?;

        if look_ahead(
            tokens,
            &vec![
                TokenType::Eol,
                TokenType::Indent(parse_ctx.indent_level() + parse_ctx.indent_step()),
            ],
        ) {
            let tokens = expect_token(tokens, TokenType::Eol)?;

            parse_ctx.indent();

            let (module_inner, tokens) = ModuleInner::parse(tokens, parse_ctx)?;

            parse_ctx.dedent();

            Ok((
                Module {
                    name: Some(name),
                    top_levels: module_inner.top_levels,
                    is_inline: true,
                    filepath: None,
                },
                tokens,
            ))
        } else {
            let tokens = expect_token(tokens, TokenType::Eol)?;
            let old_file_name = parse_ctx.current_file.clone();

            parse_ctx.add_file_relative(name.name.clone());
            let path = parse_ctx.current_file.clone().unwrap();

            let mut module: Module = parse_file::<ModuleInner>(path.clone(), parse_ctx)?.into();

            module.filepath = Some(path);
            module.name = Some(name);

            parse_ctx.current_file = old_file_name;

            Ok((module, tokens))
        }
    }
}

impl From<ModuleInner> for Module {
    fn from(module_inner: ModuleInner) -> Self {
        Module {
            name: None,
            top_levels: module_inner.top_levels,
            is_inline: false,
            filepath: None,
        }
    }
}

impl Parsable for ModuleInner {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let mut top_levels = Vec::new();
        let mut tokens = tokens;

        loop {
            tokens = ignore_empty_lines(&tokens);

            if tokens.is_empty() || tokens[0].token_type == TokenType::Eof {
                break;
            }

            if let Ok(new_tokens) = parse_ctx.consume_indent(&tokens) {
                tokens = new_tokens;
            } else {
                break;
            }

            let (top_level, new_tokens) = TopLevel::parse(tokens, parse_ctx)?;

            if tokens.is_empty() || tokens[0].token_type == TokenType::Eof {
                break;
            }

            tokens = new_tokens;

            top_levels.push(top_level);
        }

        Ok((ModuleInner { top_levels }, tokens))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Program, parser::parse_string};

    #[test]
    fn program_with_newlines() {
        let input = r#"

main = -> 1


test = -> 2


"#;

        assert!(parse_string::<Program>(input).is_ok());
    }

    #[test]
    fn program_with_no_newlines() {
        let input = r#"main = -> 1
test = -> 2"#;

        assert!(parse_string::<Program>(input).is_ok());
    }

    #[test]
    fn program_with_good_2_indent() {
        let input = r#"main = ->
  a
  2"#;

        assert!(parse_string::<Program>(input).is_ok());
    }

    #[test]
    fn program_with_good_4_indent() {
        let input = r#"main = ->
    a
    2"#;

        assert!(parse_string::<Program>(input).is_ok());
    }

    #[test]
    fn program_with_bad_indent() {
        let input = r#"main = ->
    a
  2"#;

        assert!(parse_string::<Program>(input).is_err());
    }
}
