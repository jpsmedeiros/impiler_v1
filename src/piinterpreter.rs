use std::io::BufRead;
use std::fmt;
use std;
use std::boxed::Box;
use std::collections::LinkedList;
use std::option::Option;

pub enum Statement{
    Exp,
}

#[derive(Debug)]
pub enum Exp{
    ArithExp(ArithExp),
    BoolExp(BoolExp),
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

pub struct PiAut{
    control_stack: LinkedList<Box<ArithExp>>,
    value_stack: LinkedList<Box<ArithExp>>,
}

impl PiAut{
    pub fn new() -> PiAut{
        PiAut{ control_stack: LinkedList::new(), value_stack: LinkedList::new() }
    }

    pub fn push_ctrl(&mut self,x: Box<ArithExp>){
        self.control_stack.push_front(x);
    }

    pub fn pop_ctrl(&mut self) -> Option<Box<ArithExp>>{
        self.control_stack.pop_front()
    }

    pub fn push_value(&mut self,x: Box<ArithExp>){
        self.value_stack.push_front(x);
    }

    pub fn pop_value(&mut self) -> Option<Box<ArithExp>>{
        self.value_stack.pop_front()
    }

    pub fn print_ctrl(&self){
        let i = self.control_stack.iter();
        for element in i{
            println!("{:?}",element);
        }
    }

    pub fn print_value(&self){
        let i = self.value_stack.iter();
        for element in i{
            println!("{:?}",element);
        }
    }
}

pub fn eval_automata(mut aut: PiAut) -> PiAut{

    while !aut.control_stack.is_empty(){
        let tree = aut.pop_ctrl();
        match *tree.unwrap(){
            ArithExp::Num{value} => aut.push_value(num(value)),
            _ => unreachable!(),
        }
    }
    aut
}





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
 
pub fn arithExp_as_exp(expression: Box<ArithExp>) -> Box<Exp> {
    Box::new(Exp::ArithExp(*expression))
}

pub fn boolExp_as_exp(expression: Box<BoolExp>) -> Box<Exp> {
    //let exp = &*Box::leak(expression);
    Box::new(Exp::BoolExp(*expression))
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
