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
    let result = parser::parse_input("5".to_owned());
    let expected = piinterpreter::arithExp_as_statement(num(5.0));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_arith_2() {
    let result = parser::parse_input("5+2".to_owned());
    let expected = piinterpreter::arithExp_as_statement(sum(num(5.0), num(2.0)));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_arith_3() {
    let result = parser::parse_input("5*(3+2)".to_owned());
    let expected = piinterpreter::arithExp_as_statement(mul(num(5.0), sum(num(3.0), num(2.0))));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_arith_4() {
    let result = parser::parse_input("5*(3+2)-(1+1)".to_owned());
    let expected = piinterpreter::arithExp_as_statement(sub(mul(num(5.0), sum(num(3.0), num(2.0))), sum(num(1.0), num(1.0))));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_arith_5() {
    let result = parser::parse_input("5 * x".to_owned());
    let expected = piinterpreter::arithExp_as_statement(mul(num(5.0), arith_id("x".to_owned())));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_boolop_1() {
    let result = parser::parse_input("true".to_owned());
    let expected = piinterpreter::boolExp_as_statement(boolean(true));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_boolop_2() {
    let result = parser::parse_input("true/\\false".to_owned());
    let expected = piinterpreter::boolExp_as_statement(and(boolean(true), boolean(false)));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_boolop_3() {
    let result = parser::parse_input("~true".to_owned());
    let expected = piinterpreter::boolExp_as_statement(neg(boolean(true)));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_boolop_4() {
    let result = parser::parse_input("~true\\/true".to_owned());
    let expected = piinterpreter::boolExp_as_statement(or(neg(boolean(true)), boolean(true)));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_boolop_5() {
    let result = parser::parse_input("~true\\/(true/\\false)".to_owned());
    let expected = piinterpreter::boolExp_as_statement(or(neg(boolean(true)), and(boolean(true), boolean(false))));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_iop_1() {
    let result = parser::parse_input("3 > 2".to_owned());
    let expected = piinterpreter::boolExp_as_statement(gt(num(3.0), num(2.0)));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_iop_2() {
    let result = parser::parse_input("3 < 2".to_owned());
    let expected = piinterpreter::boolExp_as_statement(lt(num(3.0), num(2.0)));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_iop_3() {
    let result = parser::parse_input("3 >= 2".to_owned());
    let expected = piinterpreter::boolExp_as_statement(ge(num(3.0), num(2.0)));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_iop_4() {
    let result = parser::parse_input("3 <= 2".to_owned());
    let expected = piinterpreter::boolExp_as_statement(le(num(3.0), num(2.0)));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_iop_5() {
    let result = parser::parse_input("(3*2) > 5+1".to_owned());
    let expected = piinterpreter::boolExp_as_statement(gt(mul(num(3.0), num(2.0)), sum(num(5.0), num(1.0))));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_cmd_1() {
    let result = parser::parse_input("x := 5".to_owned());
    let expected = piinterpreter::cmd_as_statement(assign(id("x".to_owned()), piinterpreter::arithExp_as_exp(num(5.0))));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_cmd_2() {
    let result = parser::parse_input("x := 5; y := 2".to_owned());
    let expected = piinterpreter::cmd_as_statement(cseq(assign(id("x".to_owned()), piinterpreter::arithExp_as_exp(num(5.0))), assign(id("y".to_owned()), piinterpreter::arithExp_as_exp(num(2.0)))));
    assert_eq!(result, expected);
}

#[test]
fn test_parser_cmd_3() {
    let result = parser::parse_input("while (true) do { x := 5 }".to_owned());
    let expected = piinterpreter::cmd_as_statement(while_loop(boolean(true), assign(id("x".to_owned()), piinterpreter::arithExp_as_exp(num(5.0)))));
    assert_eq!(result, expected);
}

#[test]
fn test_aut_arith_1() {
    let mut aut: piinterpreter::PiAut = piinterpreter::PiAut::new();
    let result = parser::get_aut(parser::parse_input("5 + 3".to_owned()));
    aut.value_stack.push_front(piinterpreter::arithExp_as_statement(num(8.0)));
    let expected = aut;
    assert_eq!(result, expected);
}

#[test]
fn test_aut_bool_1() {
    let mut aut: piinterpreter::PiAut = piinterpreter::PiAut::new();
    let result = parser::get_aut(parser::parse_input("true /\\ false".to_owned()));
    aut.value_stack.push_front(piinterpreter::boolExp_as_statement(boolean(false)));
    let expected = aut;
    assert_eq!(result, expected);
}

#[test]
fn test_aut_assign_1() {
    let mut aut: piinterpreter::PiAut = piinterpreter::PiAut::new();
    let result = parser::get_aut(parser::parse_input("x := 5".to_owned()));
    aut.store.insert("x".to_owned(), *piinterpreter::arithExp_as_exp(num(5.0)));
    let expected = aut;
    assert_eq!(result, expected);
}

#[test]
fn test_aut_assign_2() {
    let mut aut: piinterpreter::PiAut = piinterpreter::PiAut::new();
    let result = parser::get_aut(parser::parse_input("x := 5; y := x*2".to_owned()));
    aut.store.insert("x".to_owned(), *piinterpreter::arithExp_as_exp(num(5.0)));
    aut.store.insert("y".to_owned(), *piinterpreter::arithExp_as_exp(num(10.0)));
    let expected = aut;
    assert_eq!(result, expected);
}

#[test]
fn test_aut_assign_3() {
    let mut aut: piinterpreter::PiAut = piinterpreter::PiAut::new();
    let result = parser::get_aut(parser::parse_input("x := true".to_owned()));
    aut.store.insert("x".to_owned(), *piinterpreter::boolExp_as_exp(boolean(true)));
    let expected = aut;
    assert_eq!(result, expected);
}

#[test]
fn test_aut_assign_4() {
    let mut aut: piinterpreter::PiAut = piinterpreter::PiAut::new();
    let result = parser::get_aut(parser::parse_input("x := ~true".to_owned()));
    aut.store.insert("x".to_owned(), *piinterpreter::boolExp_as_exp(boolean(false)));
    let expected = aut;
    assert_eq!(result, expected);
}

#[test]
fn test_aut_while_1() {
    let mut aut: piinterpreter::PiAut = piinterpreter::PiAut::new();
    let result = parser::get_aut(parser::parse_input("x := 0; while(x < 2) do { x := x + 1 }".to_owned()));
    aut.store.insert("x".to_owned(), *piinterpreter::arithExp_as_exp(num(2.0)));
    let expected = aut;
    assert_eq!(result, expected);
}

#[test]
fn test_aut_while_2() {
    let mut aut: piinterpreter::PiAut = piinterpreter::PiAut::new();
    let result = parser::get_aut(parser::parse_input("factorial := 5; i := 1; value := 1; while(i <= factorial) do { value := value * i; i := i + 1 }".to_owned()));
    aut.store.insert("i".to_owned(), *piinterpreter::arithExp_as_exp(num(6.0)));
    aut.store.insert("factorial".to_owned(), *piinterpreter::arithExp_as_exp(num(5.0)));
    aut.store.insert("value".to_owned(), *piinterpreter::arithExp_as_exp(num(120.0)));
    let expected = aut;
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

pub fn gt(lhs: Box<piinterpreter::ArithExp>, rhs: Box<piinterpreter::ArithExp>) -> Box<piinterpreter::BoolExp>{
    piinterpreter::gt(lhs, rhs)
}

pub fn ge(lhs: Box<piinterpreter::ArithExp>, rhs: Box<piinterpreter::ArithExp>) -> Box<piinterpreter::BoolExp>{
    piinterpreter::ge(lhs, rhs)
}

pub fn lt(lhs: Box<piinterpreter::ArithExp>, rhs: Box<piinterpreter::ArithExp>) -> Box<piinterpreter::BoolExp>{
    piinterpreter::lt(lhs, rhs)
}

pub fn le(lhs: Box<piinterpreter::ArithExp>, rhs: Box<piinterpreter::ArithExp>) -> Box<piinterpreter::BoolExp>{
    piinterpreter::le(lhs, rhs)
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

pub fn id(value: String) -> piinterpreter::Id{
    piinterpreter::id(value)
}

pub fn arith_id(value: String) -> Box<piinterpreter::ArithExp> {
    piinterpreter::arith_id(value)
}

pub fn bool_id(value: String) -> Box<piinterpreter::BoolExp> {
    piinterpreter::bool_id(value)
}

pub fn assign(id: piinterpreter::Id, value: Box<piinterpreter::Exp>) -> Box<piinterpreter::Cmd> {
    piinterpreter::assign(id, value)
}

pub fn while_loop(bool_exp: Box<piinterpreter::BoolExp>, cmd: Box<piinterpreter::Cmd>) -> Box<piinterpreter::Cmd> {
    piinterpreter::while_loop(bool_exp, cmd)
}

pub fn cseq(command: Box<piinterpreter::Cmd>, next_command: Box<piinterpreter::Cmd>) -> Box<piinterpreter::Cmd> {
    piinterpreter::cseq(command, next_command)
}
