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

mod parser;
use parser::*;

mod testy;
use testy::*;

fn main() {
    //parse();

    let mut aut = PiAut::new();
    let a = id(String::from("a"));
    let b = arithExp_as_exp(num(3.0));
    let x = assign(a,b);
    let y = statement_as_ctrl_stack_type(cmd_as_statement(x));
    aut.push_ctrl(y);

    aut = eval_automata(aut);
    println!("Control Stack:");
    aut.print_ctrl();
    println!("Value Stack:");
    aut.print_value();

    //testing();

}

#[cfg(test)]
mod tests;
