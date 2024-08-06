use super::{and::And, map::Map, opt::Opt, or::Or, IResult, Input};

pub trait Parser {
    type Output;

    fn process<'a, 'b>(&'b mut self, tokens: Input<'a>) -> IResult<'a, Self::Output>;

    fn or<Parser2>(self, parser2: Parser2) -> Or<Self, Parser2>
    where
        Parser2: Parser,
        Self: Sized,
    {
        Or::new(self, parser2)
    }

    fn map<F, T>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::Output) -> T,
        Self: Sized,
    {
        Map::new(self, f)
    }

    fn and<Parser2, T>(self, parser2: Parser2) -> And<Self, Parser2>
    where
        Parser2: Parser<Output = T>,
        Self: Sized,
    {
        And::new(self, parser2)
    }

    fn opt(self) -> Opt<Self>
    where
        Self: Sized,
    {
        Opt::new(self)
    }
}
