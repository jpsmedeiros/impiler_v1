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

#[derive(Debug)]
pub enum KW{
    KWSum,
    KWSub,
    KWMul,
    KWDiv,
}

#[derive(Debug)]
pub enum Ctrl_stack_type{
    Exp(Exp),
    KW(KW),
}

pub struct PiAut{
    control_stack: LinkedList<Box<Ctrl_stack_type>>,
    value_stack: LinkedList<Box<ArithExp>>,
}

impl PiAut{
    pub fn new() -> PiAut{
        PiAut{ control_stack: LinkedList::new(), value_stack: LinkedList::new() }
    }

    pub fn push_ctrl(&mut self,x: Box<Ctrl_stack_type>){
        self.control_stack.push_front(x);
    }

    pub fn pop_ctrl(&mut self) -> Option<Box<Ctrl_stack_type>>{
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

    pub fn sum_rule(&mut self, lhs:Box<ArithExp>, rhs:Box<ArithExp>){
        let x = Box::new(KW::KWSum);
        self.push_ctrl(kw_as_ctrl_stack_type(x));

        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(rhs)));
        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(lhs)));
    }

    pub fn sub_rule(&mut self, lhs:Box<ArithExp>, rhs:Box<ArithExp>){
        let x = Box::new(KW::KWSub);
        self.push_ctrl(kw_as_ctrl_stack_type(x));

        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(lhs)));
        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(rhs)));
    }

    pub fn mul_rule(&mut self, lhs:Box<ArithExp>, rhs:Box<ArithExp>){
        let x = Box::new(KW::KWMul);
        self.push_ctrl(kw_as_ctrl_stack_type(x));

        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(lhs)));
        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(rhs)));
    }

    pub fn div_rule(&mut self, lhs:Box<ArithExp>, rhs:Box<ArithExp>){
        let x = Box::new(KW::KWDiv);
        self.push_ctrl(kw_as_ctrl_stack_type(x));

        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(lhs)));
        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(rhs)));
    }

    pub fn sum_kw_rule(&mut self){
        let n1 = get_num_value(self.pop_value().unwrap());
        let n2 = get_num_value(self.pop_value().unwrap());
        let result = n1 + n2;

        self.push_value(num(result));
    }

    pub fn sub_kw_rule(&mut self){
        let n1 = get_num_value(self.pop_value().unwrap());
        let n2 = get_num_value(self.pop_value().unwrap());
        let result = n1 - n2;

        self.push_value(num(result));
    }

    pub fn mul_kw_rule(&mut self){
        let n1 = get_num_value(self.pop_value().unwrap());
        let n2 = get_num_value(self.pop_value().unwrap());
        let result = n1 * n2;

        self.push_value(num(result));
    }

    pub fn div_kw_rule(&mut self){
        let n1 = get_num_value(self.pop_value().unwrap());
        let n2 = get_num_value(self.pop_value().unwrap());
        let result = n1 / n2;

        self.push_value(num(result));
    }
}

pub fn eval_aexp_aut(aexp: ArithExp, mut aut: PiAut) -> PiAut{
    match aexp{
        ArithExp::Num{value} => aut.push_value(num(value)),
        ArithExp::Sum{lhs,rhs} => aut.sum_rule(lhs,rhs),
        ArithExp::Sub{lhs,rhs} => aut.sub_rule(lhs,rhs),
        ArithExp::Mul{lhs,rhs} => aut.mul_rule(lhs,rhs),
        ArithExp::Div{lhs,rhs} => aut.div_rule(lhs,rhs),
        _ => unreachable!(),
    }
    aut
}

pub fn eval_exp_aut(expression: Exp,mut aut: PiAut) -> PiAut{
    match expression{
        Exp::ArithExp(aexp) => aut = eval_aexp_aut(aexp,aut),
        _ => unreachable!(),
    }
    aut
}

pub fn eval_kw_aut(keyword: KW,mut aut: PiAut) -> PiAut{
    match keyword{
        KW::KWSum => aut.sum_kw_rule(),
        KW::KWSub => aut.sub_kw_rule(),
        KW::KWMul => aut.mul_kw_rule(),
        KW::KWDiv => aut.div_kw_rule(),
        _ => unreachable!(),
    }
    aut
}

pub fn eval_automata(mut aut: PiAut) -> PiAut{

    while !aut.control_stack.is_empty(){
        let tree = aut.pop_ctrl();
        match *tree.unwrap(){
            Ctrl_stack_type::Exp(exp) => aut = eval_exp_aut(exp,aut),
            Ctrl_stack_type::KW(kw) => aut = eval_kw_aut(kw,aut),

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

pub fn exp_as_ctrl_stack_type(expression: Box<Exp>) -> Box<Ctrl_stack_type>{
    Box::new(Ctrl_stack_type::Exp(*expression))
}

pub fn kw_as_ctrl_stack_type(keyword: Box<KW>) -> Box<Ctrl_stack_type>{
    Box::new(Ctrl_stack_type::KW(*keyword))
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
