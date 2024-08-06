use std::ops::Deref;
use std::path::PathBuf;
use std::slice::SliceIndex;

use iresult::IResult;
use many::many;
use parse_error::ParseError;
use parser_trait::Parser;

use crate::ast::tree::*;
use crate::lexer::{Lexer, Span, Token, TokenType};
use crate::Config;

mod and;
mod fns;
mod iresult;
mod many;
mod map;
mod opt;
mod or;
mod parse_error;
mod parser_trait;
mod token_type;
mod tuples;

pub type Input<'a> = ParseCtx<'a>;

#[derive(Clone, Debug)]
pub struct ParseCtx<'a> {
    pub tokens: &'a [Token],
    pub indent_level: usize,
    pub config: &'a Config,
}

impl ParseCtx<'_> {
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn consume(&self) -> Result<(Self, Token), ParseError> {
        if self.tokens.is_empty() {
            return Err(ParseError::UnexpectedEOF);
        }

        Ok((
            ParseCtx {
                tokens: &self.tokens[1..],
                ..*self
            },
            self.tokens[0].clone(),
        ))
    }

    pub fn seek(&self) -> Result<Token, ParseError> {
        if self.tokens.is_empty() {
            return Err(ParseError::UnexpectedEOF);
        }

        Ok(self.tokens[0].clone())
    }

    pub fn seek_nth(&self, n: usize) -> Result<Token, ParseError> {
        if self.tokens.len() < n {
            return Err(ParseError::UnexpectedEOF);
        }

        Ok(self.tokens[n].clone())
    }

    pub fn from<'a>(tokens: &'a [Token], config: &'a Config) -> ParseCtx<'a> {
        ParseCtx {
            tokens,
            indent_level: 0,
            config,
        }
    }
}

/* impl<'a> From<&'a [Token]> for ParseCtx<'a> {
    fn from(tokens: &'a [Token]) -> Self {
        ParseCtx {
            tokens,
            indent_level: 0,
        }
    }
} */

impl Copy for ParseCtx<'_> {}

/* impl<Idx> std::ops::Index<Idx> for ParseCtx<'_>
where
    Idx: std::slice::SliceIndex<[Token]>,
{
    type Output = Self;

    fn index(&self, index: Idx) -> &Self::Output {
        &ParseCtx {
            tokens: &self.tokens[index],
            indent_level: self.indent_level,
        }
    }
}*/

/* impl<'a> Deref for ParseCtx<'a> {
    type Target = [Token];

    fn deref(&self) -> &'a Self::Target {
        self.tokens
    }
} */

fn parse_function_decl<'a>(stream: Input<'a>) -> IResult<'a, FunctionDecl> {
    (
        TokenType::Arobase.opt(),
        parse_ident,
        TokenType::Equal,
        parse_lambda_decl,
        TokenType::Eol.opt(),
    )
        .map(|(inject_self, ident, _, lambda, _)| FunctionDecl {
            name: ident,
            lambda,
            inject_self: inject_self.is_some(),
        })
        .process(stream)
        .map(|(stream, function_decl)| {
            println!("FUNCTION DECL: {:#?}", function_decl);
            (stream, function_decl)
        })
}

fn parse_ident(stream: Input) -> IResult<Ident> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Ident(name) = &token.token_type {
        println!("IDENT: {:#?}", token);
        Ok((
            stream,
            Ident {
                name: name.clone(),
                span: token.span.clone(),
            },
        ))
    } else {
        Err(ParseError::ExpectedIdent(token.clone()))
    }
}

fn parse_lambda_decl(stream: Input) -> IResult<LambdaDecl> {
    (parse_parameters, TokenType::Arrow, parse_block)
        .map(|(parameters, _, body)| LambdaDecl {
            parameters,
            body,
            shorthand_tokens: None,
        })
        .process(stream)
}

fn parse_parameters(stream: Input) -> IResult<Vec<Pattern>> {
    many((parse_pattern, TokenType::Coma.opt()))
        .map(|pat_vec| pat_vec.into_iter().map(|(pat, _)| pat).collect::<Vec<_>>())
        .process(stream)
}

