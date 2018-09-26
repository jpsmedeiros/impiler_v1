use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use std::io::BufRead;
use std::fmt;
use std::f64;
use std;

use piinterpreter;

const _GRAMMAR: &str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Impiler;

lazy_static! { //declare lazy evaluated static
    static ref MATH_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use self::Assoc::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left)
        ])
    };
}

fn transform_arith(expression: Pairs<Rule>) -> Box<piinterpreter::ArithExp> {
    MATH_CLIMBER.climb(
       expression,
       |pair: Pair<Rule>| match pair.as_rule() {
           Rule::num => piinterpreter::num(pair.as_str().parse::<f64>().unwrap()),
           Rule::aexp => transform_arith(pair.into_inner()),
           Rule::id => piinterpreter::arith_id(str::replace(pair.as_str(), " ", "")),
           _ => unreachable!(),
       },
       |lhs, op: Pair<Rule>, rhs | match op.as_rule() {
           Rule::add      => piinterpreter::sum(lhs, rhs),
           Rule::subtract => piinterpreter::sub(lhs, rhs),
           Rule::multiply => piinterpreter::mul(lhs, rhs),
           Rule::divide   => piinterpreter::div(lhs, rhs),
           _ => unreachable!(),
       }
   )
}

fn transform_bool(pair: Pair<Rule>) -> Box<piinterpreter::BoolExp> {
    let mut lhs: Box<piinterpreter::Exp> = piinterpreter::boolExp_as_exp(piinterpreter::boolean(false));
    let mut rhs: Box<piinterpreter::Exp> = piinterpreter::boolExp_as_exp(piinterpreter::boolean(false));

    let mut result: Box<piinterpreter::BoolExp>;

    let mut pairs = pair.clone().into_inner();
    let length = pair.into_inner().count();
    let mut p: Pair<Rule>;
    let mut x = 0;
    while x < length {
        p = pairs.next().unwrap();
        match p.as_rule(){
            Rule::bexp => {
                lhs = piinterpreter::boolExp_as_exp(transform_bool(p));
                x = x + 1;
            },
            Rule::aexp => {
                lhs = piinterpreter::arithExp_as_exp(transform_arith(p.into_inner()));
                x = x + 1;
            },
            Rule::boolean => {
                lhs = piinterpreter::boolExp_as_exp(piinterpreter::boolean(bool_value(p)));
                x = x + 1;
            },
            Rule::id => {
                lhs = piinterpreter::boolExp_as_exp(piinterpreter::bool_id(str::replace(p.as_str(), " ", "")));
                x = x + 1;
            },
            Rule::and => {
                let mut next_pair = pairs.next().unwrap();
                match next_pair.as_rule() { // se for and devemos pegar o próximo valor
                    Rule::neg => {
                        next_pair = pairs.next().unwrap();
                        rhs = piinterpreter::boolExp_as_exp(piinterpreter::neg(transform_bool_for_op(next_pair)));
                        x = x + 1;
                    },
                    _ => {
                        rhs = piinterpreter::boolExp_as_exp(transform_bool_for_op(next_pair));
                    },
                }
                x = x + 2;
                lhs = piinterpreter::boolExp_as_exp(piinterpreter::and(piinterpreter::exp_as_boolExp(lhs), piinterpreter::exp_as_boolExp(rhs)));
            },
            Rule::neg => {
                let mut next_pair = pairs.next().unwrap(); // se for neg devemos pegar o próximo valor
                rhs = piinterpreter::boolExp_as_exp(transform_bool_for_op(next_pair));
                lhs = piinterpreter::boolExp_as_exp(piinterpreter::neg(piinterpreter::exp_as_boolExp(rhs)));
                x = x + 2;
            }
            Rule::or => {
                let mut next_pair = pairs.next().unwrap();
                match next_pair.as_rule() { // se for or devemos pegar o próximo valor
                    Rule::neg => {
                        next_pair = pairs.next().unwrap();
                        rhs = piinterpreter::boolExp_as_exp(piinterpreter::neg(transform_bool_for_op(next_pair)));
                        x = x + 1;
                    },
                    _ => {
                        rhs = piinterpreter::boolExp_as_exp(transform_bool_for_op(next_pair));
                    },
                }
                x = x + 2;
                lhs = piinterpreter::boolExp_as_exp(piinterpreter::or(piinterpreter::exp_as_boolExp(lhs), piinterpreter::exp_as_boolExp(rhs)));
            },
            Rule::equal => {
                let mut next_pair = pairs.next().unwrap();
                match next_pair.as_rule() { // se for or devemos pegar o próximo valor
                    Rule::neg => {
                        next_pair = pairs.next().unwrap();
                        rhs = piinterpreter::boolExp_as_exp(piinterpreter::neg(transform_bool_for_op(next_pair)));
                        x = x + 1;
                    },
                    _ => {
                        rhs = piinterpreter::boolExp_as_exp(transform_bool_for_op(next_pair));
                    },
                }
                x = x + 2;
                lhs = piinterpreter::boolExp_as_exp(piinterpreter::eq(piinterpreter::exp_as_boolExp(lhs), piinterpreter::exp_as_boolExp(rhs)));
            },
            Rule::greater_than => {
                let mut next_pair = pairs.next().unwrap();
                rhs = piinterpreter::arithExp_as_exp(transform_arith(next_pair.into_inner()));
                lhs = piinterpreter::boolExp_as_exp(piinterpreter::gt(piinterpreter::exp_as_arithExp(lhs), piinterpreter::exp_as_arithExp(rhs)));
                x = x + 2;
            },
            Rule::greater_equal => {
                let mut next_pair = pairs.next().unwrap();
                rhs = piinterpreter::arithExp_as_exp(transform_arith(next_pair.into_inner()));
                lhs = piinterpreter::boolExp_as_exp(piinterpreter::ge(piinterpreter::exp_as_arithExp(lhs), piinterpreter::exp_as_arithExp(rhs)));
                x = x + 2;
            },
            Rule::less_than => {
                let mut next_pair = pairs.next().unwrap();
                rhs = piinterpreter::arithExp_as_exp(transform_arith(next_pair.into_inner()));
                lhs = piinterpreter::boolExp_as_exp(piinterpreter::lt(piinterpreter::exp_as_arithExp(lhs), piinterpreter::exp_as_arithExp(rhs)));
                x = x + 2;
            },
            Rule::less_equal => {
                let mut next_pair = pairs.next().unwrap();
                rhs = piinterpreter::arithExp_as_exp(transform_arith(next_pair.into_inner()));
                lhs = piinterpreter::boolExp_as_exp(piinterpreter::le(piinterpreter::exp_as_arithExp(lhs), piinterpreter::exp_as_arithExp(rhs)));
                x = x + 2;
            },
            _ => unreachable!(),
        }
    }
    result = piinterpreter::exp_as_boolExp(lhs);
    result
}

