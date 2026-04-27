use vstd::prelude::*;

fn main() {}

verus! {

fn sum_range_list(arr: &Vec<i64>, start: usize, end: usize) -> (sum: i128){
    let mut index = start;
    let mut sum = 0i128;
    let _end = end + 1;

    while index < _end {
        sum = sum + arr[index] as i128;
        index += 1;
    }
    sum
}

} // verus!