fn parse_pattern(stream: Input) -> IResult<Pattern> {
    parse_ident
        .map(|ident| Pattern {
            binding: Some(ident.clone()),
            kind: PatternKind::Ident(ident),
        })
        .process(stream)
}

fn parse_block(stream: Input) -> IResult<Block> {
    many((parse_statement, TokenType::Eol))
        .map(|statements| Block {
            statements: statements.into_iter().map(|(stmt, _)| stmt).collect(),
        })
        .process(stream)
}

fn parse_statement(stream: Input) -> IResult<Statement> {
    /* parse_assignment
    .map(Statement::Assignment)
    .or(parse_expression.map(Statement::Expression))
    .or(parse_return.map(Statement::Return))
    .or(parse_continue.map(Statement::Continue))
    .or(parse_break.map(Statement::Break))
    .or(TokenType::Eol.map(|_| Statement::EmptyLine)) */
    parse_expression.map(Statement::Expression).process(stream)
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

// Execute parser F if token type is found but does not consume it
fn seek<'a, T, F>(token_type: TokenType, parser: F) -> impl Fn(Input<'a>) -> IResult<'a, T>
where
    F: Fn(Input<'a>) -> IResult<'a, T>,
{
    move |stream| {
        let (_, token) = stream.consume()?;

        if token.token_type == token_type {
            parser(stream)
        } else {
            Err(ParseError::UnexpectedToken(token.clone()))
        }
    }
}

/* fn parse_macro_decl(stream: Input) -> IResult<MacroDecl> {
    (parse_ident, TokenType::Equal, parse_block)
        .map(|(name, _, body)| MacroDecl { name, body })
        .process(stream)
} */

fn parse_indent(stream: Input) -> IResult<()> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Indent(level) = &token.token_type {
        if *level as usize != stream.indent_level {
            return Err(ParseError::UnexpectedIndent(*level));
        }

        Ok((stream, ()))
    } else {
        Err(ParseError::UnexpectedToken(token.clone()))
    }
}

fn ignore_empty_lines(stream: Input) -> IResult<()> {
    many((TokenType::Indent(stream.indent_level as u8), TokenType::Eol))
        .map(|_| ())
        .process(stream)
}

fn parse_top_level(stream: Input) -> IResult<TopLevel> {
    println!("TOP LEVEL TOKENS: {:#?}", stream.tokens);

    (
        ignore_empty_lines,
        parse_indent,
        (
            /* seek(TokenType::Keyword("macro".to_string()), parse_macro_decl)
            .map(|macro_decl| TopLevel {
                ident: macro_decl.name.clone(),
                kind: TopLevelKind::MacroDecl(macro_decl),
            }) */
            /* .or(
                seek(TokenType::Keyword("struct".to_string()), parse_struct_decl).map(|struct_decl| {
                    TopLevel {
                        ident: Ident::default(), // FIXME
                        kind: TopLevelKind::StructDecl(struct_decl),
                    }
                }),
            )
            .or(
                seek(TokenType::Keyword("enum".to_string()), parse_enum_decl).map(|enum_decl| {
                    TopLevel {
                        ident: Ident::default(), // FIXME
                        kind: TopLevelKind::EnumDecl(enum_decl),
                    }
                }),
            )
            .or(
                seek(TokenType::Keyword("trait".to_string()), parse_trait_decl).map(|trait_decl| {
                    TopLevel {
                        ident: Ident::default(), // FIXME
                        kind: TopLevelKind::TraitDecl(trait_decl),
                    }
                }),
            )
            .or(
                seek(TokenType::Keyword("impl".to_string()), parse_impl).map(|impl_| {
                    TopLevel {
                        ident: Ident::default(), // FIXME
                        kind: TopLevelKind::Impl(impl_),
                    }
                }),
            )
            .or(seek(
                TokenType::Keyword("infix".to_string()),
                parse_infix_operator,
            )
            .map(|(precedence, op)| {
                TopLevel {
                    ident: Ident::default(), // FIXME
                    kind: TopLevelKind::InfixOperator(precedence, op),
                }
            }))
            .or(
                seek(TokenType::Keyword("mod".to_string()), parse_module).map(|module| TopLevel {
                    ident: module.name.clone().unwrap_or_default(),
                    kind: TopLevelKind::Module(ModuleDecl(module)),
                }),
            )
            .or(
                seek(TokenType::Keyword("extern".to_string()), parse_function_sig).map(
                    |function_sig| TopLevel {
                        ident: function_sig.name.clone(),
                        kind: TopLevelKind::Extern(function_sig),
                    },
                ),
            )
            .or(
                seek(TokenType::Keyword("type".to_string()), parse_new_type).map(
                    |(parse_type_inner, parse_type)| {
                        TopLevel {
                            ident: Ident::default(), // FIXME
                            kind: TopLevelKind::NewType(parse_type_inner, parse_type),
                        }
                    },
                ),
            ) */
            parse_function_decl,
        ),
        ignore_empty_lines,
    )
        .map(|(_, _, (function_decl,), _)| TopLevel {
            ident: function_decl.name.clone(),
            kind: TopLevelKind::FunctionDecl(function_decl),
        })
        .process(stream)
}

