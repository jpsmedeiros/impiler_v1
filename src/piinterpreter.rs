use std::io::BufRead;
use std::fmt;
use std;
use std::boxed::Box;

enum Statement{
    Exp,
}

#[derive(Debug)]
pub enum Exp{
    ArithExp,
    BoolExp,
}

#[derive(Debug)]
pub enum ArithExp{
    Sum{
        lhs: Box<ArithExp>,
        rhs: Box<ArithExp>
    },
    Sub {
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

#[derive(Debug)]
pub enum BoolExp{
    Eq {
        lhs: Box<BoolExp>,
        rhs: Box<BoolExp>
    },
    Neg { 
        value: Box<BoolExp>
    },
    And {
        lhs: Box<BoolExp>,
        rhs: Box<BoolExp>
    },
    Or {
        lhs: Box<BoolExp>,
        rhs: Box<BoolExp>
    },
    Bool {
        value: bool
    }
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


pub fn num(value: f64) -> Box<ArithExp>{
    Box::new(ArithExp::Num { value })
}

pub fn sum(lhs: Box<ArithExp>, rhs: Box<ArithExp>) -> Box<ArithExp>{
    Box::new(ArithExp::Sum { lhs, rhs })
}

pub fn sub(lhs: Box<ArithExp>, rhs: Box<ArithExp>) -> Box<ArithExp>{
    Box::new(ArithExp::Sub { lhs, rhs })
}

pub fn mul(lhs: Box<ArithExp>, rhs: Box<ArithExp>) -> Box<ArithExp>{
    Box::new(ArithExp::Mul { lhs, rhs })
}

pub fn div(lhs: Box<ArithExp>, rhs: Box<ArithExp>) -> Box<ArithExp>{
    Box::new(ArithExp::Div { lhs, rhs })
}

pub fn boolean(value: bool) -> Box<BoolExp>{
    Box::new(BoolExp::Bool { value })
}

pub fn eq(lhs: Box<BoolExp>, rhs: Box<BoolExp>) -> Box<BoolExp>{
    Box::new(BoolExp::Eq { lhs, rhs })
}

pub fn neg(value: Box<BoolExp>) -> Box<BoolExp>{
    Box::new(BoolExp::Neg { value })
}

pub fn and(lhs: Box<BoolExp>, rhs: Box<BoolExp>) -> Box<BoolExp>{
    Box::new(BoolExp::And { lhs, rhs })
}

pub fn or(lhs: Box<BoolExp>, rhs: Box<BoolExp>) -> Box<BoolExp>{
    Box::new(BoolExp::Or { lhs, rhs })
}

pub fn get_num_value(num: Box<ArithExp>) -> f64 {
    match *num {
        ArithExp::Num{value} => value,
        _ => unreachable!(),
    }
}

pub fn eval_tree(program: &ArithExp) {
    match program {
        ArithExp::Sum {lhs, rhs} => println!("sum"),
        ArithExp::Sub {lhs, rhs} => println!("sub"),
        ArithExp::Mul {lhs, rhs} => println!("mul"),
        ArithExp::Div {lhs, rhs} => println!("div"),
        ArithExp::Num {value} => println!("{}", value)
    }    
}
