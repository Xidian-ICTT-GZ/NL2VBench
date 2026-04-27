use vstd::prelude::*;
verus! {
fn count_frequency(elements: &Vec<i64>, key: i64) -> (frequency: usize){
    let mut counter = 0;
    let mut index = 0;
    while index < elements.len()
    {
        if (elements[index] == key) {
            counter += 1;
        }
        index += 1;
    }
    counter
}
fn remove_duplicates(numbers: &Vec<i64>) -> (unique_numbers: Vec<i64>){
    let mut unique_numbers: Vec<i64> = Vec::new();
    for index in 0..numbers.len()
    {
        if count_frequency(&numbers, numbers[index]) == 1 {
            unique_numbers.push(numbers[index]);
        }
    }
    unique_numbers
}
} 