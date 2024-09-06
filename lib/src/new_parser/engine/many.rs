use super::{parse_error::ParseError, parser_trait::Parser, IResult, Input};

pub struct Many<P> {
    parser: P,
}

impl<P> Parser for Many<P>
where
    P: Parser,
{
    type Output = Vec<P::Output>;

    fn process<'a>(&mut self, mut tokens: Input<'a>) -> IResult<'a, Self::Output> {
        let mut output = Vec::new();

        loop {
            if tokens.is_empty() {
                break;
            }

            if let Ok((new_tokens, t)) = self.parser.process(tokens) {
                tokens = new_tokens;
                output.push(t);
            } else {
                break;
            }
        }

        Ok((tokens, output))
    }
}

pub fn many<'a, T: Parser>(parser: T) -> Many<T> {
    Many { parser }
}

pub struct Many1<P> {
    parser: P,
}

impl<P> Parser for Many1<P>
where
    P: Parser,
{
    type Output = Vec<P::Output>;

    fn process<'a>(&mut self, mut tokens: Input<'a>) -> IResult<'a, Self::Output> {
        let mut output = Vec::new();

        loop {
            if tokens.is_empty() {
                break;
            }

            if let Ok((new_tokens, t)) = self.parser.process(tokens) {
                tokens = new_tokens;
                output.push(t);
            } else {
                break;
            }
        }

        if output.is_empty() {
            return Err(ParseError::ExpectedOneOrMore);
        }

        Ok((tokens, output))
    }
}

pub fn many1<'a, T: Parser>(parser: T) -> Many1<T> {
    Many1 { parser }
}
