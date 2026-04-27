use vstd::prelude::*;
verus! {
#[derive(PartialEq, Eq)]
pub struct Complex {
    pub re: i32,
    pub im: i32,
}

fn real_if_close(arr: Vec<Complex>, tol: i32) -> (result: Vec<Complex>){
    let mut all_close = true;
    let mut i = 0;
    while i < arr.len()
    {
        let c = &arr[i];
        let im_val = c.im as i64;
        let abs_im = if im_val >= 0 { im_val } else { -im_val };
        let tol_val = tol as i64;
        let not_close = abs_im > tol_val * 2;
        if not_close {
            all_close = false;
        }
        i = i + 1;
    }
    let mut result: Vec<Complex> = Vec::new();
    let mut j = 0;
    while j < arr.len()
    {
        let c = &arr[j];
        if all_close {
            let new_c = Complex { re: c.re, im: 0 };
            result.push(new_c);
        } else {
            result.push(Complex { re: c.re, im: c.im });
        }
        j = j + 1;
    }
    result
}
}
fn main() {}