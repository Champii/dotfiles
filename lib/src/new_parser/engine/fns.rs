use super::{parser_trait::Parser, IResult, Input};

impl<F, T> Parser for F
where
    F: FnMut(Input<'_>) -> IResult<T>,
{
    type Output = T;

    fn process<'a, 'b>(&'b mut self, tokens: Input<'a>) -> IResult<'a, Self::Output> {
        self(tokens)
    }
}
