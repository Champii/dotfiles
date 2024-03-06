use crate::{
    ast::{Ident, MacroDecl, MacroEntry, MacroFragment, MacroInvoc},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{
            consume_tokens_until, consume_tokens_until_one_of, expect_token, parse_vec_of,
            ParseError,
        },
    },
};

impl Parsable for MacroDecl {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("macro".to_string()))?;

        let (name, mut remaining_tokens) = Ident::parse(remaining_tokens, parse_ctx)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        parse_ctx.indent_level += 2;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Indent(2))?;

        let (entries, remaining_tokens) =
            parse_vec_of::<MacroEntry>(remaining_tokens, Some(TokenType::Indent(2)), parse_ctx)?;

        parse_ctx.indent_level -= 2;

        Ok((MacroDecl { name, entries }, remaining_tokens))
    }
}

impl Parsable for MacroEntry {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        // let remaining_tokens = expect_token(tokens, TokenType::Indent(parse_ctx.indent_level))?;

        let (defs, mut remaining_tokens) = consume_tokens_until(tokens, TokenType::FatArrow);

        let mut new_defs = Vec::new();

        let mut skip_until = 0;
        for (i, def) in defs.iter().enumerate() {
            if i < skip_until {
                continue;
            }

            if let TokenType::MacroVar(name) = &def.token_type {
                if defs[i + 1].token_type != TokenType::Colon {
                    return Err(ParseError::UnexpectedToken(
                        defs[i + 1].clone(),
                        vec![TokenType::Colon],
                    ));
                }

                if defs[i + 2].token_type == TokenType::Ident("ident".to_string()) {
                    new_defs.push(MacroFragment::Ident(Ident {
                        name: name.clone(),
                        span: def.span.clone(),
                    }));
                }

                skip_until = i + 3;

                continue;
            } else {
                new_defs.push(MacroFragment::Token(def.clone()));
            }
        }
        // parse the defs and replace with MacroFragment

        remaining_tokens = expect_token(remaining_tokens, TokenType::FatArrow)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        parse_ctx.indent_level += 2;

        let (mut block, remaining_tokens) = consume_tokens_until_one_of(
            remaining_tokens,
            vec![TokenType::Indent(0), TokenType::Indent(2)],
        );

        block.push(Token {
            token_type: TokenType::Eof,
            span: block.last().unwrap().span.clone(),
        });

        parse_ctx.indent_level -= 2;

        Ok((
            MacroEntry {
                defs: new_defs,
                block,
            },
            remaining_tokens,
        ))
    }
}

impl Parsable for MacroInvoc {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        if let Some(token) = tokens.get(0) {
            if let TokenType::MacroInvoc(ident) = &token.token_type {
                let (args, remaining_tokens) = consume_tokens_until(&tokens[1..], TokenType::Eol);
                let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

                return Ok((
                    MacroInvoc {
                        name: Ident {
                            name: ident.clone(),
                            span: token.span.clone(),
                        },
                        args,
                    },
                    remaining_tokens,
                ));
            }
        }

        Err(ParseError::UnexpectedEof)
    }
}
