use crate::{
    ast::{FunctionDecl, MacroDecl, MacroInvoc, TopLevel},
    lexer::Token,
    parser::{parsable::Parsable, util::ParseError},
};

impl Parsable for TopLevel {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        if let Ok((function_decl, new_tokens)) = FunctionDecl::parse(tokens) {
            Ok((TopLevel::FunctionDecl(function_decl), new_tokens))
        } else if let Ok((macro_decl, new_tokens)) = MacroDecl::parse(tokens) {
            Ok((TopLevel::MacroDecl(macro_decl), new_tokens))
        } else if let Ok((macro_invoc, new_tokens)) = MacroInvoc::parse(tokens) {
            Ok((TopLevel::MacroInvoc(macro_invoc), new_tokens))
        } else {
            Err(ParseError::UnexpectedToken(tokens[0].clone()))
        }
    }
}
