use vstd::prelude::*;
verus! {
fn is_sub_list_at_index(main: &Vec<i32>, sub: &Vec<i32>, idx: usize) -> (result: bool){
    let mut i = 0;
    while i < sub.len()
    {
        if (main[idx + i] != sub[i]) {
            return false;
        }
        i += 1;
    }
    true
}
fn is_sub_list(main: &Vec<i32>, sub: &Vec<i32>) -> (result: bool){
    if sub.len() > main.len() {
        return false;
    }
    let mut index = 0;
    while index <= (main.len() - sub.len())
    {
        if (is_sub_list_at_index(&main, &sub, index)) {
            return true;
        }
        index += 1;
    }
    false
}
fn main() {}
} 