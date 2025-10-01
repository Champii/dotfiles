use crate::lexer::{Span, Token, TokenType};
use crate::new_parser::{
    engine::*, Argument, Expression, Ident, IdentOrNumber, Operand, PrimaryExpr, SecondaryExpr,
    Tuple, UnaryExpr,
};

use super::{
    block, function_shorthand, get_span, ident, ident_path, indent, instance, int, lambda_decl,
    native_operator, operator, parenthesis, r#loop, r#match,
};
use super::{literal, stuck_operator_token};
use super::{parse_if, parse_type, indent_token};

pub fn expression(stream: Input) -> IResult<Expression> {
    (
        unary_expr,
        (operator, expression)
            .or(preceded(
                TokenType::Eol,
                indented(preceded(indent, (operator, expression))),
            ))
            .or(preceded(
                TokenType::Eol,
                preceded(indent, (operator, expression)),
            ))
            .opt(),
    )
        .map(|(unary, binop_opt)| {
            if let Some((op, expr)) = binop_opt {
                Expression::BinopExpr(unary, op, Box::new(expr))
            } else {
                Expression::UnaryExpr(unary)
            }
        })
        .process(stream)
}

pub fn unary_expr(stream: Input) -> IResult<UnaryExpr> {
    (stuck_operator_token, unary_expr)
        .map(|(op, unary)| UnaryExpr::UnaryExpr(op, Box::new(unary)))
        .or(primary_expr.map(UnaryExpr::PrimaryExpr))
        .process(stream)
}

pub fn primary_expr(stream: Input) -> IResult<PrimaryExpr> {
    (
        operand,
        many(secondary),
        preceded(TokenType::Colon, parse_type).opt(),
    )
        .map(|(operand, secondaries, type_annotation)| PrimaryExpr {
            operand,
            secondaries: if secondaries.is_empty() {
                None
            } else {
                Some(secondaries)
            },
            type_annotation,
        })
        .process(stream)
}

