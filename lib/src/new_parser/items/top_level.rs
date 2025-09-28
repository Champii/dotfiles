use crate::{
    lexer::TokenType,
    new_parser::{
        engine::*,
        items::{primitives::indent, utils::empty_lines},
        ModuleDecl, TopLevel,
    },
};

use super::{
    enum_decl, function_decl, function_sig, macro_decl, macro_invoc, module, operator, parse_type,
    parse_type_inner, path, primitives, r#impl, r#trait, struct_decl,
};

pub fn top_level(stream: Input) -> IResult<TopLevel> {
    (
        empty_lines,
        indent,
        function_decl
            .map(TopLevel::FunctionDecl)
            .or(struct_decl.map(TopLevel::StructDecl))
            .or(macro_decl.map(TopLevel::MacroDecl))
            .or(macro_invoc.map(TopLevel::MacroInvoc))
            .or(enum_decl.map(TopLevel::EnumDecl))
            .or(r#trait.map(TopLevel::TraitDecl))
            .or(r#impl.map(TopLevel::Impl))
            .or(infix_operator_decl
                .map(|(precedence, name)| TopLevel::InfixOperator(precedence, name)))
            .or(
                preceded(TokenType::Keyword("extern".to_string()), function_sig)
                    .map(TopLevel::Extern),
            )
            .or(module.map(ModuleDecl).map(TopLevel::Module))
            .or((
                TokenType::Keyword("type".to_string()),
                parse_type_inner,
                TokenType::Equal,
                parse_type,
                TokenType::Eol,
            )
                .map(|(_, name, _, ty, _)| TopLevel::NewType(name, ty)))
            .or(preceded(
                TokenType::Operator(">".to_string()),
                followed(path, TokenType::Eol),
            )
            .map(TopLevel::Import))
            .or(preceded(
                TokenType::Operator("<".to_string()),
                followed(path, TokenType::Eol),
            )
            .map(TopLevel::Export))
            .or(macro_invoc.map(TopLevel::MacroInvoc))
            .or(function_sig.map(TopLevel::FunctionSig)),
        empty_lines,
    )
        .map(|(_, _, top_level, _)| top_level)
        .process(stream)
}

pub fn infix_operator_decl(stream: Input) -> IResult<(u8, String)> {
    (
        TokenType::Keyword("infix".to_string()),
        primitives::int.assert(|precedence| *precedence <= 9),
        operator,
        TokenType::Eol,
    )
        .map(|(_, precedence, name, _)| (precedence as u8, name.value))
        .process(stream)
}

#[cfg(test)]
mod parse_top_level {
    use crate::{
        new_parser::{lex_test, lex_test_toplevel, Path},
        Config,
    };

    use super::*;

    #[test]
    fn parse_infix_operator() {
        let input = "infix 5 |>\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, (precedence, name)) = infix_operator_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(precedence, 5);
        assert_eq!(name, "|>".to_string());
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn extern_sig() {
        let input = "extern toto : Toto -> Tata\n";
        let tokens = lex_test_toplevel(input);
        let config = Config::default();

        let (rest, _top_level) = top_level.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_import_syntax() {
        let input = "> std::collections::HashMap\n";
        let tokens = lex_test_toplevel(input);
        let config = Config::default();

        let (rest, top_level) = top_level.process(ParseCtx::from(&tokens, &config)).unwrap();

        match top_level {
            TopLevel::Import(path) => {
                // Test passes if we get an import
                match path {
                    Path::Type(type_path) => {
                        assert!(type_path.path.len() > 0);
                    }
                    Path::Ident(ident_path) => {
                        assert!(ident_path.path.len() > 0);
                    }
                }
            }
            _ => panic!("Expected import, got: {:?}", top_level),
        }

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_export_syntax() {
        let input = "< MyStruct\n";
        let tokens = lex_test_toplevel(input);
        let config = Config::default();

        let (rest, top_level) = top_level.process(ParseCtx::from(&tokens, &config)).unwrap();

        match top_level {
            TopLevel::Export(path) => {
                // Test passes if we get an export
                match path {
                    Path::Type(type_path) => {
                        assert!(type_path.path.len() > 0);
                    }
                    Path::Ident(ident_path) => {
                        assert!(ident_path.path.len() > 0);
                    }
                }
            }
            _ => panic!("Expected export, got: {:?}", top_level),
        }

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_type_alias() {
        let input = "type MyInt = Int32\n";
        let tokens = lex_test_toplevel(input);
        let config = Config::default();

        let (rest, top_level) = top_level.process(ParseCtx::from(&tokens, &config)).unwrap();

        match top_level {
            TopLevel::NewType(name, _ty) => {
                assert_eq!(name.name, "MyInt");
                // Test passes if we get a type alias
            }
            _ => panic!("Expected type alias, got: {:?}", top_level),
        }

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_multiple_imports() {
        let input = "> std::fs::File\n> std::io::Write\n";
        let tokens = lex_test_toplevel(input);
        let config = Config::default();

        // Parse first import
        let (rest, top_level1) = top_level.process(ParseCtx::from(&tokens, &config)).unwrap();
        match top_level1 {
            TopLevel::Import(_) => {
                // Test passes for first import
            }
            _ => panic!("Expected first import, got: {:?}", top_level1),
        }

        // Parse second import
        let (rest, top_level2) = top_level.process(rest).unwrap();
        match top_level2 {
            TopLevel::Import(_) => {
                // Test passes for second import
            }
            _ => panic!("Expected second import, got: {:?}", top_level2),
        }

        assert_eq!(rest.len(), 0);
    }
}
