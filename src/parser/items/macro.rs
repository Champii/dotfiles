use crate::{
    ast::{Ident, MacroDecl, MacroEntry, MacroFragment, MacroInvoc},
    lexer::{Span, Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{consume_tokens_until, expect_token, parse_vec_of, ParseError},
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

        parse_ctx.indent();

        let remaining_tokens = parse_ctx.consume_indent(remaining_tokens)?;

        let (entries, remaining_tokens) =
            parse_vec_of::<MacroEntry>(remaining_tokens, Some(TokenType::Indent(2)), parse_ctx)?;

        parse_ctx.dedent();

        Ok((MacroDecl { name, entries }, remaining_tokens))
    }
}

impl Parsable for MacroEntry {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let (defs, mut remaining_tokens) = parse_macro_head_recursive(tokens, parse_ctx)?;

        remaining_tokens = expect_token(remaining_tokens, TokenType::FatArrow)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        // parse macro body
        let (mut body, remaining_tokens) =
            parse_macro_block_recursive(remaining_tokens, parse_ctx)?;

        body.push(MacroFragment::Token(Token {
            token_type: TokenType::Eof,
            span: Span::default(),
        }));

        Ok((MacroEntry { defs, body }, remaining_tokens))
    }
}

fn parse_macro_head_recursive<'a>(
    tokens: &'a [Token],
    parse_ctx: &mut ParseCtx,
) -> Result<(Vec<MacroFragment>, &'a [Token]), ParseError> {
    let mut defs = Vec::new();
    let mut remaining_tokens = tokens;

    let mut skip_until = 0;
    while let Some(token) = remaining_tokens.get(0) {
        if skip_until > 0 {
            skip_until -= 1;
            remaining_tokens = &remaining_tokens[1..];
            continue;
        }
        match &token.token_type {
            TokenType::MacroVar(name) => {
                if remaining_tokens.get(1).unwrap().token_type != TokenType::Colon {
                    return Err(ParseError::UnexpectedToken(
                        remaining_tokens.get(1).unwrap().clone(),
                        vec![TokenType::Colon],
                    ));
                }

                if remaining_tokens.get(2).unwrap().token_type
                    == TokenType::Ident("ident".to_string())
                {
                    defs.push(MacroFragment::Ident(Ident {
                        name: name.clone(),
                        span: token.span.clone(),
                    }));
                }

                skip_until = 3;

                continue;
            }
            TokenType::MacroRepeatOpen => {
                let (inner_block, new_remaining_tokens) =
                    parse_macro_head_recursive(&remaining_tokens[1..], parse_ctx)?;

                remaining_tokens = new_remaining_tokens;
                defs.push(MacroFragment::Repetition(inner_block));
            }
            TokenType::MacroRepeatClose => {
                return Ok((defs, &remaining_tokens[1..]));
            }
            TokenType::FatArrow => {
                return Ok((defs, remaining_tokens));
            }
            _ => {
                defs.push(MacroFragment::Token(token.clone()));
                remaining_tokens = &remaining_tokens[1..];
            }
        }
    }

    Ok((defs, remaining_tokens))
}

fn parse_macro_block_recursive<'a>(
    tokens: &'a [Token],
    parse_ctx: &mut ParseCtx,
) -> Result<(Vec<MacroFragment>, &'a [Token]), ParseError> {
    let mut block = Vec::new();
    let mut remaining_tokens = tokens;

    while let Some(token) = remaining_tokens.get(0) {
        match &token.token_type {
            TokenType::MacroVar(name) => {
                let ident = MacroFragment::Ident(Ident {
                    name: name.clone(),
                    span: token.span.clone(),
                });
                block.push(ident);
                remaining_tokens = &remaining_tokens[1..];
            }
            TokenType::MacroRepeatOpen => {
                let (inner_block, new_remaining_tokens) =
                    parse_macro_block_recursive(&remaining_tokens[1..], parse_ctx)?;

                remaining_tokens = new_remaining_tokens;
                block.push(MacroFragment::Repetition(inner_block));
            }
            TokenType::MacroRepeatClose => {
                return Ok((block, &remaining_tokens[1..]));
            }
            _ => {
                let mut token = token.clone();

                // fix the indentation for the parser
                if let TokenType::Indent(level) = token.token_type {
                    if level == 0 || level == 2 {
                        // the definition is over
                        return Ok((block, remaining_tokens));
                    }
                    token.token_type = TokenType::Indent(level - 4);
                }

                block.push(MacroFragment::Token(token));
                remaining_tokens = &remaining_tokens[1..];
            }
        }
    }

    Ok((block, remaining_tokens))
}

impl Parsable for MacroInvoc {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        if let Some(token) = tokens.get(0) {
            if let TokenType::MacroInvoc(ident) = &token.token_type {
                let (args, remaining_tokens) =
                    consume_tokens_until(&tokens[1..], TokenType::Indent(0));
                // let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;
                let args = args
                    .iter()
                    .filter(|t| {
                        t.token_type != TokenType::Eol
                            && if let TokenType::Indent(_) = t.token_type {
                                false
                            } else {
                                true
                            }
                    })
                    .cloned()
                    .collect();

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

        Err(ParseError::UnexpectedEof(TokenType::MacroInvoc(
            "".to_string(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::lexer::Lexer;

    fn lex(input: &str) -> Vec<Token> {
        Lexer::new(PathBuf::new(), input)
            .unwrap()
            .with_newline_at_end(false)
            .collect()
            .unwrap()
    }

    #[test]
    fn test_parse_macro_decl() {
        let input = "macro mymacro\n  $a:ident =>\n    statement";
        let tokens = lex(input);
        let tokens = &tokens[1..]; // skip the Indent(0)
        let (macro_decl, rest) = MacroDecl::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(macro_decl.name.name, "mymacro");
        assert_eq!(macro_decl.entries.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_macro_entry() {
        let input = "$a:ident =>\n    statement";
        let tokens = lex(input);
        let tokens = &tokens[1..]; // skip the Indent(0)
        let (macro_entry, rest) = MacroEntry::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(macro_entry.defs.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_macro_invoc() {
        let input = "%mymacro\n  a\n  b";
        let tokens = lex(input);
        let tokens = &tokens[1..]; // skip the Indent(0)
        let (macro_invoc, rest) = MacroInvoc::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(macro_invoc.name.name, "mymacro");
        assert_eq!(macro_invoc.args.len(), 3); // FIXME, should be 2
        assert_eq!(rest.len(), 0);
    }
}
