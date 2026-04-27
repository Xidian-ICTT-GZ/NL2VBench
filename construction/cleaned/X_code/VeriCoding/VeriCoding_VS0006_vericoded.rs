use vstd::prelude::*;
verus! {
fn bitwise_or(a: Vec<u8>, b: Vec<u8>) -> (result: Vec<u8>){
    let n = a.len();
    let mut res: Vec<u8> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let ai = a[i];
        let bi = b[i];
        let c: u8 = ai | bi;
        res.push(c);
        let ri = res[i];
        i = i + 1;
    }
    res
}
}
fn main() {}