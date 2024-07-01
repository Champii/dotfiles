use crate::{
    ast::{
        EnumDecl, FunctionDecl, FunctionSig, Ident, IdentifierPath, Impl, MacroDecl, MacroInvoc,
        Module, ModuleDecl, Operator, ParseType, ParseTypeInner, StructDecl, TopLevel,
        TopLevelKind, TraitDecl,
    },
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, ParseError},
    },
};

impl Parsable for TopLevel {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        if let TokenType::Keyword(keyword) = &tokens[0].token_type {
            if keyword == "macro" {
                return MacroDecl::parse(tokens, parse_ctx).map(|(macro_decl, new_tokens)| {
                    (
                        TopLevel {
                            ident: macro_decl.name.clone(),
                            kind: TopLevelKind::MacroDecl(macro_decl),
                        },
                        new_tokens,
                    )
                });
            } else if keyword == "struct" {
                return StructDecl::parse(tokens, parse_ctx).map(|(struct_decl, new_tokens)| {
                    (
                        TopLevel {
                            ident: Ident::default(), // FIXME
                            kind: TopLevelKind::StructDecl(struct_decl),
                        },
                        new_tokens,
                    )
                });
            } else if keyword == "enum" {
                return EnumDecl::parse(tokens, parse_ctx).map(|(enum_decl, new_tokens)| {
                    (
                        TopLevel {
                            ident: Ident::default(), // FIXME
                            kind: TopLevelKind::EnumDecl(enum_decl),
                        },
                        new_tokens,
                    )
                });
            } else if keyword == "trait" {
                return TraitDecl::parse(tokens, parse_ctx).map(|(trait_decl, new_tokens)| {
                    (
                        TopLevel {
                            ident: Ident::default(), // FIXME
                            kind: TopLevelKind::TraitDecl(trait_decl),
                        },
                        new_tokens,
                    )
                });
            } else if keyword == "impl" {
                return Impl::parse(tokens, parse_ctx).map(|(impl_, new_tokens)| {
                    (
                        TopLevel {
                            ident: Ident::default(), // FIXME
                            kind: TopLevelKind::Impl(impl_),
                        },
                        new_tokens,
                    )
                });
            } else if keyword == "infix" {
                if let TokenType::Number(precedence) = &tokens[1].token_type {
                    let precedence: u8 = precedence.parse().unwrap();

                    if precedence > 9 {
                        return Err(
                            ParseError::InvalidPrecedence(precedence, tokens[1].clone()).into()
                        );
                    }

                    let (op, remaining_tokens) = if let Ok((op, remaining_tokens)) =
                        Operator::parse(&tokens[2..], parse_ctx)
                    {
                        (op, remaining_tokens)
                    } else if let TokenType::SpacedDot = tokens[2].token_type {
                        let remaining_tokens = expect_token(&tokens[2..], TokenType::SpacedDot)?;

                        let operator = Operator {
                            value: ".".to_string(),
                            span: tokens[2].span.clone(),
                        };

                        (operator, remaining_tokens)
                    } else {
                        return Err(ParseError::UnexpectedToken(
                            tokens[2].clone(),
                            vec![TokenType::Operator("".to_string())],
                        )
                        .into());
                    };

                    let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

                    return Ok((
                        TopLevel {
                            ident: Ident::default(), // FIXME
                            kind: TopLevelKind::InfixOperator(precedence, op.value.clone()),
                        },
                        remaining_tokens,
                    ));
                } else {
                    return Err(ParseError::UnexpectedToken(
                        tokens[1].clone(),
                        vec![TokenType::Number("".to_string())],
                    )
                    .into());
                }
            } else if keyword == "mod" {
                return Module::parse(tokens, parse_ctx).map(|(module, new_tokens)| {
                    (
                        TopLevel {
                            ident: module.name.clone().unwrap_or_default(),
                            kind: TopLevelKind::Module(ModuleDecl(module)),
                        },
                        new_tokens,
                    )
                });
            } else if keyword == "extern" {
                return FunctionSig::parse(&tokens[1..], parse_ctx).map(
                    |(function_sig, new_tokens)| {
                        (
                            TopLevel {
                                ident: function_sig.name.clone(),
                                kind: TopLevelKind::Extern(function_sig),
                            },
                            new_tokens,
                        )
                    },
                );
            } else if keyword == "type" {
                let (parse_type_inner, remaining_tokens) =
                    ParseTypeInner::parse(&tokens[1..], parse_ctx)?;

                let remaining_tokens = expect_token(remaining_tokens, TokenType::Equal)?;

                let (parse_type, remaining_tokens) = ParseType::parse(remaining_tokens, parse_ctx)?;

                let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

                return Ok((
                    TopLevel {
                        ident: Ident::default(), // FIXME
                        kind: TopLevelKind::NewType(parse_type_inner, parse_type),
                    },
                    remaining_tokens,
                ));
            } else {
                return Err(ParseError::UnexpectedKeyword(
                    tokens[0].clone(),
                    vec!["macro".to_string()],
                )
                .into());
            }
        }

