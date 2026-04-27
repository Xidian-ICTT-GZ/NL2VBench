use vstd::prelude::*;
verus! {
fn diag_val(i: int, j: int) -> (result: f64){
    if i == j { 1.0 } else { 0.0 }
}
fn identity(n: usize) -> (result: Vec<Vec<f64>>){
    let mut mat: Vec<Vec<f64>> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let mut row: Vec<f64> = Vec::new();
        let mut j: usize = 0;
        while j < n
        {
            let v: f64 = if j == i { 1.0 } else { 0.0 };
            row.push(v);
            j = j + 1;
        }
        mat.push(row);
        i = i + 1;
    }
    mat
}
}
fn main() {}