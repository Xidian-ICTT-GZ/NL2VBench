use vstd::prelude::*;

fn main() {}
verus! {

fn sum(a: &Vec<u32>, b: &Vec<u32>) -> (c: Vec<u32>){
    let mut c = Vec::with_capacity(a.len());
    let mut n: usize = 0;
    let len: usize = a.len();
    while n != len {
        let sum: u32 = a[n] + b[n];
        c.push(sum);
        n = n + 1;
    }
    c
}

} // verus!