use piinterpreter;
use parser;

// Test model
// #[test]
// fn test_part_subpart_number() {
//     let result = program_result;
//     let expected = expected_value;
//     assert_eq!(result, expected);
// }

#[test]
fn test_parser_arith_1() {
    let result = parser::parse_expression("5".to_owned());
    let expected = piinterpreter::arithExp_as_exp(num(5.0));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_arith_2() {
    let result = parser::parse_expression("5+2".to_owned());
    let expected = piinterpreter::arithExp_as_exp(sum(num(5.0), num(2.0)));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_arith_3() {
    let result = parser::parse_expression("5*(3+2)".to_owned());
    let expected = piinterpreter::arithExp_as_exp(mul(num(5.0), sum(num(3.0), num(2.0))));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_arith_4() {
    let result = parser::parse_expression("5*(3+2)-(1+1)".to_owned());
    let expected = piinterpreter::arithExp_as_exp(sub(mul(num(5.0), sum(num(3.0), num(2.0))), sum(num(1.0), num(1.0))));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_bool_1() {
    let result = parser::parse_expression("true".to_owned());
    let expected = piinterpreter::boolExp_as_exp(boolean(true));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_bool_2() {
    let result = parser::parse_expression("true/\\false".to_owned());
    let expected = piinterpreter::boolExp_as_exp(and(boolean(true), boolean(false)));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_bool_3() {
    let result = parser::parse_expression("~true".to_owned());
    let expected = piinterpreter::boolExp_as_exp(neg(boolean(true)));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_bool_4() {
    let result = parser::parse_expression("~true\\/true".to_owned());
    let expected = piinterpreter::boolExp_as_exp(or(neg(boolean(true)), boolean(true)));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_bool_5() {
    let result = parser::parse_expression("~true\\/(true/\\false)".to_owned());
    let expected = piinterpreter::boolExp_as_exp(or(neg(boolean(true)), and(boolean(true), boolean(false))));
    assert_eq!(result, expected);
}


pub fn boolean(value: bool) -> Box<piinterpreter::BoolExp>{
    piinterpreter::boolean(value)
}

pub fn and(lhs: Box<piinterpreter::BoolExp>, rhs: Box<piinterpreter::BoolExp>) -> Box<piinterpreter::BoolExp>{
    piinterpreter::and(lhs, rhs)
}

pub fn or(lhs: Box<piinterpreter::BoolExp>, rhs: Box<piinterpreter::BoolExp>) -> Box<piinterpreter::BoolExp>{
    piinterpreter::or(lhs, rhs)
}

pub fn eq(lhs: Box<piinterpreter::BoolExp>, rhs: Box<piinterpreter::BoolExp>) -> Box<piinterpreter::BoolExp>{
    piinterpreter::eq(lhs, rhs)
}

pub fn neg(rhs: Box<piinterpreter::BoolExp>) -> Box<piinterpreter::BoolExp>{
    piinterpreter::neg(rhs)
}

pub fn num(value: f64) -> Box<piinterpreter::ArithExp> {
    piinterpreter::num(value)
}

pub fn sum(lhs: Box<piinterpreter::ArithExp>, rhs: Box<piinterpreter::ArithExp>) -> Box<piinterpreter::ArithExp> {
    piinterpreter::sum(lhs, rhs)
}

pub fn sub(lhs: Box<piinterpreter::ArithExp>, rhs: Box<piinterpreter::ArithExp>) -> Box<piinterpreter::ArithExp> {
    piinterpreter::sub(lhs, rhs)
}

pub fn mul(lhs: Box<piinterpreter::ArithExp>, rhs: Box<piinterpreter::ArithExp>) -> Box<piinterpreter::ArithExp> {
    piinterpreter::mul(lhs, rhs)
}

pub fn div(lhs: Box<piinterpreter::ArithExp>, rhs: Box<piinterpreter::ArithExp>) -> Box<piinterpreter::ArithExp> {
    piinterpreter::div(lhs, rhs)
}