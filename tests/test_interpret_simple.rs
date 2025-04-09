//! This file will test Interpreter module

use rlox::value::LoxValue;
use rlox::scanner::Scanner;
use rlox::parser::Parser;
use rlox::ast::*;
use rlox::interpreter::Interpreter;

use rstest::rstest;

#[rstest()]
/// Test cases for simple expressions
// test literal values
#[case::literal("13813812138", LoxValue::Number(13813812138.0))]
#[case::literal("nil", LoxValue::Null)]
#[case::literal("true", LoxValue::Boolean(true))]
#[case::literal("false", LoxValue::Boolean(false))]
#[case::literal("\"hello\"", LoxValue::String("hello".to_string()))]
#[case::literal("\"hello \\\"world\\\"\"", LoxValue::String("hello \"world\"".to_string()))]
// test simple unary expressions
#[case::simple_unary("--3.6", LoxValue::Number(3.6))]
#[case::simple_unary("-1234.1234", LoxValue::Number(-1234.1234))]
#[case::simple_unary("!true", LoxValue::Boolean(false))]
#[case::simple_unary("!false", LoxValue::Boolean(true))]
#[case::simple_unary("!!false", LoxValue::Boolean(false))]
#[case::simple_unary("!nil", LoxValue::Boolean(true))]
#[case::simple_unary("!\"hello world\"", LoxValue::Boolean(false))]
// test simple binary expressions
#[case::simple_binary("12135.531 + 1234.1234", LoxValue::Number(12135.531 + 1234.1234))]
#[case::simple_binary("1232323.123 - 1238.2", LoxValue::Number(1232323.123-1238.2))]
#[case::simple_binary("1234.1234 * 1234.1234", LoxValue::Number(1234.1234 * 1234.1234))]
#[case::simple_binary("123.4 / 1234.1234", LoxValue::Number(123.4 / 1234.1234))]
#[case::simple_binary("1234.1234 + 2123.21", LoxValue::Number(1234.1234 + 2123.21))]
#[case::simple_binary("\"hello\" + \" \" + \"world\" + \" \" + \"rlox\"", LoxValue::String("hello world rlox".to_string()))]
#[case::simple_binary("1234.1234 == 1234.1234", LoxValue::Boolean(true))]
#[case::simple_binary("1234.1234 != 1234.1234", LoxValue::Boolean(false))]
#[case::simple_binary("1234.1235 > 1234.1234", LoxValue::Boolean(true))]
#[case::simple_binary("1234.1234 < 1234.1235", LoxValue::Boolean(true))]
#[case::simple_binary("1234.1234 >= 1234.1234", LoxValue::Boolean(true))]
#[case::simple_binary("1234.1234 <= 1234.1234", LoxValue::Boolean(true))]
#[case::simple_binary("12 >= 1234.1235", LoxValue::Boolean(false))]
#[case::simple_binary("1234.1 <= 1234.1235", LoxValue::Boolean(true))]
#[case::simple_binary("true == false", LoxValue::Boolean(false))]
#[case::simple_binary("true != false", LoxValue::Boolean(true))]
#[case::simple_binary("nil == 1.0", LoxValue::Boolean(false))]
#[case::simple_binary("nil != 1.0", LoxValue::Boolean(true))]
#[case::simple_binary("\"true\" == \"true\"", LoxValue::Boolean(true))]
#[case::simple_binary("\"true\" != \"false\"", LoxValue::Boolean(true))]
// test complex expressions
#[case::grouping_arith("((3 + 2) * (4 - 1))", LoxValue::Number(15.0))]
#[case::negated_group("-((5 + 2) * 3)", LoxValue::Number(-21.0))]
#[case::bang_on_comparison("!((5 > 3) == false)", LoxValue::Boolean(true))]
#[case::double_bang_eq("!!((1 + 1) == 2)", LoxValue::Boolean(true))]
#[case::string_concat_group("\"hello \" + (\"world\" + \"!\")", LoxValue::String("hello world!".to_string()))]
#[case::deep_logical_expr("((true == true) != false) == true", LoxValue::Boolean(true))]
#[case::bracket_comparisons("(123 + 456) > (100 + 200)", LoxValue::Boolean(true))]
#[case::nested_comparison_eq("123 == (100 + 23)", LoxValue::Boolean(true))]
#[case::nil_equality_expr("((nil != nil) == false)", LoxValue::Boolean(true))]
#[case::complex_numeric_expr("((1234.0 * 0.0) + (10 / 2))", LoxValue::Number(5.0))]
#[case::string_number_logic("(\"abc\" != nil) == true", LoxValue::Boolean(true))]
#[case::negation_of_logic("!(1234.0 > 1234.0)", LoxValue::Boolean(true))]
fn test_expr(#[case] source: &str, #[case] expected: LoxValue) {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let result = expression.accept(&mut interpreter).unwrap();
    assert_eq!(result, expected);
}


#[rstest]
/// These expressions should cause runtime errors (e.g. type errors, division by zero)
#[case::string_plus_number("\"hello\" + 123")]
#[case::string_minus_number("\"hello\" - 123")]
#[case::boolean_plus_string("true + \"world\"")]
#[case::number_plus_string("123 + \"world\"")]
#[case::invalid_negation("-\"hello\"")]
#[case::comparison_with_string("\"abc\" > \"def\"")]
#[case::division_by_zero("123 / 0")]
#[should_panic]
fn test_runtime_error(#[case] source: &str) {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    let _ = expression.accept(&mut interpreter).unwrap(); // should panic
}