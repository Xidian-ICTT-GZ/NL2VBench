use vstd::prelude::*;
verus! {
fn getdomain(x: Vec<i8>) -> (result: Vec<i8>){
    let mut minv: i8 = x[0];
    let mut maxv: i8 = x[0];
    let mut i: usize = 1;
    while i < x.len()
    {
        let xi = x[i];
        if xi < minv {
            minv = xi;
        } else if xi > maxv {
            maxv = xi;
        }
        i = i + 1;
    }
    let mut r: Vec<i8> = Vec::new();
    r.push(minv);
    r.push(maxv);
    r
}
}
fn main() {}