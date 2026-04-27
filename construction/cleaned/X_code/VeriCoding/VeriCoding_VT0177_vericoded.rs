use vstd::prelude::*;
verus! {
fn create_row(n: u8) -> (row: Vec<u8>){
    let mut row_vec: Vec<u8> = Vec::new();
    let mut i: u8 = 0;
    while i < n
    {
        row_vec.push(i);
        i = i + 1;
    }
    row_vec
}
fn indices(n: u8) -> (grid: Vec<Vec<u8>>){
    let row = create_row(n);
    let mut grid: Vec<Vec<u8>> = Vec::new();
    grid.push(row);
    grid
}
}
fn main() {}