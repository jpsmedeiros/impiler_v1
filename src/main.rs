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
struct Impiler;


lazy_static! { //declare lazy evaluated static
    static ref MATH_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left)
        ])
    };
    static ref BOOL_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(equal, Left),
            Operator::new(and, Left) | Operator::new(or, Left),
            Operator::new(neg, Right)
        ])
    };
}

fn transform_arith(expression: Pairs<Rule>) -> std::boxed::Box<piinterpreter::ArithExp> {
    MATH_CLIMBER.climb(
       expression,
       |pair: Pair<Rule>| match pair.as_rule() {
           Rule::num => num(pair.as_str().parse::<f64>().unwrap()),
           Rule::aexp => transform_arith(pair.into_inner()),
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

fn transform_bool(expression: Pairs<Rule>) -> std::boxed::Box<piinterpreter::BoolExp> {
    println!("Bool Expression = {}", expression);
    BOOL_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule(){
            Rule::boolean => boolean(pair.as_str().parse::<bool>().unwrap()),
            Rule::bexp => transform_bool(pair.into_inner()),
            //Rule::neg => neg(boolean(pair.into_inner().as_str().parse::<bool>().unwrap())),
            _ => unreachable!(),
        },
        |lhs, op: Pair<Rule>, rhs | match op.as_rule(){
            Rule::equal => eq(lhs, rhs),
            Rule::and   => and(lhs, rhs),
            Rule::or    => or(lhs, rhs),
            _ => unreachable!(),
        }
    )
}

fn transform(pair: Pair<Rule>) -> Box<Exp> {
    match pair.as_rule() {
       Rule::aexp => arithExp_as_exp(transform_arith(pair.into_inner())),
       Rule::bexp => boolExp_as_exp(transform_bool(pair.into_inner())),
       _ => unreachable!()
    }
}

fn print_input_message() {
    println!("\nDigite o comando desejado");
}

fn print_aut(result: Box<Exp>){
    let mut aut: piinterpreter::PiAut = piinterpreter::PiAut::new();
    match *result {
        Exp::ArithExp(arithExp) => aut.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(Box::new(arithExp)))),
        Exp::BoolExp(boolExp)  => aut.push_ctrl(exp_as_ctrl_stack_type(boolExp_as_exp(Box::new(boolExp)))),
        _ => unreachable!()
    }
    aut = eval_automata(aut);

    println!("Control Stack:");
    aut.print_ctrl();
    println!("Value Stack:");
    aut.print_value();
}

fn main() {

    print_input_message();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parse_result = Impiler::parse(Rule::impiler, &line);
        match parse_result {
            Ok(mut pairs) => {
                let enclosed = pairs.next().unwrap();
                let pilib_result = transform(enclosed);
                println!("Result = {:?}", pilib_result);
                print_aut(pilib_result);
            },
            Err(_) => println!(" Syntax error"),
        }
        print_input_message();
    }

}
