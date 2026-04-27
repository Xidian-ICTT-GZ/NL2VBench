use vstd::prelude::*;
verus! {
fn max2(a: i32, b: i32) -> i32 {
    if a >= b { a } else { b }
}
fn max_of_list(lst: Vec<i32>) -> (result: i32){
    let mut i: usize = 1;
    let mut cur: i32 = lst[0];
    while i < lst.len()
    {
        let v = lst[i];
        if v > cur {
            cur = v;
        }
        i = i + 1;
    }
    cur
}
}
fn main() {}