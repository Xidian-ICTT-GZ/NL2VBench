use vstd::prelude::*;
verus! {
pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}
impl<T: Copy> Matrix<T> {
}
fn create_constant_row(val: i8, len: usize) -> (res: Vec<i8>){
    let mut row_vec = Vec::with_capacity(len);
    let mut j: usize = 0;
    while j < len
    {
        row_vec.push(val);
        j += 1;
    }
    row_vec
}
fn copy_vec(v: &Vec<i8>) -> (res: Vec<i8>){
    let len = v.len();
    let mut new_vec = Vec::with_capacity(len);
    let mut i: usize = 0;
    while i < len
    {
        new_vec.push(v[i]);
        i += 1;
    }
    new_vec
}
fn broadcast(a: Vec<i8>, shape: Vec<u8>) -> (ret: Matrix<i8>){
    let rows = shape[0] as usize;
    let cols = shape[1] as usize;
    let mut data: Vec<Vec<i8>> = Vec::new();
    if rows == a.len() {
        let mut i: usize = 0;
        while i < rows
        {
            let row_vec = create_constant_row(a[i], cols);
            data.push(row_vec);
            i += 1;
        }
    } else {
        let mut i: usize = 0;
        while i < rows
        {
            let row_vec = copy_vec(&a);
            data.push(row_vec);
            i += 1;
        }
    }
    Matrix { data, rows, cols }
}
}
fn main() {}