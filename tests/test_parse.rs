//! This file will test Scanner and Parser modules

use rlox::ast::pretty_printer;
use rlox::scanner::Scanner;
use rlox::parser::Parser;
use rstest::rstest;


#[rstest]
#[case("1 + 2 * 3;", "[(+ 1 (* 2 3))]")]
#[case("1 + 2 * 3 - 4 / 5;", "[(- (+ 1 (* 2 3)) (/ 4 5))]")]
#[case("-1 + 2;", "[(+ (- 1) 2)]")]
#[case("1 != 3 + 4;", "[(!= 1 (+ 3 4))]")]
#[case("\"hello\" + \"world\";", "[(+ hello world)]")]
#[case("-1 + 2 * 3 == 4 / 5 == true == false != nil;", "[(!= (== (== (== (+ (- 1) (* 2 3)) (/ 4 5)) true) false) nil)]")]
#[case("var a; a = 1 + 2 + b * (c = d);", "[(var a);(= a (+ (+ 1 2) (* b (group (= c d)))))]")]
#[case::assignment_right_associative("a = b = c = d;", "[(= a (= b (= c d)))]")]
#[case("true and false;", "[(and true false)]")]
#[case("true or false;", "[(or true false)]")]
#[case("true and false or true;", "[(or (and true false) true)]")]
#[case("a or b and c;", "[(or a (and b c))]")]
#[case("a and b or c and d;", "[(or (and a b) (and c d))]")]
#[case("a and (b or c);", "[(and a (group (or b c)))]")]
#[case("(a or b) and c;", "[(and (group (or a b)) c)]")]
#[case("a or b and c or d;", "[(or (or a (and b c)) d)]")]
#[case("a and !b;", "[(and a (! b))]")]
fn test_expr_stmt(#[case] source: &str, #[case] expected: &str) {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let statement = parser.parse().unwrap(); // 表达式语句视为 statement
    let mut printer = pretty_printer::AstPrinter();
    assert_eq!(statement.accept(&mut printer), expected);
}


#[rstest()]
#[case("1 + 2;", "[(+ 1 2)]")]
#[case("var a = 1 + 2;", "[(var a = (+ 1 2))]")]
#[case("print 1 + 2;", "[(print (+ 1 2))]")]
#[case("var a; print a;", "[(var a);(print a)]")]
fn test_simple_stmt(#[case] source: &str, #[case] expected: &str) {
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let statement = parser.parse().unwrap();
    let mut printer = pretty_printer::AstPrinter();
    assert_eq!(statement.accept(&mut printer), expected);
}