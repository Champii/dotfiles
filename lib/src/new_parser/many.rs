use super::{parser_trait::Parser, IResult, Input, Token};

pub struct Many<P> {
    parser: P,
}

impl<P> Parser for Many<P>
where
    P: Parser,
{
    type Output = Vec<P::Output>;

    fn process<'a, 'b>(&'b mut self, mut tokens: Input<'a>) -> IResult<'a, Self::Output> {
        let mut output = Vec::new();
        println!("START MANY {:#?}", tokens);

        loop {
            if tokens.is_empty() {
                break;
            }

            if let Ok((new_tokens, t)) = self.parser.process(tokens) {
                tokens = new_tokens;
                println!("REMAINING TOKENS {:#?}", new_tokens);
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
