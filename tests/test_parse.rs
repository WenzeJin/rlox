//! This file will test Scanner and Parser modules

use rlox::ast::pretty_printer;
use rlox::scanner::Scanner;
use rlox::parser::Parser;


#[test]
fn test_expr() {
    let source = "1 + 2 * 3";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();
    let mut printer = pretty_printer::AstPrinter();
    assert_eq!(expression.accept(&mut printer), "(+ 1 (* 2 3))");

    let source = "1 + 2 * 3 - 4 / 5";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();
    let mut printer = pretty_printer::AstPrinter();
    assert_eq!(expression.accept(&mut printer), "(- (+ 1 (* 2 3)) (/ 4 5))");

    let source = "-1 + 2";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();
    let mut printer = pretty_printer::AstPrinter();
    assert_eq!(expression.accept(&mut printer), "(+ (- 1) 2)");

    let source = "1 != 3 + 4";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();
    let mut printer = pretty_printer::AstPrinter();
    assert_eq!(expression.accept(&mut printer), "(!= 1 (+ 3 4))");

    let source = "1 + 2 * 3 == 4 / 5 == true == false != nil";
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();
    let mut printer = pretty_printer::AstPrinter();
    assert_eq!(expression.accept(&mut printer), "(!= (== (== (== (+ 1 (* 2 3)) (/ 4 5)) true) false) nil)");
}