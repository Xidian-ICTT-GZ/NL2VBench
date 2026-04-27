use vstd::prelude::*;

fn main() {}

verus! {

pub fn linear_search(a: &Vec<i32>, e: i32) -> (n: usize){
    let mut n: usize = 0;
    while n != a.len() {
        if a[n] == e {
            return n;
        }
        n = n + 1;
    }
    n
}

} // verus!