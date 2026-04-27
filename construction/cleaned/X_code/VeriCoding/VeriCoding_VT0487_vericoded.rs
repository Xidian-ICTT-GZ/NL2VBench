use vstd::prelude::*;
verus! {
fn trunc_len(n: usize, m: usize) -> (res: usize){
    if n > m { n - m } else { 1 }
}
fn legder(c: Vec<f64>, m: u8, scl: f64) -> (result: Vec<f64>){
    if m == 0u8 {
        return c;
    }
    let m_usize = m as usize;
    let n = c.len();
    if m_usize < n {
        let new_len = n - m_usize;
        let mut res: Vec<f64> = Vec::new();
        let mut i: usize = 0;
        while i < new_len
        {
            let x = c[i];
            res.push(x);
            i = i + 1;
        }
        res
    } else {
        let mut res: Vec<f64> = Vec::new();
        let v0 = c[0];
        res.push(v0);
        res
    }
}
}
fn main() {}