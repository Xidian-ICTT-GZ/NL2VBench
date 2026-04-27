use vstd::prelude::*;
verus! {
fn reverse_to_k(list: &Vec<i32>, n: usize) -> (reversed_list: Vec<i32>){
    let mut reversed_list = Vec::new();
    let mut index = 0;
    while index < n
    {
        reversed_list.push(list[n - 1 - index]);
        index += 1;
    }
    index = n;
    while index < list.len()
    {
        reversed_list.push(list[index]);
        index += 1;
    }
    reversed_list
}
fn main() {}
} 