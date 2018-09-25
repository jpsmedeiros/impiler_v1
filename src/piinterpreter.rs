use std::io::BufRead;
use std::fmt;
use std;
use std::boxed::Box;
use std::collections::LinkedList;
use std::option::Option;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Statement{
    Cmd(Cmd),
    Exp(Exp),
}

#[derive(Debug, PartialEq)]
pub enum Exp{
    ArithExp(ArithExp),
    BoolExp(BoolExp),
}

#[derive(Debug, Clone, PartialEq)]
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
    Id {
        value: String
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    },
    Gt {
        lhs: Box<ArithExp>,
        rhs: Box<ArithExp>
    },
    Ge {
        lhs: Box<ArithExp>,
        rhs: Box<ArithExp>
    },
    Lt {
        lhs: Box<ArithExp>,
        rhs: Box<ArithExp>
    },
    Le {
        lhs: Box<ArithExp>,
        rhs: Box<ArithExp>
    },
    Id {
        value: String
    }
}

#[derive(Debug, PartialEq)]
pub enum Cmd{
    Assign {
        id: Id,
        value: Box<Exp>
    },
    While {
        boolExp: Box<BoolExp>,
        cmd: Box<Cmd>
    },
    CSeq {
        command: Box<Cmd>,
        next_command: Box<Cmd>
    },
}

#[derive(Debug, PartialEq)]
pub struct Id {
    pub value: String
}

#[derive(Debug)]
pub enum KW{
    KWSum,
    KWSub,
    KWMul,
    KWDiv,
    KWAnd,
    KWOr,
    KWEq,
    KWNeg,
    KWGt,
    KWGe,
    KWLt,
    KWLe,
    KWAss,
    KWCseq,
}

#[derive(Debug)]
pub enum Ctrl_stack_type{
    Statement(Statement),
    KW(KW),
}

pub struct PiAut{
    control_stack: LinkedList<Box<Ctrl_stack_type>>,
    value_stack: LinkedList<Box<Exp>>,
    store: HashMap<String, Box<Exp>>,
}

impl PiAut{
    pub fn new() -> PiAut{
        PiAut{ control_stack: LinkedList::new(), value_stack: LinkedList::new(), store: HashMap::new() }
    }

    pub fn push_ctrl(&mut self,x: Box<Ctrl_stack_type>){
        self.control_stack.push_front(x);
    }

    pub fn pop_ctrl(&mut self) -> Option<Box<Ctrl_stack_type>>{
        self.control_stack.pop_front()
    }

    pub fn push_value(&mut self,x: Box<Exp>){
        self.value_stack.push_front(x);
    }

