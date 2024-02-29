use crate::{
    ast::{FunctionDecl, MacroDecl, MacroInvoc, TopLevel, TopLevelKind},
    lexer::Token,
    parser::{parsable::Parsable, parse_ctx::ParseCtx, util::ParseError},
};

impl Parsable for TopLevel {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        if let Ok((function_decl, new_tokens)) = FunctionDecl::parse(tokens, parse_ctx) {
            Ok((
                TopLevel {
                    ident: function_decl.name.clone(),
                    kind: TopLevelKind::FunctionDecl(function_decl),
                },
                new_tokens,
            ))
        } else if let Ok((macro_decl, new_tokens)) = MacroDecl::parse(tokens, parse_ctx) {
            Ok((
                TopLevel {
                    ident: macro_decl.name.clone(),
                    kind: TopLevelKind::MacroDecl(macro_decl),
                },
                new_tokens,
            ))
        } else if let Ok((macro_invoc, new_tokens)) = MacroInvoc::parse(tokens, parse_ctx) {
            Ok((
                TopLevel {
                    ident: macro_invoc.name.clone(),
                    kind: TopLevelKind::MacroInvoc(macro_invoc),
                },
                new_tokens,
            ))
        } else {
            Err(ParseError::UnexpectedToken(tokens[0].clone()))
        }
    }
}
