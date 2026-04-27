use vstd::prelude::*;
verus! {
fn tangent(r: Vec<u64>, x: Vec<u64>) -> (b: bool){
    let mut temp_b = false;
    let mut k: usize = 0;
    while k < r.len() && !temp_b
    {
        let mut l: usize = 0;
        let mut tangent_missing = false;
        while l < x.len() && !tangent_missing && k < r.len()
        {
            if r[k] == x[l] {
                temp_b = true;
            }
            if l == x.len() - 1 && r[k] < x[l] {
                tangent_missing = true;
            }
            l = l + 1;
        }
        k = k + 1;
    }
    temp_b
}
fn main() {}
}