pub fn operand(stream: Input) -> IResult<Operand> {
    parse_if
        .map(Box::new)
        .map(Operand::If)
        .or(r#loop.map(Box::new).map(Operand::Loop))
        .or(r#match.map(Box::new).map(Operand::Match))
        .or(preceded(TokenType::Keyword("unsafe".to_string()), block).map(Operand::Unsafe))
        .or(self_ident)
        .or(instance.map(Operand::Instance))
        .or(tuple.map(Operand::Tuple))
        .or(function_shorthand.map(Operand::LambdaDecl))
        .or(parenthesis(reset_inside_argument_list(expression))
            .map(Box::new)
            .map(Operand::Expression))
        // TODO: disallow function calls after literal
        .or(literal.map(Operand::Literal))
        .or(lambda_decl.map(Operand::LambdaDecl))
        .or(native_operator.map(Operand::NativeOperator))
        .or(preceded(not(operator), ident_path.map(Operand::Ident)))
        .process(stream)
}

pub fn tuple(stream: Input) -> IResult<Tuple> {
    parenthesis(separated1(expression, TokenType::Coma))
        .map(|elements| Tuple { elements })
        .process(stream)
        .map(|(stream, tuple)| {
            if tuple.elements.len() < 2 {
                Err(ParseError::UnexpectedToken(
                    TokenType::OpenParen.discriminant().to_string(),
                    Token {
                        token_type: TokenType::OpenParen,
                        span: Span::default(),
                    },
                ))
            } else {
                Ok((stream, tuple))
            }
        })?
}

pub fn self_ident(stream: Input) -> IResult<Operand> {
    preceded(TokenType::Arobase, ident.map(Operand::SelfIdent))
        .or((get_span, TokenType::Arobase).map(|(span, _)| {
            Operand::SelfIdent(Ident {
                name: "self".to_string(),
                span,
            })
        }))
        .process(stream)
}

pub fn secondary(stream: Input) -> IResult<SecondaryExpr> {
    // Check for argument list short circuit on multiline dots
    // For inline argument lists (e.g., .method1 a), multiline dots should always close the argument list
    // For multiline argument lists, only close if the dot is at the method chain level
    if stream.inside_argument_list {
        if let Ok((_, (_, indent_level, _))) = (TokenType::Eol, indent_token, TokenType::Dot).process(stream) {
            // If we're in an inline argument list, any multiline dot should close it
            if stream.inside_inline_argument_list {
                return arguments_list_short_circuit(stream).and_then(|_| Err(ParseError::ShortCircuit));
            }

            // For multiline argument lists, calculate the method chain indent level
            // Arguments are at stream.indent_level, method chains would be at indent_level - indent_step
            let method_chain_level = if stream.indent_level >= stream.indent_step {
                stream.indent_level - stream.indent_step
            } else {
                0
            };

            // Only close if the dot is at or below the method chain level
            if (indent_level as usize) <= method_chain_level + stream.indent_step {
                return arguments_list_short_circuit(stream).and_then(|_| Err(ParseError::ShortCircuit));
            }
        }
    }

    let result = indice
        .map(SecondaryExpr::Indice)
        .or(dot.map(SecondaryExpr::Dot))
        .or(double_dot.map(SecondaryExpr::DoubleDot))
        .or(arguments.map(SecondaryExpr::Arguments))
        .or(TokenType::Interogation.map(|_| SecondaryExpr::Interogation))
        .process(stream)?;

    let (mut stream, secondary) = result;

    // Clear the after_closing_paren flag after parsing the secondary
    // It should only affect the first secondary after a closing paren
    stream.after_closing_paren = false;

    Ok((stream, secondary))
}

pub fn arguments(stream: Input) -> IResult<Vec<Argument>> {
    TokenType::StuckOperator("!".to_string())
        .map(|_| vec![])
        .or(TokenType::Operator("!".to_string()).map(|_| vec![]))
        .or(preceded(
            not(operator),
            inside_inline_argument_list(separated1(
                expression.map(|arg| Argument { arg }),
                TokenType::Coma,
            )),
        ))
        .or(preceded(
            not(operator),
            preceded(
                not_multi_line_fn_call_short_circuit,
                preceded(
                    TokenType::Eol,
                    // Use context-aware argument parsing to avoid ambiguity
                    multiline_arguments_context_aware,
                ),
            ),
        ))
        .process(stream)
}

pub fn multiline_arguments_context_aware(stream: Input) -> IResult<Vec<Argument>> {
    // Smart argument parsing that prevents ambiguous syntax
    // Arguments must be indented MORE than the current context to avoid ambiguity

    // Check the actual indentation level of the arguments
    let arg_indent_level = if let Ok(token) = stream.seek() {
        if let TokenType::Indent(level) = token.token_type {
            level as usize
        } else {
            0
        }
    } else {
        return Err(ParseError::UnexpectedEOF);
    };

    // Arguments must be indented MORE than the current context
    if arg_indent_level <= stream.indent_level {
        // Not indented at all - definitely not arguments
        return Err(ParseError::UnexpectedIndent(arg_indent_level as u8));
    }

    // Prevent ambiguous cases where arguments could be confused with method chains
    // This only applies at the base level (indent 0) where multiline dots create ambiguity
    // Inside function bodies or other nested contexts, there's no ambiguity
    if stream.indent_step == 4 && stream.indent_level == 0 {
        if arg_indent_level == 4 {
            // At base level, arguments at indent 4 are ambiguous (could be method chain)
            // Arguments at indent 8+ are clearly arguments
            return Err(ParseError::UnexpectedIndent(arg_indent_level as u8));
        }
    }

    // Parse arguments at their actual indent level (which we've already validated)
    // We need to temporarily set the stream's indent level to match the arguments
    let original_indent = stream.indent_level;
    let mut arg_stream = stream;
    arg_stream.indent_level = arg_indent_level;

    let result = inside_argument_list(separated1(
        preceded(indent, expression.map(|arg| Argument { arg })),
        TokenType::Eol,
    )).process(arg_stream);

    // Restore the original indent level in the returned stream
    match result {
        Ok((mut stream, args)) => {
            stream.indent_level = original_indent;
            Ok((stream, args))
        }
        Err(e) => Err(e),
    }
}

pub fn inside_argument_list<P: Parser>(mut parser: P) -> impl FnMut(Input) -> IResult<P::Output> {
    move |mut stream| {
        let old_value = stream.inside_argument_list;
        stream.inside_argument_list = true;

        match parser.process(stream) {
            Ok((mut stream, t)) => {
                stream.inside_argument_list = old_value;

                Ok((stream, t))
            }
            Err(e) => {
                stream.inside_argument_list = old_value;

                Err(e)
            }
        }
    }
}

pub fn inside_inline_argument_list<P: Parser>(mut parser: P) -> impl FnMut(Input) -> IResult<P::Output> {
    move |mut stream| {
        let old_value = stream.inside_argument_list;
        let old_inline_value = stream.inside_inline_argument_list;
        stream.inside_argument_list = true;
        stream.inside_inline_argument_list = true;

        match parser.process(stream) {
            Ok((mut stream, t)) => {
                stream.inside_argument_list = old_value;
                stream.inside_inline_argument_list = old_inline_value;

                Ok((stream, t))
            }
            Err(e) => {
                stream.inside_argument_list = old_value;
                stream.inside_inline_argument_list = old_inline_value;

                Err(e)
            }
        }
    }
}

pub fn reset_inside_argument_list<P: Parser>(
    mut parser: P,
) -> impl FnMut(Input) -> IResult<P::Output> {
    move |mut stream| {
        let old_value = stream.inside_argument_list;
        let old_inline_value = stream.inside_inline_argument_list;
        stream.inside_argument_list = false;
        stream.inside_inline_argument_list = false;

        match parser.process(stream) {
            Ok((mut stream, t)) => {
                stream.inside_argument_list = old_value;
                stream.inside_inline_argument_list = old_inline_value;

                Ok((stream, t))
            }
            Err(e) => {
                stream.inside_argument_list = old_value;
                stream.inside_inline_argument_list = old_inline_value;

                Err(e)
            }
        }
    }
}

pub fn disallow_multiline_fn_call<P: Parser>(
    mut parser: P,
) -> impl FnMut(Input) -> IResult<P::Output> {
    move |mut stream| {
        let old_value = stream.disallowed_multiline_fn_call;
        stream.disallowed_multiline_fn_call = true;

        match parser.process(stream) {
            Ok((mut stream, t)) => {
                stream.disallowed_multiline_fn_call = old_value;

                Ok((stream, t))
            }
            Err(e) => {
                stream.disallowed_multiline_fn_call = old_value;

                Err(e)
            }
        }
    }
}

pub fn not_multi_line_fn_call_short_circuit(stream: Input) -> IResult<()> {
    if stream.disallowed_multiline_fn_call {
        Err(ParseError::ShortCircuit)
    } else {
        Ok((stream, ()))
    }
}

pub fn arguments_list_short_circuit(mut stream: Input) -> IResult<()> {
    stream.argument_list_short_circuit()?;

    Ok((stream, ()))
}

pub fn dot(stream: Input) -> IResult<IdentOrNumber> {
    // Short-circuit inline dots after closing paren in argument context (e.g., foo(x).method)
    // This prevents .method from being parsed as part of the argument
    if stream.after_closing_paren && stream.inside_argument_list {
        if let Ok(_) = TokenType::Dot.process(stream) {
            return arguments_list_short_circuit(stream).and_then(|_| Err(ParseError::ShortCircuit));
        }
    }

    preceded(
        // First try: Eol + Indent + arguments_list_short_circuit + Dot (for closing argument lists)
        (TokenType::Eol, preceded(arguments_list_short_circuit, (indent_token, TokenType::Dot)))
            .map(|_| ())
            .or(
                // Second try: Eol + Indent + Dot (normal multiline dot)
                (TokenType::Eol, indented((indent, TokenType::Dot)))
                    .map(|_| ())
            )
            .or(
                // Third try: inline dots
                TokenType::Dot
                    .or(preceded(arguments_list_short_circuit, TokenType::SpacedDot))
                    .map(|_| ())
            ),
        ident_or_number,
    )
    .process(stream)
}

pub fn double_dot(stream: Input) -> IResult<IdentOrNumber> {
    preceded(
        (TokenType::Eol, indented((indent, TokenType::DoubleDot)))
            .map(|_| ())
            .or(TokenType::DoubleDot.map(|_| ())),
        preceded(arguments_list_short_circuit, ident_or_number),
    )
    .process(stream)
}

pub fn ident_or_number(stream: Input) -> IResult<IdentOrNumber> {
    ident
        .map(IdentOrNumber::Ident)
        .or(int.map(IdentOrNumber::Number))
        .process(stream)
}

pub fn indice(stream: Input) -> IResult<Box<Expression>> {
    preceded(
        TokenType::OpenBracket,
        followed(expression.map(Box::new), TokenType::CloseBracket),
    )
    .process(stream)
}
