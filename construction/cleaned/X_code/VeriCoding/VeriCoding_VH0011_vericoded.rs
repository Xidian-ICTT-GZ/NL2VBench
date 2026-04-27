use vstd::prelude::*;
verus! {
fn bit_xor_char(x: char, y: char) -> (c: char){
    if x == y { '0' } else { '1' }
}
fn string_xor(a: Vec<char>, b: Vec<char>) -> (result: Vec<char>){
    let n = a.len();
    let mut result_vec: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let c = bit_xor_char(a[i], b[i]);
        result_vec.push(c);
        i += 1;
    }
    result_vec
}
}
fn main() {}