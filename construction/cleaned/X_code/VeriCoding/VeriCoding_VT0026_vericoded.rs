use vstd::prelude::*;
verus! {
fn meshgrid(x: Vec<f32>, y: Vec<f32>) -> (result: (Vec<Vec<f32>>, Vec<Vec<f32>>)){
    let mut xx: Vec<Vec<f32>> = Vec::new();
    let mut yy: Vec<Vec<f32>> = Vec::new();
    let mut i = 0;
    while i < y.len()
    {
        let mut x_row: Vec<f32> = Vec::new();
        let mut y_row: Vec<f32> = Vec::new();
        let mut j = 0;
        while j < x.len()
        {
            x_row.push(x[j]);
            y_row.push(y[i]);
            j += 1;
        }
        xx.push(x_row);
        yy.push(y_row);
        i += 1;
    }
    (xx, yy)
}
}
fn main() {}