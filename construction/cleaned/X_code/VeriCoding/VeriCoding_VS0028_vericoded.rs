use vstd::prelude::*;
verus! {
fn is_close_i8(a: i8, b: i8, tol: i8) -> (res: bool){
    let diff = (a as i16) - (b as i16);
    let tol_i16 = tol as i16;
    -tol_i16 < diff && diff < tol_i16
}
fn np_isclose(a: Vec<i8>, b: Vec<i8>, tol: i8) -> (result: Vec<bool>){
    let mut result = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        let close = is_close_i8(a[i], b[i], tol);
        result.push(close);
        i = i + 1;
    }
    result
}
}
fn main() {}