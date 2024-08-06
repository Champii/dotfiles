use crate::lexer::TokenType;
use crate::new_parser::{
    engine::*, Argument, Expression, Operand, PrimaryExpr, SecondaryExpr, UnaryExpr,
};

use super::ident_path;
use super::literal;
use super::parse_type;

pub fn expression(stream: Input) -> IResult<Expression> {
    unary_expr.map(Expression::UnaryExpr).process(stream)
}

pub fn unary_expr(stream: Input) -> IResult<UnaryExpr> {
    primary_expr.map(UnaryExpr::PrimaryExpr).process(stream)
}

pub fn primary_expr(stream: Input) -> IResult<PrimaryExpr> {
    (
        operand,
        many((secondary, TokenType::Dot)),
        (TokenType::Colon, parse_type).opt(),
    )
        .map(|(operand, secondaries, type_annotation)| PrimaryExpr {
            operand,
            secondaries: Some(secondaries.into_iter().map(|(op, _)| op).collect()),
            type_annotation: type_annotation.map(|(_, ty)| ty),
        })
        .process(stream)
}

pub fn operand(stream: Input) -> IResult<Operand> {
    literal
        .map(Operand::Literal)
        .or(ident_path.map(Operand::Ident))
        .process(stream)
}

pub fn secondary(stream: Input) -> IResult<SecondaryExpr> {
    arguments.map(SecondaryExpr::Arguments).process(stream)
}

pub fn arguments(stream: Input) -> IResult<Vec<Argument>> {
    (
        TokenType::OpenParen,
        many(expression),
        TokenType::CloseParen,
    )
        .map(|(_, args, _)| args.into_iter().map(|arg| Argument { arg }).collect())
        .process(stream)
}
