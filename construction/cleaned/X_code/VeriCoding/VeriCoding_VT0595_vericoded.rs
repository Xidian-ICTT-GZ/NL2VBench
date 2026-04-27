use vstd::prelude::*;
verus! {
fn get_min_val(a: &Vec<i8>) -> (result: i8){
    let mut min_val = a[0];
    let mut i = 1;
    while i < a.len()
    {
        if a[i] < min_val {
            min_val = a[i];
        }
        i = i + 1;
    }
    min_val
}
fn get_max_val(a: &Vec<i8>) -> (result: i8){
    let mut max_val = a[0];
    let mut i = 1;
    while i < a.len()
    {
        if a[i] > max_val {
            max_val = a[i];
        }
        i = i + 1;
    }
    max_val
}
fn nanquantile(a: Vec<i8>, q: i8) -> (result: i8){
    if q == 0 {
        get_min_val(&a)
    } else if q == 100 {
        get_max_val(&a)
    } else {
        get_min_val(&a)
    }
}
}
fn main() {}