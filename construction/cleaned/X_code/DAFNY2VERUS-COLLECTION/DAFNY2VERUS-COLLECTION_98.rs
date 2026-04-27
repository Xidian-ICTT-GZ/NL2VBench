use vstd::prelude::*;
verus! {
fn find(blood: Vec<i32>, key: i32) -> (index: i32){
    let mut index: i32 = 0;
    while index < blood.len() as i32
    {
        if blood[index as usize] == key {
            return index;
        }
        index += 1;
    }
    -1
}
fn main() {}
}