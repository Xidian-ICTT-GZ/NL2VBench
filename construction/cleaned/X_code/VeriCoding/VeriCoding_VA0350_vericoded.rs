use vstd::prelude::*;
verus! {
fn solve(queries: Vec<i8>) -> (results: Vec<i8>){
    let mut results = Vec::new();
    let mut i = 0;
    while i < queries.len()
    {
        let n = queries[i];
        let result = if n >= 4 {
            (n % 2) as i8
        } else {
            (4 - n) as i8
        };
        results.push(result);
        i = i + 1;
    }
    results
}
}
fn main() {}