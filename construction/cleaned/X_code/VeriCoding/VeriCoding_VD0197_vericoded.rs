use vstd::prelude::*;
verus! {
#[derive(PartialEq, Eq)]
enum Exp {
    Const(int),
    Var(String),
    Plus(Box<Exp>, Box<Exp>),
    Mult(Box<Exp>, Box<Exp>),
}
fn optimize_correct(e: Exp, s: Map<String, int>){
    match e {
        Exp::Const(_) => {},
        Exp::Var(_) => {},
        Exp::Plus(e1, e2) => {
        },
        Exp::Mult(e1, e2) => {
        }
    }
}
fn main() {}
}