fn parse_struct_decl(stream: Input) -> IResult<StructDecl> {
    (
        TokenType::Keyword("struct".to_string()),
        parse_type_inner,
        TokenType::Eol,
        many(parse_struct_decl_field),
    )
        .map(|(_, name, _, fields)| StructDecl { name, fields })
        .process(stream)
}

fn parse_struct_decl_field(stream: Input) -> IResult<StructDeclField> {
    (
        TokenType::Indent(4),
        TokenType::Operator("<".to_string()).opt(),
        parse_ident,
        TokenType::Colon,
        parse_type,
        (TokenType::Equal, parse_expression).opt(),
        TokenType::Eol,
    )
        .map(|(_, public, name, _, ty, expr_opt, _)| StructDeclField {
            name,
            ty,
            public: public.is_some(),
            default: expr_opt.map(|(_, expr)| expr),
        })
        .process(stream)
}

fn parse_expression(stream: Input) -> IResult<Expression> {
    parse_unary_expr.map(Expression::UnaryExpr).process(stream)
}

fn parse_unary_expr(stream: Input) -> IResult<UnaryExpr> {
    parse_primary_expr
        .map(UnaryExpr::PrimaryExpr)
        .process(stream)
}

fn parse_primary_expr(stream: Input) -> IResult<PrimaryExpr> {
    (
        parse_operand,
        many((parse_secondary, TokenType::Dot)),
        (TokenType::Colon, parse_type).opt(),
    )
        .map(|(operand, secondaries, type_annotation)| PrimaryExpr {
            operand,
            secondaries: Some(secondaries.into_iter().map(|(op, _)| op).collect()),
            type_annotation: type_annotation.map(|(_, ty)| ty),
        })
        .process(stream)
}

fn parse_operand(stream: Input) -> IResult<Operand> {
    parse_literal
        .map(Operand::Literal)
        .or(parse_ident_path.map(Operand::Ident))
        .process(stream)
}

#[derive(Debug)]
struct Delimited<P, D> {
    parser: P,
    delimiter: D,
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

        Ok((remaining_tokens, items))
    }
}

fn delimited<P, D>(parser: P, delimiter: D) -> Delimited<P, D> {
    Delimited { parser, delimiter }
}

fn parse_ident_path(stream: Input) -> IResult<IdentifierPath> {
    delimited(parse_ident_or_type, TokenType::DoubleColon)
        .map(|idents| IdentifierPath { path: idents })
        .process(stream)
}

fn parse_ident_or_type(stream: Input) -> IResult<IdentOrType> {
    parse_ident
        .map(IdentOrType::Ident)
        .or(parse_type.map(IdentOrType::Type))
        .process(stream)
}

fn parse_secondary(stream: Input) -> IResult<SecondaryExpr> {
    parse_arguments
        .map(SecondaryExpr::Arguments)
        .process(stream)
}

fn parse_arguments(stream: Input) -> IResult<Vec<Argument>> {
    (
        TokenType::OpenParen,
        many(parse_expression),
        TokenType::CloseParen,
    )
        .map(|(_, args, _)| args.into_iter().map(|arg| Argument { arg }).collect())
        .process(stream)
}

