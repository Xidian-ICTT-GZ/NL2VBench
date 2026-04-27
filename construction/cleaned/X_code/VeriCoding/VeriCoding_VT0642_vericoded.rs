use vstd::prelude::*;
verus! {
fn singleton_vec_from_string(s: String) -> (v: Vec<String>){
    let mut v: Vec<String> = Vec::new();
    v.push(s);
    v
}
fn splitlines(a: Vec<String>, keepends: bool) -> (result: Vec<Vec<String>>){
    let mut b: Vec<String> = a;
    let mut result: Vec<Vec<String>> = Vec::new();
    while b.len() > 0
    {
        let opt = b.pop();
        match opt {
            Option::Some(s) => {
                let inner: Vec<String> = singleton_vec_from_string(s);
                result.push(inner);
            }
            Option::None => {
            }
        }
    }
    result
}
}
fn main() {}