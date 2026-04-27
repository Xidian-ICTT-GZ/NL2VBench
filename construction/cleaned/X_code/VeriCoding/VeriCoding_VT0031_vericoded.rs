use vstd::prelude::*;
verus! {
fn val_for(i: usize, j: usize, k: i32) -> (result: f64){
    if k >= 0 {
        let kuz: usize = k as usize;
        if j <= i {
            1.0
        } else {
            let d: usize = j - i;
            if d <= kuz {
                1.0
            } else {
                0.0
            }
        }
    } else {
        let neg64: i64 = -(k as i64);
        let neg_us: usize = neg64 as usize;
        if i < neg_us {
            0.0
        } else {
            let upper: usize = i - neg_us;
            if j <= upper {
                1.0
            } else {
                0.0
            }
        }
    }
}
fn tri(n: usize, m: usize, k: i32) -> (result: Vec<Vec<f64>>){
    let mut res: Vec<Vec<f64>> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let mut row: Vec<f64> = Vec::new();
        let mut j: usize = 0;
        while j < m
        {
            let v = val_for(i, j, k);
            row.push(v);
            j = j + 1;
        }
        res.push(row);
        i = i + 1;
    }
    res
}
}
fn main() {}