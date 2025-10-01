use crate::{lexer::TokenType, new_parser::engine::*};

pub fn parenthesis<P: Parser>(parser: P) -> impl FnMut(Input) -> IResult<P::Output> {
    let mut parser = (TokenType::OpenParen, parser, TokenType::CloseParen).map(|(_, x, _)| x);

    move |mut input: Input| {
        let was_in_arg_list = input.inside_argument_list;
        let result = parser.process(input)?;
        let (mut output_stream, value) = result;

        // If we just closed a paren while inside an argument list,
        // set the flag so the next dot will close the argument list
        if was_in_arg_list {
            output_stream.after_closing_paren = true;
        }

        Ok((output_stream, value))
    }
}
