use vstd::prelude::*;
verus! {
fn count_uppercase(text: &Vec<char>) -> (count: u64){
    let mut index = 0;
    let mut count = 0;
    while index < text.len()
    {
        if ((text[index] as u32) >= 65 && (text[index] as u32) <= 90) {
            count += 1;
        }
        index += 1;
    }
    count
}
fn main() {}
} 