    pub fn pop_value(&mut self) -> Option<Box<Exp>>{
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

    pub fn print_memory(&self){
        let i = self.store.iter();
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

    pub fn and_rule(&mut self, lhs:Box<BoolExp>, rhs:Box<BoolExp>){
        let x = Box::new(KW::KWAnd);
        self.push_ctrl(kw_as_ctrl_stack_type(x));

        self.push_ctrl(exp_as_ctrl_stack_type(boolExp_as_exp(lhs)));
        self.push_ctrl(exp_as_ctrl_stack_type(boolExp_as_exp(rhs)));
    }

    pub fn or_rule(&mut self, lhs:Box<BoolExp>, rhs:Box<BoolExp>){
        let x = Box::new(KW::KWOr);
        self.push_ctrl(kw_as_ctrl_stack_type(x));

        self.push_ctrl(exp_as_ctrl_stack_type(boolExp_as_exp(lhs)));
        self.push_ctrl(exp_as_ctrl_stack_type(boolExp_as_exp(rhs)));
    }

    pub fn eq_rule(&mut self, lhs:Box<BoolExp>, rhs:Box<BoolExp>){
        let x = Box::new(KW::KWEq);
        self.push_ctrl(kw_as_ctrl_stack_type(x));

        self.push_ctrl(exp_as_ctrl_stack_type(boolExp_as_exp(lhs)));
        self.push_ctrl(exp_as_ctrl_stack_type(boolExp_as_exp(rhs)));
    }

    pub fn neg_rule(&mut self, value:Box<BoolExp>){
        let x = Box::new(KW::KWNeg);
        self.push_ctrl(kw_as_ctrl_stack_type(x));

        self.push_ctrl(exp_as_ctrl_stack_type(boolExp_as_exp(value)));
    }

    pub fn gt_rule(&mut self, lhs:Box<ArithExp>, rhs:Box<ArithExp>){
        let x = Box::new(KW::KWGt);
        self.push_ctrl(kw_as_ctrl_stack_type(x));

        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(lhs)));
        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(rhs)));
    }

    pub fn ge_rule(&mut self, lhs:Box<ArithExp>, rhs:Box<ArithExp>){
        let x = Box::new(KW::KWGe);
        self.push_ctrl(kw_as_ctrl_stack_type(x));

        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(lhs)));
        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(rhs)));
    }

    pub fn lt_rule(&mut self, lhs:Box<ArithExp>, rhs:Box<ArithExp>){
        let x = Box::new(KW::KWLt);
        self.push_ctrl(kw_as_ctrl_stack_type(x));

        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(lhs)));
        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(rhs)));
    }

    pub fn le_rule(&mut self, lhs:Box<ArithExp>, rhs:Box<ArithExp>){
        let x = Box::new(KW::KWLe);
        self.push_ctrl(kw_as_ctrl_stack_type(x));

        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(lhs)));
        self.push_ctrl(exp_as_ctrl_stack_type(arithExp_as_exp(rhs)));
    }

    pub fn assign_rule(&mut self, lhs: Id, rhs: Box<Exp>){
        let x = Box::new(KW::KWAss);
        self.push_ctrl(kw_as_ctrl_stack_type(x));
        /*
        {
            let y = rhs;
            match *y{
                Exp::ArithExp(_aexp) => { self.push_value(arithExp_as_exp(id_as_aexp(lhs))); }
                Exp::BoolExp(_bexp) => { self.push_value(boolExp_as_exp(id_as_bexp(lhs))); }
                _ => unreachable!(),
            }
        }
        */
        self.push_value(arithExp_as_exp(id_as_aexp(lhs)));

        self.push_ctrl(exp_as_ctrl_stack_type(rhs));


        //self.push_value(id_as_exp(lhs));
    }

    pub fn cseq_rule(&mut self, lhs: Box<Cmd>, rhs: Box<Cmd>){
        // let x = Box::new(KW::KWCseq);
        // self.push_ctrl(kw_as_ctrl_stack_type(x));
        // TODO
    }

    pub fn sum_kw_rule(&mut self){
        let n1 = get_num_value(self.pop_value().unwrap());
        let n2 = get_num_value(self.pop_value().unwrap());
        let result = n1 + n2;

        self.push_value(arithExp_as_exp(num(result)));
    }

    pub fn sub_kw_rule(&mut self){
        let n1 = get_num_value(self.pop_value().unwrap());
        let n2 = get_num_value(self.pop_value().unwrap());
        let result = n1 - n2;

        self.push_value(arithExp_as_exp(num(result)));
    }

    pub fn mul_kw_rule(&mut self){
        let n1 = get_num_value(self.pop_value().unwrap());
        let n2 = get_num_value(self.pop_value().unwrap());
        let result = n1 * n2;

        self.push_value(arithExp_as_exp(num(result)));
    }

    pub fn div_kw_rule(&mut self){
        let n1 = get_num_value(self.pop_value().unwrap());
        let n2 = get_num_value(self.pop_value().unwrap());
        let result = n1 / n2;

        self.push_value(arithExp_as_exp(num(result)));
    }

    pub fn and_kw_rule(&mut self){
        let n1 = get_bool_value(self.pop_value().unwrap());
        let n2 = get_bool_value(self.pop_value().unwrap());
        let result = n1 && n2;

        self.push_value(boolExp_as_exp(boolean(result)));
    }

    pub fn or_kw_rule(&mut self){
        let n1 = get_bool_value(self.pop_value().unwrap());
        let n2 = get_bool_value(self.pop_value().unwrap());
        let result = n1 || n2;

        self.push_value(boolExp_as_exp(boolean(result)));
    }

    pub fn eq_kw_rule(&mut self){
        let n1 = get_bool_value(self.pop_value().unwrap());
        let n2 = get_bool_value(self.pop_value().unwrap());
        let mut result:bool;
        if n1 == n2{
            result = true
        }else{
            result = false
        }

        self.push_value(boolExp_as_exp(boolean(result)));
    }

    pub fn gt_kw_rule(&mut self){
        let n1 = get_num_value(self.pop_value().unwrap());
        let n2 = get_num_value(self.pop_value().unwrap());
        let result = n1 > n2;

        self.push_value(boolExp_as_exp(boolean(result)));
    }

    pub fn ge_kw_rule(&mut self){
        let n1 = get_num_value(self.pop_value().unwrap());
        let n2 = get_num_value(self.pop_value().unwrap());
        let result = n1 >= n2;

        self.push_value(boolExp_as_exp(boolean(result)));
    }

    pub fn lt_kw_rule(&mut self){
        let n1 = get_num_value(self.pop_value().unwrap());
        let n2 = get_num_value(self.pop_value().unwrap());
        let result = n1 < n2;

        self.push_value(boolExp_as_exp(boolean(result)));
    }

    pub fn le_kw_rule(&mut self){
        let n1 = get_num_value(self.pop_value().unwrap());
        let n2 = get_num_value(self.pop_value().unwrap());
        let result = n1 <= n2;

        self.push_value(boolExp_as_exp(boolean(result)));
    }

    pub fn neg_kw_rule(&mut self){
        let n = get_bool_value(self.pop_value().unwrap());
        let mut result:bool;
        result = !n;

        self.push_value(boolExp_as_exp(boolean(result)));
    }

    pub fn assign_kw_rule(&mut self){
        let value = self.pop_value().unwrap();
        let x = self.pop_value().unwrap();
        let key: String;

        /*
        let a: ArithExp;
        let b: BoolExp;
        let t: i32;
        match *x{
            Exp::ArithExp(aexp) => { t = 1; a = aexp },
            Exp::BoolExp(bexp) => { t = 2; b = bexp },
            //Exp::Id(id) =>  key = get_id_value(id) ,
            _ => unreachable!(),
        }

        if t == 1{
            match a{
                ArithExp::Id{value} => key = value,
                _ => unreachable!(),
            }
        }
        else if t == 2{
            match b{
                BoolExp::Id{value} => key = value,
                _ => unreachable!(),
            }
        }
        */

        key = exp_to_id_value(x).unwrap();

        self.store.insert(key,value);
    }

}

