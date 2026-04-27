use vstd::prelude::*;
fn main() {}

verus!{
fn choose_odd(v: &Vec<u64>) -> (odd_index: usize){
    let mut j: usize = 0;
    while (j < v.len())
    {
        if v[j] % 2 == 1 {
            return j;
        }
        j = j + 1;
    }
    j
}
}