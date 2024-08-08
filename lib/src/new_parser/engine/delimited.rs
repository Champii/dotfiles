use super::{parser_trait::Parser, IResult, Input};

pub struct Delimited<P, D> {
    parser: P,
    delimiter: D,
    at_least_one_result: bool,
}

impl<P, D> Parser for Delimited<P, D>
where
    P: Parser,
    D: Parser,
{
    type Output = Vec<P::Output>;

    fn process<'a, 'b>(&'b mut self, tokens: Input<'a>) -> IResult<'a, Self::Output> {
        let mut remaining_tokens = tokens.clone();
        let mut items = Vec::new();
        // let mut diagnostics = Diagnostics::default();

        let mut remaining_tokens_with_delim = tokens;

        loop {
            if remaining_tokens.is_empty() {
                remaining_tokens = remaining_tokens_with_delim;
                break;
            }

            let (new_remaining_tokens, item) = match self.parser.process(remaining_tokens) {
                Ok((new_remaining_tokens, item)) => (new_remaining_tokens, item),
                Err(e) => {
                    remaining_tokens = remaining_tokens_with_delim;
                    // diagnostics = e;
                    break;
                }
            };

            remaining_tokens = new_remaining_tokens;
            remaining_tokens_with_delim = new_remaining_tokens;

            items.push(item);

            if let Ok((new_remaining_tokens, _)) =
                self.delimiter.process(remaining_tokens_with_delim)
            {
                remaining_tokens = new_remaining_tokens;
            } else {
                break;
            }
        }

        if self.at_least_one_result && items.is_empty() {
            return Err(super::ParseError::ExpectedOneOrMore);
        }

        Ok((remaining_tokens, items))
    }
}

pub fn delimited<P, D>(parser: P, delimiter: D) -> Delimited<P, D> {
    Delimited {
        parser,
        delimiter,
        at_least_one_result: false,
    }
}

pub fn delimited1<P, D>(parser: P, delimiter: D) -> Delimited<P, D> {
    Delimited {
        parser,
        delimiter,
        at_least_one_result: true,
    }
}
