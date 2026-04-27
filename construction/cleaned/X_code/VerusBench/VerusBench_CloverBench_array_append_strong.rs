use vstd::prelude::*;

fn main() {}
verus! {

fn append(v: &Vec<u64>, elem: u64) -> (c: Vec<u64>){
    let mut c = Vec::with_capacity(v.len() + 1);
    let mut n: usize = 0;
    let len: usize = v.len();
    while n != len {
        c.push(v[n]);
        n = n + 1;
    }
    c.push(elem);
    c
}

} // verus!