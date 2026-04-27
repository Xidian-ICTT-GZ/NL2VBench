use vstd::prelude::*;
verus! {
fn broadcast_arrays(a: Vec<i8>, b: Vec<i8>) -> (result: (Vec<i8>, Vec<i8>)){
    let max_len = if a.len() > b.len() { a.len() } else { b.len() };
    let mut a_broadcast: Vec<i8>;
    if a.len() == 1 {
        let val = a[0];
        let mut out: Vec<i8> = Vec::new();
        let mut i: usize = 0;
        while i < max_len
        {
            out.push(val);
            i += 1;
        }
        a_broadcast = out;
    } else {
        a_broadcast = a.clone();
    }
    let mut b_broadcast: Vec<i8>;
    if b.len() == 1 {
        let val = b[0];
        let mut out: Vec<i8> = Vec::new();
        let mut i: usize = 0;
        while i < max_len
        {
            out.push(val);
            i += 1;
        }
        b_broadcast = out;
    } else {
        b_broadcast = b.clone();
    }
    (a_broadcast, b_broadcast)
}
}
fn main() {}