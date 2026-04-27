use vstd::prelude::*;
fn main() {}
verus!{
pub fn havoc_inline_post(v: &mut Vec<u32>, a: u32, b: bool){  
    let c: bool = !b;
    let mut idx: usize = v.len();
    while (idx > 0)
    {
        idx = idx - 1;
        v.set(idx, v[idx] + a);
    }
}
}