pub fn eval_aexp_aut(aexp: ArithExp, mut aut: PiAut) -> PiAut{
    match aexp{
        ArithExp::Num{value} => aut.push_value(arithExp_as_exp(num(value))),
        ArithExp::Sum{lhs,rhs} => aut.sum_rule(lhs,rhs),
        ArithExp::Sub{lhs,rhs} => aut.sub_rule(lhs,rhs),
        ArithExp::Mul{lhs,rhs} => aut.mul_rule(lhs,rhs),
        ArithExp::Div{lhs,rhs} => aut.div_rule(lhs,rhs),
        ArithExp::Id{value} => aut.push_value(arithExp_as_exp(id_as_aexp(id(value)))),
        _ => unreachable!(),
    }
    aut
}

pub fn eval_bexp_aut(bexp: BoolExp, mut aut: PiAut) -> PiAut{
    match bexp{
        BoolExp::Bool{value} => aut.push_value(boolExp_as_exp(boolean(value))),
        BoolExp::And{lhs,rhs} => aut.and_rule(lhs,rhs),
        BoolExp::Or{lhs,rhs} => aut.or_rule(lhs,rhs),
        BoolExp::Eq{lhs,rhs} => aut.eq_rule(lhs,rhs),
        BoolExp::Neg{value} => aut.neg_rule(value),
        BoolExp::Gt{lhs,rhs} => aut.gt_rule(lhs, rhs),
        BoolExp::Ge{lhs,rhs} => aut.ge_rule(lhs, rhs),
        BoolExp::Lt{lhs,rhs} => aut.lt_rule(lhs, rhs),
        BoolExp::Le{lhs,rhs} => aut.le_rule(lhs, rhs),
        BoolExp::Id{value} => aut.push_value(boolExp_as_exp(id_as_bexp(id(value)))),
        _ => unreachable!(),
    }
    aut
}

