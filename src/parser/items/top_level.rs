use crate::{
    ast::{
        EnumDecl, FunctionDecl, Ident, Impl, MacroDecl, MacroInvoc, StructDecl, TopLevel,
        TopLevelKind,
    },
    lexer::{Token, TokenType},
    parser::{parsable::Parsable, parse_ctx::ParseCtx, util::ParseError},
};

impl Parsable for TopLevel {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
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
            } else {
                return Err(ParseError::UnexpectedKeyword(
                    tokens[0].clone(),
                    vec!["macro".to_string()],
                ));
            }
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

        if let TokenType::Ident(_name) = &tokens[0].token_type {
            return FunctionDecl::parse(tokens, parse_ctx).map(|(function_decl, new_tokens)| {
                (
                    TopLevel {
                        ident: function_decl.name.clone(),
                        kind: TopLevelKind::FunctionDecl(function_decl),
                    },
                    new_tokens,
                )
            });
        }

        Err(ParseError::UnexpectedToken(
            tokens[0].clone(),
            vec![
                TokenType::Keyword("macro".to_string()),
                TokenType::MacroInvoc("".to_string()),
                TokenType::Ident("".to_string()),
            ],
        ))
    }
}
