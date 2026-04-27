use vstd::prelude::*;
verus! {
fn contains_z_from(text: &Vec<char>, i: usize) -> (res: bool){
    if i == text.len() {
        false
    } else {
        (text[i] == 'Z' || text[i] == 'z') || contains_z_from(text, i + 1)
    }
}
fn contains_z(text: &Vec<char>) -> (result: bool){
    let result = contains_z_from(text, 0);
    result
}
}
fn main() {}