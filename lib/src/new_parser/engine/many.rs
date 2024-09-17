use super::{parser_trait::Parser, IResult, Input};

pub struct Many<P> {
    parser: P,
    at_least_one_result: bool,
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

        if self.at_least_one_result && output.is_empty() {
            return Err(super::ParseError::ExpectedOneOrMore);
        }

        Ok((tokens, output))
    }
}

pub fn many<'a, T: Parser>(parser: T) -> Many<T> {
    Many {
        parser,
        at_least_one_result: false,
    }
}

pub fn many1<'a, T: Parser>(parser: T) -> Many<T> {
    Many {
        parser,
        at_least_one_result: true,
    }
}
