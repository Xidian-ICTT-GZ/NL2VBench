use vstd::prelude::*;
verus! {
fn numpy_matrix_transpose(x: Vec<Vec<f32>>) -> (result: Vec<Vec<f32>>){
    let num_rows = x.len();
    let num_cols = x[0].len();
    let mut result: Vec<Vec<f32>> = Vec::new();
    let mut j: usize = 0;
    while j < num_cols
    {
        let mut new_row: Vec<f32> = Vec::new();
        let mut i: usize = 0;
        while i < num_rows
        {
            new_row.push(x[i][j]);
            i = i + 1;
        }
        result.push(new_row);
        j = j + 1;
    }
    result
}
}
fn main() {}