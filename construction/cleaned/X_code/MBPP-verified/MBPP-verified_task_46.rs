use vstd::prelude::*;
verus! {
fn is_product_even(arr: &Vec<u32>) -> (result: bool){
    let mut index = 0;
    while index < arr.len()
    {
        if (arr[index] % 2 == 0) {
            return true;
        }
        index += 1;
    }
    false
}
fn main() {}
} 