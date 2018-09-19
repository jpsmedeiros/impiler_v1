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

const _GRAMMAR: &str = include_str!("grammar.pest");

mod piinterpreter;


#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Calculator;


lazy_static! { //declare lazy evaluated static
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right)
        ])
    };
}

fn eval(expression: Pairs<Rule>) -> &str {
   PREC_CLIMBER.climb(
       expression,
       |pair: Pair<Rule>| match pair.as_rule() {
           //Rule::num => num(pair.as_str().parse::<f64>().unwrap()),
           //Rule::expr => eval(pair.into_inner()),
           _ => unreachable!(),
       },
       |lhs, op: Pair<Rule>, rhs | match op.as_rule() {
           //Rule::add      => sum(lhs, rhs),
           //Rule::subtract => sum(lhs, rhs),
           //Rule::multiply => mul(lhs, rhs),
           //Rule::divide   => lhs / rhs,
           _ => unreachable!(),
       },
   )
}

fn print_input_message() {
    println!("\nDigite o cálculo desejado");
}

fn main() {

    // Exemplos
    // let soma = sum(num(3.0), num(2.0));
    // println!("SOMA = {:?}", soma);
    // match soma {
    //     ArithExp => println!("ArithExp")
    // }

    print_input_message();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parse_result = Calculator::parse(Rule::calculation, &line);
        //let result;
        //match parse_result {
        //    Ok(calc) => result = eval(calc),
        //    Err(_) => println!(" Syntax error"),
        //}
        let result = Box::leak(piinterpreter::sum(piinterpreter::num(3.0), piinterpreter::num(2.0)));
        piinterpreter::eval(result);
        print_input_message();
    }
}
