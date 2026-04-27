use vstd::prelude::*;
verus! {
fn is_greater(n: i32, a: &Vec<i32>) -> (result: bool){
    let mut res: bool = true;
    let mut i: usize = 0;
    while i < a.len()
    {
        let ai = a[i];
        res = res && n > ai;
        i = i + 1;
    }
    res
}
}
fn main() {}