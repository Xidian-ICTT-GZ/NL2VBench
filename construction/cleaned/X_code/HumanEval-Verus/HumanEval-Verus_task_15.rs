use vstd::assert_seqs_equal;
use vstd::prelude::*;
verus! {
pub fn intersperse(numbers: Vec<u64>, delimiter: u64) -> (result: Vec<u64>){
    if numbers.len() <= 1 {
        numbers
    } else {
        let mut result = Vec::new();
        let mut index = 0;
        while index < numbers.len() - 1
        {
            result.push(numbers[index]);
            result.push(delimiter);
            index += 1;
        }
        result.push(numbers[numbers.len() - 1]);
        result
    }
}
} 