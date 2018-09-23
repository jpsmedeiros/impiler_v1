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

fn main() {
    parse();
}

#[cfg(test)]
mod tests;