pub fn eval_exp_aut(expression: Exp,mut aut: PiAut) -> PiAut{
    match expression{
        Exp::ArithExp(aexp) => aut = eval_aexp_aut(aexp,aut),
        Exp::BoolExp(bexp) => aut = eval_bexp_aut(bexp,aut),
        //Exp::Id(id) => aut = eval_id_aut()
        _ => unreachable!(),
    }
    aut
}

pub fn eval_command(cmd: Cmd, mut aut: PiAut) -> PiAut{
    println!("CMD = {:?}", cmd);
    match cmd{
        Cmd::Assign{id,value} => aut.assign_rule(id,value),
        Cmd::CSeq{command, next_command} => aut.cseq_rule(command, next_command),
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
        KW::KWAnd => aut.and_kw_rule(),
        KW::KWOr => aut.or_kw_rule(),
        KW::KWEq => aut.eq_kw_rule(),
        KW::KWNeg => aut.neg_kw_rule(),
        KW::KWGt => aut.gt_kw_rule(),
        KW::KWGe => aut.ge_kw_rule(),
        KW::KWLt => aut.lt_kw_rule(),
        KW::KWLe => aut.le_kw_rule(),
        KW::KWAss => aut.assign_kw_rule(),
        _ => unreachable!(),
    }
    aut
}



pub fn eval_statement(statement: Statement ,mut aut: PiAut) -> PiAut{

    match statement{
        Statement::Exp(exp) => aut = eval_exp_aut(exp,aut),
        Statement::Cmd(cmd) => aut = eval_command(cmd,aut),
        _ => unreachable!(),
    }
    aut
}