fn parse_operator(stream: Input) -> IResult<String> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Operator(name) = &token.token_type {
        Ok((stream, name.clone()))
    } else {
        Err(ParseError::ExpectedOperator(token.clone()))
    }
}

fn parse_literal(stream: Input) -> IResult<Literal> {
    let (stream, span) = get_span(stream)?;

    parse_bool_literal
        .map(LiteralKind::Bool)
        .or(parse_int_literal.map(LiteralKind::Number))
        .or(parse_float_literal.map(LiteralKind::Float))
        .or(parse_array_literal.map(LiteralKind::Array))
        .or(parse_string_literal.map(LiteralKind::String))
        .or(parse_char_literal.map(LiteralKind::Char))
        .map(|kind| Literal {
            kind,
            span: span.clone(),
        })
        .process(stream)
}

fn parse_bool_literal(stream: Input) -> IResult<bool> {
    TokenType::Keyword("true".to_string())
        .map(|_| true)
        .or(TokenType::Keyword("false".to_string()).map(|_| false))
        .process(stream)
}

fn parse_int_literal(stream: Input) -> IResult<u64> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Number(value) = &token.token_type {
        Ok((stream, value.parse().unwrap()))
    } else {
        Err(ParseError::ExpectedNumber(token.clone()))
    }
}

fn parse_float_literal(stream: Input) -> IResult<f64> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Float(value) = &token.token_type {
        Ok((stream, value.parse().unwrap()))
    } else {
        Err(ParseError::UnexpectedToken(token.clone()))
    }
}

fn parse_array_literal(stream: Input) -> IResult<Array> {
    (
        TokenType::OpenBracket,
        many(parse_expression),
        TokenType::CloseBracket,
    )
        .map(|(_, elements, _)| Array { elements })
        .process(stream)
}

fn parse_string_literal(stream: Input) -> IResult<String> {
    let (stream, token) = stream.consume()?;

    if let TokenType::String(value) = &token.token_type {
        Ok((stream, value.clone()))
    } else {
        Err(ParseError::UnexpectedToken(token.clone()))
    }
}

fn parse_char_literal(stream: Input) -> IResult<String> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Char(value) = &token.token_type {
        Ok((stream, value.clone()))
    } else {
        Err(ParseError::UnexpectedToken(token.clone()))
    }
}

fn get_span(stream: Input) -> IResult<Span> {
    let span = stream.seek()?.span.clone();

    Ok((stream, span))
}

fn parse_type(stream: Input) -> IResult<ParseType> {
    // parse_function_type
    /* .or(parse_array_type)
    .or(parse_tuple_type) */
    // .or(parse_type_inner.map(ParseType::Type))
    parse_type_inner.map(ParseType::Type).process(stream)
}

/* fn parse_function_type(stream: Input) -> IResult<ParseType> {
    (
        TokenType::OpenParen,
        parse_type,
        TokenType::Coma,
        parse_type,
        TokenType::CloseParen,
        TokenType::Arrow,
        parse_type,
    )
        .map(|(_, input, _, output, _, _, _)| FunctionType { input, output })
        .process(stream)
} */

fn parse_type_inner(stream: Input) -> IResult<ParseTypeInner> {
    (
        get_span,
        parse_token_type,
        many((parse_type, TokenType::Coma.opt())),
    )
        .map(|(span, name, generics)| ParseTypeInner {
            name,
            generics: generics.into_iter().map(|(name, _)| name).collect(),
            span,
        })
        .process(stream)
}

fn parse_token_type(stream: Input) -> IResult<String> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Type(name) = &token.token_type {
        Ok((stream, name.clone()))
    } else {
        Err(ParseError::ExpectedType(token.clone()))
    }
}

fn parse_program(stream: Input) -> IResult<Program> {
    parse_module_inline
        .map(|module| Program { module })
        .process(stream)
}

