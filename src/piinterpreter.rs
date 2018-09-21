use std::io::BufRead;
use std::fmt;
use std;
use std::boxed::Box;
use std::collections::LinkedList;

enum Statement{
    Exp,
}

enum Exp{
    ArithExp,
    BoolExp,
}

#[derive(Debug)]
pub enum ArithExp{
    Sum{
        lhs: Box<ArithExp>,
        rhs: Box<ArithExp>
    },
    Mul {
        lhs: Box<ArithExp>,
        rhs: Box<ArithExp>
    },
    Div {
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

enum Commands{
    Sum,
    Mul,
    Div
}

struct PiAut{
    control_stack: LinkedList<Commands>,
    value_stack: LinkedList<f64>
}


pub fn num(value: f64) -> Box<ArithExp>{
    Box::new(ArithExp::Num { value })
}

pub fn sum(lhs: Box<ArithExp>, rhs: Box<ArithExp>) -> Box<ArithExp>{
    Box::new(ArithExp::Sum { lhs, rhs })
}

pub fn mul(lhs: Box<ArithExp>, rhs: Box<ArithExp>) -> Box<ArithExp>{
    Box::new(ArithExp::Mul { lhs, rhs })
}

pub fn div(lhs: Box<ArithExp>, rhs: Box<ArithExp>) -> Box<ArithExp>{
    Box::new(ArithExp::Div { lhs, rhs })
}

pub fn get_num_value(num: Box<ArithExp>) -> f64 {
    match *num {
        ArithExp::Num{value} => value,
        _ => unreachable!(),
    }
}

fn add_comand(c:Commands, aut: &mut PiAut){
    aut.control_stack.push_front(c);
}

fn add_value(v:f64, aut: &mut PiAut){
    aut.value_stack.push_front(v);
}

fn new_PiAutomata() -> PiAut {
    PiAut {control_stack: LinkedList::new(), value_stack: LinkedList::new()}
}

pub fn eval_tree(program: ArithExp) {
    let mut aut = new_PiAutomata();
    match program {
        ArithExp::Sum {lhs, rhs} => {add_comand(Commands::Sum, &mut aut);eval_tree(*lhs);eval_tree(*rhs);},
        ArithExp::Num {value} => add_value(value, &mut aut),
        ArithExp::Mul {lhs, rhs} => {add_comand(Commands::Sum, &mut aut);eval_tree(*lhs);eval_tree(*rhs);},
        ArithExp::Div {lhs, rhs} => {add_comand(Commands::Sum, &mut aut);eval_tree(*lhs);eval_tree(*rhs);}
    }

    for val in aut.control_stack.iter_mut() {
        match val {
            Sum => pop_value(&mut aut)+pop_value(&mut aut),
            Mul => println!("2"),
            Div => println!("3")
        }
    }
        
}
