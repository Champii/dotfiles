use crate::{lexer::TokenType, new_parser::*};

use super::top_level;

pub fn module_inline(stream: Input) -> IResult<Module> {
    (many(top_level), TokenType::Eof)
        .map(|(top_levels, _)| Module {
            name: None,
            top_levels,
            is_inline: true,
            filepath: None,
        })
        .process(stream)
}

pub fn module(stream: Input) -> IResult<Module> {
    (
        TokenType::Keyword("mod".to_string()),
        ident,
        TokenType::Eol,
        indented(many(top_level)),
    )
        .map(|(_, name, _, top_levels)| {
            if top_levels.is_empty() {
                let module_path = stream
                    .sibling_module_filepath(&name.name)
                    .unwrap_or_else(|_| {
                        panic!("Module '{}' is empty and no sibling file found", name)
                    });
                let mut module = parse_module(module_path, &stream.config).unwrap();
                module.name = Some(name);
                module
            } else {
                Module {
                    name: Some(name),
                    top_levels,
                    is_inline: false,
                    filepath: None,
                }
            }
        })
        .process(stream)
}
