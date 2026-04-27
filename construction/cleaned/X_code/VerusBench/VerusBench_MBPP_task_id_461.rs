use vstd::prelude::*;

fn main() {}

verus! {

fn count_uppercase(text: &[u8]) -> (count: u64){
    let mut index = 0;
    let mut count = 0;

    while index < text.len() {
        if (text[index] >= 65 && text[index] <= 90) {
            count += 1;
        }
        index += 1;
    }
    count
}

} // verus!