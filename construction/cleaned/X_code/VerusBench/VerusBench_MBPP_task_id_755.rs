use vstd::prelude::*;
fn main() {}

verus! {

fn second_smallest(numbers: &Vec<i32>) -> (indices: (usize, usize)){
    let mut min_index: usize = 0;
    let mut second_min_index: usize = 1;

    if numbers[1] < numbers[0] {
        min_index = 1;
        second_min_index = 0;
    }
    let mut index = 2;
    while index < numbers.len() {
        if numbers[index] < numbers[min_index] {
            second_min_index = min_index;
            min_index = index;
        } else if numbers[index] < numbers[second_min_index] {
            second_min_index = index;
        }
        index += 1;
    }
    (min_index, second_min_index)
}

} // verus!