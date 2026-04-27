use vstd::prelude::*;
verus! {
fn incr_list(l: Vec<i32>) -> (result: Vec<i32>){
    let mut result = Vec::with_capacity(l.len());
    for i in 0..l.len()
    {
        result.push(l[i] + 1);
    }
    result
}
} 