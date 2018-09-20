#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use std::io::BufRead;
use std::fmt;
use std::f64;

const _GRAMMAR: &str = include_str!("grammar.pest");

mod piinterpreter;

use piinterpreter::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Calculator;


lazy_static! { //declare lazy evaluated static
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left)
        ])
    };
}

fn eval(expression: Pairs<Rule>) -> std::boxed::Box<piinterpreter::Exp> {
    println!("Expression = {}", expression);
   PREC_CLIMBER.climb(
       expression,
       |pair: Pair<Rule>| match pair.as_rule() {
           Rule::num => num(pair.as_str().parse::<f64>().unwrap()),
           Rule::boolean => boolean(pair.as_str().parse::<bool>().unwrap()),
           Rule::aexp => eval(pair.into_inner()),
           Rule::bexp => eval(pair.into_inner()),
           _ => unreachable!(),
       },
       |lhs, op: Pair<Rule>, rhs | match op.as_rule() {
           Rule::add      => sum(lhs, rhs),
           Rule::subtract => sub(lhs, rhs),
           Rule::multiply => mul(lhs, rhs),
           Rule::divide   => div(lhs, rhs),
           _ => unreachable!(),
       }
   )
}

fn print_input_message() {
    println!("\nDigite o cálculo desejado");
}

fn main() {

    print_input_message();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parse_result = Calculator::parse(Rule::calculation, &line);
        match parse_result {
            Ok(calc) => println!("= {:?}", eval(calc)),
            Err(_) => println!(" Syntax error"),
        }
        print_input_message();
    }
}
