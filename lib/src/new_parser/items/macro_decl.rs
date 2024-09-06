use crate::{
    lexer::{Span, Token, TokenType},
    new_parser::{engine::*, Ident, MacroDecl, MacroEntry, MacroFragment, MacroInvoc},
};

use super::{consume_tokens_until, empty_lines, get_span, ident, indent, macro_invoc_token};

pub fn macro_decl(stream: Input) -> IResult<MacroDecl> {
    (
        TokenType::Keyword("macro".to_string()),
        ident,
        TokenType::Eol,
        indented(many(macro_entry)),
    )
        .map(|(_, name, _, entries)| MacroDecl { name, entries })
        .process(stream)
}

pub fn macro_entry(stream: Input) -> IResult<MacroEntry> {
    (
        empty_lines,
        indent,
        parse_macro_head_recursive,
        TokenType::FatArrow,
        TokenType::Eol,
        parse_macro_block_recursive,
    )
        .map(|(_, _, defs, _, _, mut body)| {
            body.push(MacroFragment::Token(Token {
                token_type: TokenType::Eof,
                span: Span::default(),
            }));

            MacroEntry { defs, body }
        })
        .process(stream)
}

pub fn macro_invoc(stream: Input) -> IResult<MacroInvoc> {
    (
        get_span,
        macro_invoc_token,
        consume_tokens_until(TokenType::Indent(0)),
    )
        .map(|(span, name, args)| MacroInvoc {
            name: Ident { name, span },
            args: args
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
                .collect::<Vec<_>>(),
        })
        .process(stream)
}

fn parse_macro_head_recursive(stream: Input<'_>) -> IResult<'_, Vec<MacroFragment>> {
    let (defs, tokens) = parse_macro_head_recursive_inner(stream.tokens, stream)?;

    Ok((Input { tokens, ..stream }, defs))
}

fn parse_macro_block_recursive(stream: Input<'_>) -> IResult<'_, Vec<MacroFragment>> {
    let (block, tokens) = parse_macro_block_recursive_inner(stream.tokens, stream)?;

    Ok((Input { tokens, ..stream }, block))
}

fn parse_macro_head_recursive_inner<'a>(
    tokens: &'a [Token],
    parse_ctx: ParseCtx,
) -> Result<(Vec<MacroFragment>, &'a [Token]), ParseError> {
    let mut defs = Vec::new();
    let mut remaining_tokens = tokens;

    let mut skip_until = 0;
    while let Some(token) = remaining_tokens.first() {
        if skip_until > 0 {
            skip_until -= 1;
            remaining_tokens = &remaining_tokens[1..];
            continue;
        }
        match &token.token_type {
            TokenType::MacroVar(name) => {
                if remaining_tokens.get(1).unwrap().token_type != TokenType::Colon {
                    return Err(ParseError::UnexpectedToken(
                        TokenType::Colon.discriminant().to_string(),
                        remaining_tokens.get(1).unwrap().clone(),
                        // vec![TokenType::Colon],
                    ));
                }

                if remaining_tokens.get(2).unwrap().token_type
                    == TokenType::Ident("ident".to_string())
                {
                    defs.push(MacroFragment::Ident(Ident {
                        name: name.clone(),
                        span: token.span.clone(),
                    }));
                } else if remaining_tokens.get(2).unwrap().token_type
                    == TokenType::Ident("expr".to_string())
                {
                    defs.push(MacroFragment::Expr(Ident {
                        name: name.clone(),
                        span: token.span.clone(),
                    }));
                } else if remaining_tokens.get(2).unwrap().token_type
                    == TokenType::Ident("ty".to_string())
                {
                    defs.push(MacroFragment::Type(Ident {
                        name: name.clone(),
                        span: token.span.clone(),
                    }));
                } else {
                    return Err(ParseError::UnexpectedToken(
                        TokenType::Ident(String::new()).discriminant().to_string(),
                        remaining_tokens.get(2).unwrap().clone(),
                        /* vec![
                            TokenType::Ident("ident".to_string()),
                            TokenType::Ident("expr".to_string()),
                            TokenType::Ident("type".to_string()),
                        ], */
                    ));
                }

                skip_until = 3;

                continue;
            }
            TokenType::MacroRepeatOpen => {
                let (inner_block, new_remaining_tokens) =
                    parse_macro_head_recursive_inner(&remaining_tokens[1..], parse_ctx)?;

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

fn parse_macro_block_recursive_inner<'a>(
    tokens: &'a [Token],
    parse_ctx: ParseCtx,
) -> Result<(Vec<MacroFragment>, &'a [Token]), ParseError> {
    let mut block = Vec::new();
    let mut remaining_tokens = tokens;

    while let Some(token) = remaining_tokens.first() {
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
                    parse_macro_block_recursive_inner(&remaining_tokens[1..], parse_ctx)?;

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
                    if level == 0 || level == parse_ctx.indent_step as u8 {
                        // the definition is over
                        return Ok((block, remaining_tokens));
                    }
                    token.token_type = TokenType::Indent(level - (parse_ctx.indent_step as u8 * 2));
                }

                block.push(MacroFragment::Token(token));
                remaining_tokens = &remaining_tokens[1..];
            }
        }
    }

    Ok((block, remaining_tokens))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::{lexer::Lexer, Config};

    fn lex(input: &str) -> Vec<Token> {
        Lexer::new(PathBuf::new(), input)
            .unwrap()
            .with_newline_at_end(false)
            .collect()
            .unwrap()
    }

    #[test]
    fn test_parse_macro_decl() {
        let input = "macro mymacro\n    $a:ident, $b:ty =>\n        statement";
        let tokens = lex(input);
        let tokens = &tokens[1..]; // skip the Indent(0)
        let config = Config::default();

        let (rest, macro_decl) = macro_decl(ParseCtx::from(tokens, &config)).unwrap();

        assert_eq!(macro_decl.name.name, "mymacro");
        assert_eq!(macro_decl.entries.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_macro_entry() {
        let input = "$a:ident =>\n        statement";
        let tokens = lex(input);
        // let tokens = &tokens[1..]; // skip the Indent(0)
        let config = Config::default();

        let mut parse_ctx = ParseCtx::from(&tokens, &config);

        // Hack to force the indent step
        parse_ctx.indent_step = 4;

        let (rest, macro_entry) = macro_entry(parse_ctx).unwrap();

        assert_eq!(macro_entry.defs.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_macro_invoc() {
        let input = "%mymacro\n    a\n    b";
        let tokens = lex(input);
        let tokens = &tokens[1..]; // skip the Indent(0)
        let config = Config::default();

        let (rest, macro_invoc) = macro_invoc(ParseCtx::from(tokens, &config)).unwrap();

        assert_eq!(macro_invoc.name.name, "mymacro");
        assert_eq!(macro_invoc.args.len(), 3); // FIXME, should be 2
        assert_eq!(rest.len(), 0);
    }
}
