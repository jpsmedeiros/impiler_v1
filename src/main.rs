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

fn transform_bool(pair: Pair<Rule>) -> std::boxed::Box<piinterpreter::BoolExp> {
    let mut lhs: std::boxed::Box<piinterpreter::BoolExp> = boolean(false);
    let mut lhsblock: bool = false;
    let mut rhs: std::boxed::Box<piinterpreter::BoolExp> = boolean(false);

    let mut result: std::boxed::Box<piinterpreter::BoolExp>;

    let mut pairs = pair.clone().into_inner();
    let length = pair.into_inner().count();
    let mut p: Pair<Rule>;
    let mut x = 0;

    while x < length {
        p = pairs.next().unwrap();
        match p.as_rule(){
            Rule::bexp => {
                if !lhsblock {
                    lhs = transform_bool(p);
                }else{
                    rhs = transform_bool(p);
                }
                x = x + 1;
            },
            Rule::boolean => {
                if !lhsblock {
                    lhs = boolean(bool_value(p));
                    lhsblock = true;
                }else{
                    rhs = boolean(bool_value(p));
                }
                x = x + 1;
            },
            Rule::and => {
                let mut next_pair = pairs.next().unwrap();
                match next_pair.as_rule() { // se for and devemos pegar o próximo valor
                    Rule::neg => {
                        next_pair = pairs.next().unwrap();
                        rhs = neg(transform_bool_for_op(next_pair));
                        x = x + 1;
                    },
                    _ => {
                        rhs = transform_bool_for_op(next_pair);
                    },
                } 
                x = x + 2;
                lhs = and(lhs, rhs.clone());
                lhsblock = false;
            },
            Rule::neg => {
                let mut next_pair = pairs.next().unwrap(); // se for neg devemos pegar o próximo valor
                rhs = transform_bool_for_op(next_pair);
                lhs = neg(rhs);
                lhsblock = false;
                x = x + 2;
            }
            Rule::or => {
                let mut next_pair = pairs.next().unwrap();
                match next_pair.as_rule() { // se for or devemos pegar o próximo valor
                    Rule::neg => {
                        next_pair = pairs.next().unwrap();
                        rhs = neg(transform_bool_for_op(next_pair));
                        x = x + 1;
                    },
                    _ => {
                        rhs = transform_bool_for_op(next_pair);
                    },
                } 
                x = x + 2;
                lhs = or(lhs, rhs.clone());
                lhsblock = false;
            },
            Rule::equal => {
                let mut next_pair = pairs.next().unwrap();
                match next_pair.as_rule() { // se for or devemos pegar o próximo valor
                    Rule::neg => {
                        next_pair = pairs.next().unwrap();
                        rhs = neg(transform_bool_for_op(next_pair));
                        x = x + 1;
                    },
                    _ => {
                        rhs = transform_bool_for_op(next_pair);
                    },
                } 
                x = x + 2;
                lhs = eq(lhs, rhs.clone());
                lhsblock = false;
            },
            _ => unreachable!(),
        }
    }
    result = lhs;
    result
}

fn transform_bool_for_op(pair: Pair<Rule>) -> std::boxed::Box<piinterpreter::BoolExp> {
    match pair.as_rule() { // se for and devemos pegar o próximo valor
        Rule::bexp => transform_bool(pair), // é uma bexp, deve ser avaliado pelo transform_bool
        Rule::boolean => boolean(bool_value(pair)),
        _ => unreachable!(),
    } 
}

fn bool_value(pair: Pair<Rule>) -> bool {
    pair.as_str().parse::<bool>().unwrap()
}

fn transform(pair: Pair<Rule>) -> Box<Exp> {
    match pair.as_rule() {
       Rule::aexp => arithExp_as_exp(transform_arith(pair.into_inner())),
       Rule::bexp => boolExp_as_exp(transform_bool(pair)),
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