fn parse_module_inline(stream: Input) -> IResult<Module> {
    (many(parse_top_level), TokenType::Eof)
        .map(|(top_levels, _)| Module {
            name: None,
            top_levels,
            comment: None,
            is_inline: true,
            filepath: None,
        })
        .process(stream)
}
fn parse_module(stream: Input) -> IResult<Module> {
    (
        TokenType::Keyword("mod".to_string()),
        parse_ident,
        TokenType::Eol,
        many(parse_top_level),
        TokenType::Eof,
    )
        .map(|(_, name, _, top_levels, _)| Module {
            name: Some(name),
            top_levels,
            comment: None,
            is_inline: false,
            filepath: None,
        })
        .process(stream)
}

/* pub fn parse_root_file(config: &Config) -> IResult<Program> {
    let file_path = config.entry_file.clone();

    let file = std::fs::read_to_string(file_path.clone())
        .map_err(|_e| ParseError::UnknownFile(file_path.to_str().unwrap().to_string()))?;

    let lexer = Lexer::new(file_path, &file).map_err(ParseError::Lexer)?;

    parse(lexer, config)
} */
/*
pub fn parse_file(file_path: PathBuf, parse_ctx: ParseCtx) -> IResult<Module> {
    let file = std::fs::read_to_string(file_path.clone())
        .map_err(|_e| ParseError::UnknownFile(file_path.to_str().unwrap().to_string()))?;

    let lexer = Lexer::new(file_path, &file).map_err(ParseError::Lexer)?;

    parse(lexer, config)
}

#[allow(dead_code)]
pub fn parse_string(input: &str) -> IResult<Program> {
    let lexer = Lexer::new(PathBuf::new(), input).map_err(ParseError::Lexer)?;

    parse(lexer, config)
} */

pub fn parse(config: &Config) -> Result<Program, ParseError> {
    let file_path = config.entry_file.clone();

    let file = std::fs::read_to_string(file_path.clone())
        .map_err(|_e| ParseError::UnknownFile(file_path.to_str().unwrap().to_string()))?;

    let mut lexer = Lexer::new(file_path, &file).map_err(ParseError::Lexer)?;
    let tokens = lexer.collect().map_err(ParseError::Lexer)?;

    /* if parse_ctx.config.has_debug_print(DebugPrint::Tokens) {
        println!("{:#?}", tokens);
    } */

    // parse_ctx.deduce_indent_step(&tokens);

    let (_ctx, program) = parse_program.process(ParseCtx::from(&tokens, config))?;

    println!("PROGRAM: {:#?}", program);

    Ok(program)
}

#[cfg(test)]
mod new_parser {
    use super::*;
    use crate::lexer::Span;
    use std::path::PathBuf;

    fn lex_test(input: &str) -> Vec<Token> {
        use crate::lexer::Lexer;

        let mut tokens = Lexer::new(std::path::PathBuf::new(), input)
            .unwrap()
            .with_newline_at_end(false)
            .collect()
            .unwrap();

        //ignore indent
        tokens.remove(0);

        //ignore EOF
        tokens.pop();

        tokens
    }

    #[test]
    fn test_parse_ident() {
        let tokens = lex_test("foo");
        let config = Config::default();

        let (parse_ctx, ident) = parse_ident(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            ident,
            Ident {
                name: "foo".to_string(),
                span: Span {
                    start: 0,
                    end: 3,
                    file_path: PathBuf::new(),
                },
            }
        );

        assert_eq!(parse_ctx.len(), 0);
    }

