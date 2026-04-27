use vstd::prelude::*;

fn main() {}

verus! {

fn count_digits(text: &[u8]) -> (count: usize){
    let mut count = 0;
    let mut index = 0;

    while index < text.len() {
        if (text[index] >= 48 && text[index] <= 57) {
            count += 1;
        }
        index += 1;
    }
    count
}

} // verus!