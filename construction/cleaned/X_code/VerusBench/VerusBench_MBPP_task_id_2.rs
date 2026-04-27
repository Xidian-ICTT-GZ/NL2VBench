use vstd::prelude::*;

fn main() {}

verus! {

fn contains(arr: &Vec<i32>, key: i32) -> (result: bool){
    let mut i = 0;
    while i < arr.len() {
        if (arr[i] == key) {
            return true;
        }
        i += 1;
    }
    false
}

fn shared_elements(list1: &Vec<i32>, list2: &Vec<i32>) -> (shared: Vec<i32>){
    let mut shared = Vec::new();
    let mut index = 0;
    while index < list1.len() {
        if (contains(list2, list1[index]) && !contains(&shared, list1[index])) {
            shared.push(list1[index]);
        }
        index += 1
    }
    shared
}

} // verus!