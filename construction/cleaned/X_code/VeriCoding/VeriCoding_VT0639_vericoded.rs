use vstd::prelude::*;
verus! {
fn make_singleton(s: String) -> (v: Vec<String>){
    let mut v: Vec<String> = Vec::new();
    v.push(s);
    v
}
fn rsplit(a: Vec<String>, sep: String, maxsplit: u8) -> (result: Vec<Vec<String>>){
    let mut r: Vec<Vec<String>> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        let s = a[i].clone();
        let v = make_singleton(s);
        r.push(v);
        i = i + 1;
    }
    r
}
}
fn main() {}