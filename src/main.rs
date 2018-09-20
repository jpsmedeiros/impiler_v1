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
use std::collections::LinkedList;

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
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right)
        ])
    };
}

fn eval(expression: Pairs<Rule>) -> std::boxed::Box<piinterpreter::ArithExp> {
   PREC_CLIMBER.climb(
       expression,
       |pair: Pair<Rule>| match pair.as_rule() {
           Rule::num => num(pair.as_str().parse::<f64>().unwrap()),
           Rule::expr => eval(pair.into_inner()),
           _ => unreachable!(),
       },
       |lhs, op: Pair<Rule>, rhs | match op.as_rule() {
           Rule::add      => sum(lhs, rhs),
           Rule::subtract => sub(lhs, rhs),
           Rule::multiply => mul(lhs, rhs),
           Rule::divide   => div(lhs, rhs),
           _ => unreachable!(),
       },
   )
}

fn print_input_message() {
    println!("\nDigite o cálculo desejado");
}

fn main() {

    /*
    let mut aut: piinterpreter::PiAut = piinterpreter::PiAut::new();
    aut.push_ctrl(num(5.0));
    */
    
    /*
    let mut list: LinkedList<Box<ArithExp>> = LinkedList::new();
    list.push_front(num(5.0));
    list.push_front(num(2.0));

    {
        let list_iter = list.iter();
        for element in list_iter{
            println!("{:?}",element);
        }
    }

    list.push_front(sum(num(3.0),num(2.0)));
    list.pop_back();
    for element in list{
        println!("{:?}",element);
    }
    */



    print_input_message();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parse_result = Calculator::parse(Rule::calculation, &line);
        match parse_result {
            Ok(calc) => { println!("= {:?}", eval(calc)); println!("aaa mlk") },
            Err(_) => println!(" Syntax error"),
        }
        print_input_message();
    }
}
