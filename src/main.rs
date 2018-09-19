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


enum Statement{
    Exp,
}

enum Exp{
    ArithExp,
    BoolExp,
}

#[derive(Debug)]
enum ArithExp{
    Sum{
        lhs: Box<ArithExp>,
        rhs: Box<ArithExp>
    },
    Num {
        value: f64
    },
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { head: None }
    }
}

//struct PiAut{
//    control_stack: Stack,
//    value_stack: Stack,
//    store: Stack,
//}

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

//fn eval(expression: Pairs<Rule>) -> ArithExp {
//    PREC_CLIMBER.climb(
//        expression,
//        |pair: Pair<Rule>| match pair.as_rule() {
//            Rule::num => ArithExp::Num {value: pair.as_str().parse::<f64>().unwrap() },
//            Rule::expr => eval(pair.into_inner()),
//            _ => unreachable!(),
//        },
//        |lhs, op: Pair<Rule>, rhs | match op.as_rule() {
//            Rule::add      => Sum {lhs: lhs, rhs: rhs},
//            //Rule::subtract => lhs - rhs,
//            //Rule::multiply => lhs * rhs,
//            //Rule::divide   => lhs / rhs,
//            _ => unreachable!(),
//        },
//    )
//}

fn print_input_message() {
    println!("\nDigite o cálculo desejado");
}

fn num(value: f64) -> std::boxed::Box<ArithExp>{
    Box::new(ArithExp::Num { value })
}

fn sum(lhs: std::boxed::Box<ArithExp>, rhs: std::boxed::Box<ArithExp>) -> std::boxed::Box<ArithExp>{
    Box::new(ArithExp::Sum { lhs, rhs })
}

fn main() {
    
    //println!("{} {}",x.lhs,x.rhs);
    let soma = sum(num(3.0), num(2.0));
    println!("SOMA = {:?}", soma);
    match soma {
        ArithExp => println!("ArithExp")
    }

    print_input_message();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parse_result = Calculator::parse(Rule::calculation, &line);

        match parse_result {
            Ok(calc) => println!("okay"),
            Err(_) => println!(" Syntax error"),
        }
        print_input_message();
    }
}
