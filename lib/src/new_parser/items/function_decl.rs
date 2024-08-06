use crate::{
    lexer::TokenType,
    new_parser::{engine::*, FunctionDecl, LambdaDecl, Pattern},
};

use super::{block, ident, pattern};

pub fn function_decl<'a>(stream: Input<'a>) -> IResult<'a, FunctionDecl> {
    (
        TokenType::Arobase.opt(),
        ident,
        TokenType::Equal,
        lambda_decl,
        TokenType::Eol,
    )
        .map(|(inject_self, ident, _, lambda, _)| FunctionDecl {
            name: ident,
            lambda,
            inject_self: inject_self.is_some(),
        })
        .process(stream)
}

pub fn lambda_decl(stream: Input) -> IResult<LambdaDecl> {
    (parameters, TokenType::Arrow, block)
        .map(|(parameters, _, body)| LambdaDecl {
            parameters,
            body,
            shorthand_tokens: None,
        })
        .process(stream)
}

fn parameters(stream: Input) -> IResult<Vec<Pattern>> {
    many((pattern, TokenType::Coma.opt()))
        .map(|pat_vec| pat_vec.into_iter().map(|(pat, _)| pat).collect::<Vec<_>>())
        .process(stream)
}
