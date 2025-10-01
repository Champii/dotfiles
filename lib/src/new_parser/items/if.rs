use crate::new_parser::*;

pub fn parse_condition(stream: Input) -> IResult<Condition> {
    disallow_multiline_fn_call((pattern.followed_by(TokenType::Equal).opt(), expression))
        .map(|(pattern, expression)| Condition {
            pattern,
            expression,
        })
        .process(stream)
}

pub fn parse_if(stream: Input) -> IResult<If> {
    (
        TokenType::Keyword("if".to_string()),
        parse_condition,
        (
            TokenType::Eol.opt(),
            indent.opt(),
            TokenType::Keyword("then".to_string()),
        )
            .opt(),
        block,
        (TokenType::Eol, indent.opt()).opt(),
        parse_else.opt(),
    )
        .map(|(_, condition, _, then, _, else_)| If {
            condition,
            then,
            else_,
        })
        .process(stream)
}

pub fn parse_else(stream: Input) -> IResult<Else> {
    preceded(
        TokenType::Keyword("else".to_string()),
        parse_if
            .map(Box::new)
            .map(Else::If)
            .or(block.map(Else::Block)),
    )
    .process(stream)
}