fn transform_bool_for_op(pair: Pair<Rule>) -> Box<piinterpreter::BoolExp> {
    match pair.as_rule() { // se for and devemos pegar o próximo valor
        Rule::bexp => transform_bool(pair), // é uma bexp, deve ser avaliado pelo transform_bool
        Rule::boolean => piinterpreter::boolean(bool_value(pair)),
        Rule::id => piinterpreter::bool_id(str::replace(pair.as_str(), " ", "")),
        _ => unreachable!(),
    }
}

fn transform_cmd(pair: Pair<Rule>) -> Box<piinterpreter::Cmd> {
    let mut pairs = pair.clone().into_inner();
    let length = pair.into_inner().count();
    let mut p: Pair<Rule>;
    let mut x = 0;
    let mut result: Box<piinterpreter::Cmd> = piinterpreter::assign(piinterpreter::Id{value: "err".to_owned()}, piinterpreter::arithExp_as_exp(piinterpreter::num(0.0)));
    while x < length {
        p = pairs.next().unwrap();
        match p.as_rule() {
            Rule::cmd => {
                result = piinterpreter::cseq(result, transform_cmd(p));
                x = x+1;
            },
            Rule::assign_cmd => {
                result = transform_assign(p);
                x = x+1;
            },
            Rule::while_cmd => {
                result = transform_while(p);
                x = x+1;
            },
            _ => unreachable!()
        }
    }

    result
}

