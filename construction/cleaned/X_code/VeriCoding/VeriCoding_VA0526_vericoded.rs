use vstd::prelude::*;
verus! {
fn pronunciation_vec(n: i8) -> (v: Vec<char>){
    let mut vec: Vec<char> = Vec::new();
    let ones: i8 = n % 10;
    if ones == 2 || ones == 4 || ones == 5 || ones == 7 || ones == 9 {
        vec.push('h');
        vec.push('o');
        vec.push('n');
        vec.push('\n');
    } else if ones == 0 || ones == 1 || ones == 6 || ones == 8 {
        vec.push('p');
        vec.push('o');
        vec.push('n');
        vec.push('\n');
    } else {
        vec.push('b');
        vec.push('o');
        vec.push('n');
        vec.push('\n');
    }
    vec
}
fn solve(n: i8) -> (result: Vec<char>){
    let result: Vec<char> = pronunciation_vec(n);
    result
}
}
fn main() {}