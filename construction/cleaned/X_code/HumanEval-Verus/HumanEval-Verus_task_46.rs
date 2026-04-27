use vstd::prelude::*;
verus! {
fn flip_case(str: &[char]) -> (flipped_case: Vec<char>){
    let mut flipped_case = Vec::with_capacity(str.len());
    for index in 0..str.len()
    {
        if (str[index] >= 'a' && str[index] <= 'z') {
            flipped_case.push(((str[index] as u8) - 32) as char);
        } else if (str[index] >= 'A' && str[index] <= 'Z') {
            flipped_case.push(((str[index] as u8) + 32) as char);
        } else {
            flipped_case.push(str[index]);
        }
    }
    flipped_case
}
} 