        if let TokenType::Comment(s) = &tokens[0].token_type {
            let tokens = expect_token(&tokens[1..], TokenType::Eol)?;

            return Ok((
                TopLevel {
                    ident: Ident::default(), // FIXME
                    kind: TopLevelKind::Comment(s.clone()),
                },
                tokens,
            ));
        }

        if TokenType::Operator(">".to_string()) == tokens[0].token_type {
            let (identifier_path, new_tokens) = IdentifierPath::parse(&tokens[1..], parse_ctx)?;

            let new_tokens = expect_token(new_tokens, TokenType::Eol)?;

            return Ok((
                TopLevel {
                    ident: Ident::default(), // FIXME
                    kind: TopLevelKind::Import(identifier_path),
                },
                new_tokens,
            ));
        }

        if TokenType::Operator("<".to_string()) == tokens[0].token_type {
            let (identifier_path, new_tokens) = IdentifierPath::parse(&tokens[1..], parse_ctx)?;

            let new_tokens = expect_token(new_tokens, TokenType::Eol)?;

            return Ok((
                TopLevel {
                    ident: Ident::default(), // FIXME
                    kind: TopLevelKind::Export(identifier_path),
                },
                new_tokens,
            ));
        }

        if let TokenType::MacroInvoc(_name) = &tokens[0].token_type {
            return MacroInvoc::parse(tokens, parse_ctx).map(|(macro_invoc, new_tokens)| {
                (
                    TopLevel {
                        ident: macro_invoc.name.clone(),
                        kind: TopLevelKind::MacroInvoc(macro_invoc),
                    },
                    new_tokens,
                )
            });
        }

        let name = match &tokens[0].token_type {
            TokenType::Ident(name) | TokenType::Operator(name) => Some(name.clone()),
            TokenType::Dot => Some(".".to_string()),
            _ => None,
        };

        if let Some(_) = name {
            if let TokenType::Equal = tokens[1].token_type {
                return FunctionDecl::parse(tokens, parse_ctx).map(
                    |(function_decl, new_tokens)| {
                        (
                            TopLevel {
                                ident: function_decl.name.clone(),
                                kind: TopLevelKind::FunctionDecl(function_decl),
                            },
                            new_tokens,
                        )
                    },
                );
            } else if let TokenType::Colon = tokens[1].token_type {
                return FunctionSig::parse(tokens, parse_ctx).map(|(function_sig, new_tokens)| {
                    (
                        TopLevel {
                            ident: function_sig.name.clone(),
                            kind: TopLevelKind::FunctionSig(function_sig),
                        },
                        new_tokens,
                    )
                });
            }
        }

        Err(ParseError::UnexpectedToken(
            tokens[0].clone(),
            vec![
                TokenType::Keyword("macro".to_string()),
                TokenType::MacroInvoc("".to_string()),
                TokenType::Ident("".to_string()),
            ],
        )
        .into())
    }
}

#[cfg(test)]
mod parse_top_level {
    use crate::{parser::util::lex_test, Config};

    use super::*;

    #[test]
    fn parse_infix_operator() {
        let input = "infix 5 |>\n";
        let tokens = lex_test(input);
        let (top_level, rest) =
            TopLevel::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        let (precedence, name) = match top_level.kind {
            TopLevelKind::InfixOperator(precedence, name) => (precedence, name),
            _ => panic!(),
        };

        assert_eq!(precedence, 5);
        assert_eq!(name, "|>".to_string());
        assert_eq!(rest.len(), 0);
    }
}
