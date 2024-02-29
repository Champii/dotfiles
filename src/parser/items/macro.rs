use crate::{
    ast::{Ident, MacroDecl, MacroFragment, MacroInvoc},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{consume_tokens_until, expect_token, ParseError},
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

        remaining_tokens = expect_token(remaining_tokens, TokenType::Indent(2))?;

        let (defs, mut remaining_tokens) =
            consume_tokens_until(remaining_tokens, TokenType::FatArrow);

        println!("ORIGINAL_DEFS {:#?}", defs);
        let mut new_defs = Vec::new();

        let mut skip_until = 0;
        for (i, def) in defs.iter().enumerate() {
            if i < skip_until {
                continue;
            }

            if let TokenType::MacroInvoc(name) = &def.token_type {
                if defs[i + 1].token_type != TokenType::Colon {
                    return Err(ParseError::UnexpectedToken(defs[i + 1].clone()));
                }

                println!("IDENT TOKEN {:#?}", defs[i + 2]);
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
        remaining_tokens = expect_token(remaining_tokens, TokenType::Indent(4))?;

        let (mut block, remaining_tokens) = consume_tokens_until(remaining_tokens, TokenType::Eol);
        block.push(Token {
            token_type: TokenType::Eol,
            span: block.last().unwrap().span.clone(),
        });

        Ok((
            MacroDecl {
                name,
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
