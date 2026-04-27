use vstd::prelude::*;
verus! {
fn singleton_vec_i8(x: i8) -> (result: Vec<i8>){
    let mut v: Vec<i8> = Vec::new();
    v.push(x);
    v
}
fn copy_vec_i8(src: &Vec<i8>) -> (dest: Vec<i8>){
    let mut d: Vec<i8> = Vec::new();
    let mut j: usize = 0;
    while j < src.len()
    {
        let val = src[j];
        d.push(val);
        j += 1;
    }
    d
}
fn ix_(seq1: Vec<i8>, seq2: Vec<i8>) -> (result: (Vec<Vec<i8>>, Vec<Vec<i8>>)){
    let mut rows0: Vec<Vec<i8>> = Vec::new();
    let mut i: usize = 0;
    while i < seq1.len()
    {
        let val = seq1[i];
        let single = singleton_vec_i8(val);
        rows0.push(single);
        i += 1;
    }
    let row = copy_vec_i8(&seq2);
    let mut rows1: Vec<Vec<i8>> = Vec::new();
    rows1.push(row);
    (rows0, rows1)
}
}
fn main() {}