pub fn eval_automata(mut aut: PiAut) -> PiAut{

    while !aut.control_stack.is_empty(){
        let tree = aut.pop_ctrl();
        match *tree.unwrap(){
            Ctrl_stack_type::KW(kw) => aut = eval_kw_aut(kw,aut),
            Ctrl_stack_type::Statement(statement) => aut = eval_statement(statement,aut),
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

pub fn gt(lhs: Box<ArithExp>, rhs: Box<ArithExp>) -> Box<BoolExp>{
    Box::new(BoolExp::Gt { lhs, rhs })
}

pub fn ge(lhs: Box<ArithExp>, rhs: Box<ArithExp>) -> Box<BoolExp>{
    Box::new(BoolExp::Ge { lhs, rhs })
}

pub fn lt(lhs: Box<ArithExp>, rhs: Box<ArithExp>) -> Box<BoolExp>{
    Box::new(BoolExp::Lt { lhs, rhs })
}

pub fn le(lhs: Box<ArithExp>, rhs: Box<ArithExp>) -> Box<BoolExp>{
    Box::new(BoolExp::Le { lhs, rhs })
}

pub fn id(value: String) -> Id {
    Id {value}
}

pub fn arith_id(value: String) -> Box<ArithExp> {
    Box::new(ArithExp::Id{ value })
}

pub fn bool_id(value: String) -> Box<BoolExp> {
    Box::new(BoolExp::Id{ value })
}

pub fn id_as_aexp(expression: Id) -> Box<ArithExp> {
    let x: String;
    match expression{
        Id{value} => x = value,
        _ => unreachable!(),
    }
    Box::new(ArithExp::Id{value: x})
}

pub fn id_as_bexp(expression: Id) -> Box<BoolExp> {
    let x: String;
    match expression{
        Id{value} => x = value,
        _ => unreachable!(),
    }
    Box::new(BoolExp::Id{value: x})
}

pub fn assign(id: Id, value: Box<Exp>) -> Box<Cmd> {
    Box::new(Cmd::Assign {id, value})
}

pub fn while_loop(boolExp: Box<BoolExp>, cmd: Box<Cmd>) -> Box<Cmd> {
    Box::new(Cmd::While {boolExp, cmd})
}

pub fn cseq(command: Box<Cmd>, next_command: Box<Cmd>) -> Box<Cmd> {
    Box::new(Cmd::CSeq {command, next_command})
}

pub fn get_id_value(id: Id) -> String{
    match id{
        Id{value} => value,
    }
}

//pub fn get_num_value(num: Box<ArithExp>) -> f64 {
pub fn get_num_value(num: Box<Exp>) -> f64 {
    let x: ArithExp;
    match *num {
        Exp::ArithExp(aexp) => x = aexp,
        _ => unreachable!(),
    }
    match x{
        ArithExp::Num{value} => value,
        _ => unreachable!(),
    }
}

pub fn get_bool_value(num: Box<Exp>) -> bool {
    let x: BoolExp;
    match *num {
        Exp::BoolExp(bexp) => x = bexp,
        _ => unreachable!(),
    }
    match x{
        BoolExp::Bool{value} => value,
        _ => unreachable!(),
    }
}

pub fn aexp_to_id_value(expression: Box<ArithExp>) -> Option<String>{
    let mut result: Option<String> = None;
    match *expression{
        ArithExp::Id{value} => result = Some(value),
        _ => unreachable!(),
    }
    result
}

pub fn exp_to_id_value(expression: Box<Exp>) -> Option<String>{
    let mut result: Option<String> = None;
    match *expression{
        Exp::ArithExp(aexp) => result = aexp_to_id_value(Box::new(aexp)),
        _ => unreachable!(),
    }
    result
}

pub fn arithExp_as_exp(expression: Box<ArithExp>) -> Box<Exp> {
    Box::new(Exp::ArithExp(*expression))
}

pub fn arithExp_as_statement(expression: Box<ArithExp>) -> Box<Statement> {
    Box::new(Statement::Exp(*arithExp_as_exp(expression)))
}
pub fn boolExp_as_exp(expression: Box<BoolExp>) -> Box<Exp> {
    Box::new(Exp::BoolExp(*expression))
}

pub fn boolExp_as_statement(expression: Box<BoolExp>) -> Box<Statement> {
    Box::new(Statement::Exp(*boolExp_as_exp(expression)))
}

pub fn exp_as_ctrl_stack_type(expression: Box<Exp>) -> Box<Ctrl_stack_type>{
    let x = exp_as_statement(expression);

    Box::new(Ctrl_stack_type::Statement(*x))
}

pub fn exp_as_statement(statement: Box<Exp>) -> Box<Statement> {
    Box::new(Statement::Exp(*statement))
}

pub fn cmd_as_statement(statement: Box<Cmd>) -> Box<Statement> {
    Box::new(Statement::Cmd(*statement))
}

pub fn kw_as_ctrl_stack_type(keyword: Box<KW>) -> Box<Ctrl_stack_type>{
    Box::new(Ctrl_stack_type::KW(*keyword))
}

pub fn statement_as_ctrl_stack_type(s: Box<Statement>) -> Box<Ctrl_stack_type>{
    Box::new(Ctrl_stack_type::Statement(*s))
}

pub fn exp_as_arithExp(expression: Box<Exp>) -> Box<ArithExp> {
    match *expression{
        Exp::ArithExp(aexp) => Box::new(aexp),
        _ => unreachable!(),
    }
}

pub fn exp_as_boolExp(expression: Box<Exp>) -> Box<BoolExp> {
    match *expression{
        Exp::BoolExp(bexp) => Box::new(bexp),
        _ => unreachable!(),
    }
}
