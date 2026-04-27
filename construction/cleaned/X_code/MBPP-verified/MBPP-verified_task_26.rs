use vstd::prelude::*;
verus! {
fn count_digits(text: &Vec<char>) -> (count: usize){
    let mut count = 0;
    let mut index = 0;
    while index < text.len()
    {
        if ((text[index] as u8) >= 48 && (text[index] as u8) <= 57) {
            count += 1;
        }
        index += 1;
    }
    count
}
fn main() {}
} 