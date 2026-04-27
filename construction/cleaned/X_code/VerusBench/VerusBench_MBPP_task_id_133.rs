use vstd::prelude::*;

fn main() {}

verus! {

fn sum_negatives(arr: &Vec<i64>) -> (sum_neg: i128){
    let mut index = 0;
    let mut sum_neg = 0i128;

    while index < arr.len() {
        if (arr[index] < 0) {
            sum_neg = sum_neg + (arr[index] as i128);
        }
        index += 1;

    }
    sum_neg
}

} // verus!