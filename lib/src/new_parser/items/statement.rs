use crate::new_parser::{engine::*, Statement};

use super::expression;

pub fn statement(stream: Input) -> IResult<Statement> {
    /* parse_assignment
    .map(Statement::Assignment)
    .or(parse_expression.map(Statement::Expression))
    .or(parse_return.map(Statement::Return))
    .or(parse_continue.map(Statement::Continue))
    .or(parse_break.map(Statement::Break))
    .or(TokenType::Eol.map(|_| Statement::EmptyLine)) */
    expression.map(Statement::Expression).process(stream)
    /* parse_ident
    .map(|ident| {
        Statement::Expression(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
            operand: Operand::Ident(IdentifierPath {
                path: vec![IdentOrType::Ident(ident)],
            }),
            secondaries: None,
            type_annotation: None,
        })))
    })
    .process(stream) */
}
