use vstd::prelude::*;
use vstd::slice::*;
verus! {
fn string_xor(a: &[char], b: &[char]) -> (result: Vec<char>){
    let a_len = a.len();
    let mut result = Vec::with_capacity(a_len);
    for pos in 0..a_len
    {
        if *slice_index_get(a, pos) == *slice_index_get(b, pos) {
            result.push('0');
        } else {
            result.push('1');
        }
    }
    result
}
} 