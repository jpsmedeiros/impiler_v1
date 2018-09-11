#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use std::io::BufRead;

const _GRAMMAR: &str = include_str!("grammar.pest");

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

fn eval(expression: Pairs<Rule>) -> String {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.to_string(),
            Rule::expr => eval(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: String, op: Pair<Rule>, rhs: String | match op.as_rule() {
            Rule::add      => (lhs.parse::<f64>().unwrap() + rhs.parse::<f64>().unwrap()).to_string(),
            Rule::subtract => (lhs.parse::<f64>().unwrap() - rhs.parse::<f64>().unwrap()).to_string(),
            Rule::multiply => (lhs.parse::<f64>().unwrap() * rhs.parse::<f64>().unwrap()).to_string(),
            Rule::divide   => (lhs.parse::<f64>().unwrap() / rhs.parse::<f64>().unwrap()).to_string(),
            Rule::power    => (lhs.parse::<f64>().unwrap().powf(rhs.parse::<f64>().unwrap())).to_string(),
            _ => unreachable!(),
        },
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
        let parse_result = Calculator::parse(Rule::calculation, &line).unwrap();
        let tokens: Vec<_> = parse_result.flatten().tokens().collect();
        let mut c = 0;
        for pair in tokens.into_iter()
                            .map(|letter| { c += 1; (letter, c) }) {
                                println!("{:?}", pair.0);
                            }

        let parse_result = Calculator::parse(Rule::calculation, &line);
        match parse_result {
            Ok(calc) => println!(" = {}", eval(calc)),
            Err(_) => println!(" Syntax error"),
        }

        print_input_message();
    }
}