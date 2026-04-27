use vstd::prelude::*;
verus! {
type Matrix = Vec<Vec<i8>>;
fn transpose(arr: Matrix) -> (ret: Matrix){
    let rows = arr.len();
    let cols = arr[0].len();
    let mut ret: Matrix = Vec::new();
    let mut j: usize = 0;
    while j < cols
    {
        let mut new_row: Vec<i8> = Vec::new();
        let mut i: usize = 0;
        while i < rows
        {
            new_row.push(arr[i][j]);
            i = i + 1;
        }
        ret.push(new_row);
        j = j + 1;
    }
    ret
}
}
fn main() {}