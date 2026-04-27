use vstd::prelude::*;
verus! {
fn derivative(xs: &Vec<u32>) -> (ret: Vec<u64>){
    let mut ret = Vec::new();
    if xs.len() == 0 {
        return ret;
    }
    let mut i = 1;
    while i < xs.len()
    {
        ret.push((i as u64) * (xs[i] as u64));
        i += 1;
    }
    ret
}
} 