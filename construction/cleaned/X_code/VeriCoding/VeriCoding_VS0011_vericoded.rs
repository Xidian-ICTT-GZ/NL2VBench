use vstd::prelude::*;
verus! {
fn get_column(input: &Vec<Vec<i8>>, j: usize, m: usize, n: usize) -> (col: Vec<i8>){
    let mut new_row: Vec<i8> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let val = input[i][j];
        new_row.push(val);
        i = i + 1;
    }
    new_row
}
fn column_stack(input: Vec<Vec<i8>>, m: usize, n: usize) -> (result: Vec<Vec<i8>>){
    let mut result: Vec<Vec<i8>> = Vec::new();
    let mut j: usize = 0;
    while j < m
    {
        let new_row = get_column(&input, j, m, n);
        result.push(new_row);
        j = j + 1;
    }
    result
}
}
fn main() {}