fn transform_assign(pair: Pair<Rule>) -> Box<piinterpreter::Cmd> {
    let mut pairs = pair.clone().into_inner();
    let id: piinterpreter::Id = piinterpreter::id(str::replace(pairs.next().unwrap().as_str(), " ", ""));
    let p: Pair<Rule> = pairs.next().unwrap();
    let exp: Box<piinterpreter::Exp> = match p.as_rule(){
        Rule::aexp => piinterpreter::arithExp_as_exp(transform_arith(p.into_inner())),
        Rule::bexp => piinterpreter::boolExp_as_exp(transform_bool(p)),
        _ => unreachable!()
    };
    piinterpreter::assign(id, exp)
}

fn transform_while(pair: Pair<Rule>) -> Box<piinterpreter::Cmd> {
    let mut pairs = pair.clone().into_inner();
    let boolean_exp: Box<piinterpreter::BoolExp> = transform_bool(pairs.next().unwrap());
    let cmd: Box<piinterpreter::Cmd> = transform_cmd(pairs.next().unwrap());
    piinterpreter::while_loop(boolean_exp, cmd)
}

fn bool_value(pair: Pair<Rule>) -> bool {
    pair.as_str().parse::<bool>().unwrap()
}

fn transform(pair: Pair<Rule>) -> Box<piinterpreter::Statement> {
    match pair.as_rule() {
       Rule::aexp => piinterpreter::arithExp_as_statement(transform_arith(pair.into_inner())),
       Rule::bexp => piinterpreter::boolExp_as_statement(transform_bool(pair)),
       Rule::cmd => piinterpreter::cmd_as_statement(transform_cmd(pair)),
       _ => unreachable!()
    }
}

fn print_input_message() {
    println!("\nDigite o comando desejado");
}

fn print_aut(result: Box<piinterpreter::Statement>){
    let mut aut: piinterpreter::PiAut = piinterpreter::PiAut::new();
    match *result {
        piinterpreter::Statement::Exp(exp) => aut.push_ctrl(piinterpreter::statement_as_ctrl_stack_type(piinterpreter::exp_as_statement(Box::new(exp)))),
        piinterpreter::Statement::Cmd(cmd)  => aut.push_ctrl(piinterpreter::statement_as_ctrl_stack_type(piinterpreter::cmd_as_statement(Box::new(cmd)))),
        _ => unreachable!()
    }
    aut = piinterpreter::eval_automata(aut);

    println!("Control Stack:");
    aut.print_ctrl();
    println!("Value Stack:");
    aut.print_value();
    println!("Memory:");
    aut.print_memory();
}

pub fn get_aut(result: Box<piinterpreter::Statement>) -> piinterpreter::PiAut {
    let mut aut: piinterpreter::PiAut = piinterpreter::PiAut::new();
    match *result {
        piinterpreter::Statement::Exp(exp) => aut.push_ctrl(piinterpreter::statement_as_ctrl_stack_type(piinterpreter::exp_as_statement(Box::new(exp)))),
        piinterpreter::Statement::Cmd(cmd)  => aut.push_ctrl(piinterpreter::statement_as_ctrl_stack_type(piinterpreter::cmd_as_statement(Box::new(cmd)))),
        _ => unreachable!()
    }

    aut = piinterpreter::eval_automata(aut);
    aut
}

pub fn parse(){
    print_input_message();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let pilib_result = parse_input(line);
        println!("PI-LIB = {:?}", pilib_result);
        print_aut(pilib_result);
        print_input_message();
    }

}

pub fn parse_input(expression: String) -> Box<piinterpreter::Statement> {
    let parse_result = Impiler::parse(Rule::impiler, &expression);
    match parse_result {
        Ok(mut pairs) => {
            let enclosed = pairs.next().unwrap();
            transform(enclosed)
        },
        Err(_) => {
            println!(" Syntax error");
            unreachable!()
        },
    }
}
