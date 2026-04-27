use vstd::prelude::*;
verus! {
fn solve(queries: Vec<(i8, i8)>) -> (results: Vec<i8>){
    let mut results = Vec::new();
    for i in 0..queries.len()
    {
        let l = queries[i].0;
        let r = queries[i].1;
        let l_i32 = l as i32;
        let r_i32 = r as i32;
        let l_minus_1_i32 = l_i32 - 1;
        let prefix_r_i32 = if (r_i32 % 2) == 0 { r_i32 / 2 } else { - (r_i32 + 1) / 2 };
        let prefix_l_minus_1_i32 = if (l_minus_1_i32 % 2) == 0 { l_minus_1_i32 / 2 } else { - (l_minus_1_i32 + 1) / 2 };
        let sum_i32 = prefix_r_i32 - prefix_l_minus_1_i32;
        results.push(sum_i32 as i8);
    }
    results
}
}
fn main() {}