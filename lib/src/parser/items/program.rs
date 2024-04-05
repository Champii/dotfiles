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
        let (name, mut tokens) = Ident::parse(tokens, parse_ctx)?;

        let comment = if let TokenType::Comment(comment) = &tokens[0].token_type {
            tokens = &tokens[1..];
            Some(comment.clone().trim().to_string())
        } else {
            None
        };

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
                    comment,
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
            module.comment = comment;

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
            comment: None,
        }
    }
}

//find the next Indent(0) that is not followed by an EOL and start from there.
fn try_recover<'a>(tokens: &'a [Token]) -> &'a [Token] {
    let mut tokens = tokens;

    while !tokens.is_empty() {
        if let TokenType::Indent(0) = tokens[0].token_type {
            if tokens
                .get(1)
                .map(|t| t.token_type != TokenType::Eol)
                .unwrap_or(false)
            {
                break;
            }
        }

        tokens = &tokens[1..];
    }

    tokens
}

impl Parsable for ModuleInner {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let mut top_levels = Vec::new();
        let mut tokens = tokens;
        let mut diagnostics = Diagnostics::default();

        loop {
            tokens = ignore_empty_lines(&tokens);

            if tokens.is_empty() || tokens[0].token_type == TokenType::Eof {
                break;
            }

            match parse_ctx.consume_indent(&tokens) {
                Ok(new_tokens) => tokens = new_tokens,
                Err(e) => {
                    diagnostics.merge(e);
                    tokens = try_recover(tokens);
                    continue;
                }
            }

            match TopLevel::parse(tokens, parse_ctx) {
                Ok((top_level, new_tokens)) => {
                    tokens = new_tokens;

                    top_levels.push(top_level);
                }
                Err(e) => {
                    diagnostics.merge(e);
                    tokens = try_recover(tokens);
                }
            }

            if tokens.is_empty() || tokens[0].token_type == TokenType::Eof {
                break;
            }
        }

        diagnostics.return_if_err()?;

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