    #[test]
    fn test_parse_function_decl() {
        let tokens = lex_test("a = foo -> foo\n");
        let config = Config::default();

        let (tokens, function_decl) =
            parse_function_decl(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            function_decl,
            FunctionDecl {
                name: Ident {
                    name: "a".to_string(),
                    span: Span {
                        start: 0,
                        end: 1,
                        file_path: PathBuf::new(),
                    },
                },
                lambda: LambdaDecl {
                    parameters: vec![Pattern {
                        binding: Some(Ident {
                            name: "foo".to_string(),
                            span: Span {
                                start: 4,
                                end: 7,
                                file_path: PathBuf::new(),
                            },
                        }),
                        kind: PatternKind::Ident(Ident {
                            name: "foo".to_string(),
                            span: Span {
                                start: 4,
                                end: 7,
                                file_path: PathBuf::new(),
                            },
                        }),
                    },],
                    body: Block {
                        statements: vec![Statement::Expression(Expression::UnaryExpr(
                            UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(IdentifierPath {
                                    path: vec![IdentOrType::Ident(Ident {
                                        name: "foo".to_string(),
                                        span: Span {
                                            start: 11,
                                            end: 14,
                                            file_path: PathBuf::new(),
                                        },
                                    })]
                                }),
                                secondaries: None,
                                type_annotation: None,
                            })
                        ))]
                    },
                    shorthand_tokens: None,
                },
                inject_self: false,
            }
        );

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn top_level() {
        let tokens = lex_test("a = foo -> foo\n");
        let config = Config::default();

        let (tokens, top_level) = parse_top_level(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            top_level,
            TopLevel {
                ident: Ident {
                    name: "a".to_string(),
                    span: Span {
                        start: 0,
                        end: 1,
                        file_path: PathBuf::new(),
                    },
                },
                kind: TopLevelKind::FunctionDecl(FunctionDecl {
                    name: Ident {
                        name: "a".to_string(),
                        span: Span {
                            start: 0,
                            end: 1,
                            file_path: PathBuf::new(),
                        },
                    },
                    lambda: LambdaDecl {
                        parameters: vec![Pattern {
                            binding: Some(Ident {
                                name: "foo".to_string(),
                                span: Span {
                                    start: 4,
                                    end: 7,
                                    file_path: PathBuf::new(),
                                },
                            }),
                            kind: PatternKind::Ident(Ident {
                                name: "foo".to_string(),
                                span: Span {
                                    start: 4,
                                    end: 7,
                                    file_path: PathBuf::new(),
                                },
                            }),
                        },],
                        body: Block {
                            statements: vec![Statement::Expression(Expression::UnaryExpr(
                                UnaryExpr::PrimaryExpr(PrimaryExpr {
                                    operand: Operand::Ident(IdentifierPath {
                                        path: vec![IdentOrType::Ident(Ident {
                                            name: "foo".to_string(),
                                            span: Span {
                                                start: 11,
                                                end: 14,
                                                file_path: PathBuf::new(),
                                            },
                                        })]
                                    }),
                                    secondaries: None,
                                    type_annotation: None,
                                })
                            ))]
                        },
                        shorthand_tokens: None,
                    },
                    inject_self: false,
                }),
            }
        );

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_parse_struct_decl() {
        let tokens = lex_test("struct Foo\n    a: Int\n    b: Int = 0\n");
        let config = Config::default();

        let (tokens, struct_decl) = parse_struct_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(
            struct_decl,
            StructDecl {
                name: ParseTypeInner {
                    name: "Foo".to_string(),
                    generics: vec![],
                    span: Span {
                        start: 7,
                        end: 10,
                        file_path: PathBuf::new(),
                    },
                },
                fields: vec![
                    StructDeclField {
                        name: Ident {
                            name: "a".to_string(),
                            span: Span {
                                start: 15,
                                end: 16,
                                file_path: PathBuf::new(),
                            },
                        },
                        ty: ParseType::Type(ParseTypeInner {
                            name: "Int".to_string(),
                            generics: vec![],
                            span: Span {
                                start: 18,
                                end: 21,
                                file_path: PathBuf::new(),
                            }
                        }),
                        public: false,
                        default: None,
                    },
                    StructDeclField {
                        name: Ident {
                            name: "b".to_string(),
                            span: Span {
                                start: 26,
                                end: 27,
                                file_path: PathBuf::new(),
                            },
                        },
                        ty: ParseType::Type(ParseTypeInner {
                            name: "Int".to_string(),
                            generics: vec![],
                            span: Span {
                                start: 29,
                                end: 32,
                                file_path: PathBuf::new(),
                            }
                        }),
                        public: false,
                        default: Some(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Literal(Literal {
                                kind: LiteralKind::Number(0),
                                span: Span {
                                    start: 35,
                                    end: 36,
                                    file_path: PathBuf::new(),
                                }
                            }),
                            secondaries: Some(vec![]),
                            type_annotation: None,
                        }))),
                    }
                ]
            }
        );

        assert_eq!(tokens.len(), 0);
    }
}
