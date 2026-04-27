use vstd::prelude::*;
verus! {
fn not_equal(a: Vec<i8>, b: Vec<i8>) -> (result: Vec<bool>){
    let n = a.len();
    let mut res: Vec<bool> = Vec::new();
    while res.len() < n
    {
        let i = res.len();
        let xi: i8 = a[i];
        let yi: i8 = b[i];
        let val: bool = xi != yi;
        res.push(val);
    }
    res
}
}
fn main() {}