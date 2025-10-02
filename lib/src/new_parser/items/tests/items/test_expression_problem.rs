use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;

#[test]
fn test_simple_function() {
    let input = "main = ->\n    1\n";
    let config = Config::default();
    
    let result = parse_string(input, &config);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_function_with_method_call() {
    let input = "main = ->\n    foo\n        .bar\n";
    let config = Config::default();
    
    let result = parse_string(input, &config);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_function_with_method_and_arg() {
    let input = "main = ->\n    foo\n        .bar 1\n";
    let config = Config::default();
    
    let result = parse_string(input, &config);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_function_with_lambda_arg() {
    let input = "main = ->\n    foo\n        .bar ->\n            1\n";
    let config = Config::default();

    let result = parse_string(input, &config);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_function_with_param_lambda_arg() {
    let input = "main = ->\n    foo\n        .bar x ->\n            1\n";
    let config = Config::default();
    
    let result = parse_string(input, &config);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

#[test]
fn test_expression_problem_full() {
    let input = "main = ->\n    foo\n        .bar lol ->\n            mdr\n        .haha\n";
    let config = Config::default();
    
    let result = parse_string(input, &config);
    assert!(result.is_ok(), "Failed: {:?}", result.err());
}

