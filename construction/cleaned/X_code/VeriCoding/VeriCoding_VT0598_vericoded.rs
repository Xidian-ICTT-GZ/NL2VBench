use vstd::prelude::*;
verus! {
fn vec_min_i8(arr: &Vec<i8>) -> (m: i8){
    let mut m_local = arr[0];
    let mut i: usize = 1;
    while i < arr.len()
    {
        if arr[i] < m_local {
            m_local = arr[i];
        }
        i += 1;
    }
    m_local
}
fn vec_max_i8(arr: &Vec<i8>) -> (m: i8){
    let mut m_local = arr[0];
    let mut i: usize = 1;
    while i < arr.len()
    {
        if arr[i] > m_local {
            m_local = arr[i];
        }
        i += 1;
    }
    m_local
}
fn percentile(arr: Vec<i8>, q: i8) -> (result: i8){
    let n = arr.len();
    if q == 0 {
        let m = vec_min_i8(&arr);
        m
    } else if q == 100 {
        let m = vec_max_i8(&arr);
        m
    } else {
        let r = arr[0];
        r
    }
}
}
fn main() {}