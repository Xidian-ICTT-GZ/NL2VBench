use vstd::prelude::*;
verus! {
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DateTime64 {
    Valid(i64),
    Nat,
}
fn isnat(x: Vec<DateTime64>) -> (result: Vec<bool>){
    let mut res: Vec<bool> = Vec::new();
    let mut i: usize = 0;
    while i < x.len()
    {
        let xi = x[i];
        let b = match xi {
            DateTime64::Nat => true,
            DateTime64::Valid(_) => false,
        };
        res.push(b);
        i = i + 1;
    }
    res
}
}
fn main() {}