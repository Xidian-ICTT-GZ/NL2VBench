use vstd::prelude::*;
fn main() {}

verus! {

fn to_toggle_case(str1: &[u8]) -> (toggle_case: Vec<u8>){
    let mut toggle_case = Vec::with_capacity(str1.len());
    let mut index = 0;
    while index < str1.len() {
        if (str1[index] >= 97 && str1[index] <= 122) {
            toggle_case.push((str1[index] - 32) as u8);
        } else if (str1[index] >= 65 && str1[index] <= 90) {
            toggle_case.push((str1[index] + 32) as u8);
        } else {
            toggle_case.push(str1[index]);
        }
        index += 1;

    }
    toggle_case
}

} // verus!