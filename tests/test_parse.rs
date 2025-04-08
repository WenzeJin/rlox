//! This file will test Scanner and Parser modules

use rlox::ast::pretty_printer;
use rlox::scanner::Scanner;
use rlox::parser::Parser;


#[test]
fn test_expr() {
    let mut printer = pretty_printer::AstPrinter();

    let source = "1 + 2 * 3";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse().unwrap();
    assert_eq!(expression.accept(&mut printer), "(+ 1 (* 2 3))");

    let source = "1 + 2 * 3 - 4 / 5";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse().unwrap();
    assert_eq!(expression.accept(&mut printer), "(- (+ 1 (* 2 3)) (/ 4 5))");

    let source = "-1 + 2";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse().unwrap();
    assert_eq!(expression.accept(&mut printer), "(+ (- 1) 2)");

    let source = "1 != 3 + 4";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse().unwrap();
    assert_eq!(expression.accept(&mut printer), "(!= 1 (+ 3 4))");

    // a very complex expression
    let source = "-1 + 2 * 3 == 4 / 5 == true == false != nil";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse().unwrap();
    assert_eq!(expression.accept(&mut printer), "(!= (== (== (== (+ (- 1) (* 2 3)) (/ 4 5)) true) false) nil)");

    // a simple string
    let source = "\"hello\" + \"world\"";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse().unwrap();
    assert_eq!(expression.accept(&mut printer), "(+ hello world)");

    // a string with escape characters
    let source = "\"hello \\\"world\\\"\" + \"foo\"";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse().unwrap();
    assert_eq!(expression.accept(&mut printer), "(+ hello \"world\" foo)");

    // error cases
    let source = "1 + 2 *";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();
    assert!(expression.is_none());
}