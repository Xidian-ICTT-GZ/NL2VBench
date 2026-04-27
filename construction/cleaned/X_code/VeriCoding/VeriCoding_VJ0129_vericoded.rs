use vstd::prelude::*;
verus! {
fn is_even_exec(n: u32) -> (result: bool){
    (n % 2) == 0
}
fn is_product_even(arr: &Vec<u32>) -> (result: bool){
    for i in 0..arr.len()
    {
        if is_even_exec(arr[i])
        {
            return true;
        }
    }
    false
}
}
fn main() {}