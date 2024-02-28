use crate::{
    ast::{Ident, MacroDecl, MacroEntry, MacroInvoc},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        util::{consume_tokens_until, expect_token, parse_vec_of, ParseError},
    },
};

impl Parsable for MacroDecl {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("macro".to_string()))?;

        let (name, mut remaining_tokens) = Ident::parse(remaining_tokens)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        let (entries, remaining_tokens) = parse_vec_of::<MacroEntry>(remaining_tokens, None)?;

        Ok((MacroDecl { name, entries }, remaining_tokens))
    }
}

impl Parsable for MacroEntry {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let mut remaining_tokens = tokens;

        remaining_tokens = expect_token(remaining_tokens, TokenType::Indent(2))?;

        let (defs, mut remaining_tokens) =
            consume_tokens_until(remaining_tokens, TokenType::FatArrow);

        remaining_tokens = expect_token(remaining_tokens, TokenType::FatArrow)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Indent(4))?;

        let (block, remaining_tokens) = consume_tokens_until(remaining_tokens, TokenType::Eol);

        Ok((MacroEntry { defs, block }, remaining_tokens))
    }
}

impl Parsable for MacroInvoc {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let remaining_tokens = expect_token(tokens, TokenType::Operator("$".to_string()))?;

        let (name, remaining_tokens) = Ident::parse(remaining_tokens)?;

        let (args, remaining_tokens) = consume_tokens_until(remaining_tokens, TokenType::Eol);

        Ok((MacroInvoc { name, args }, remaining_tokens))
    }
}
