use vstd::prelude::*;
verus! {
fn solve(input: Vec<char>) -> (result: i8){
    let len = input.len();
    if input[0] == 'R' && input[1] == 'R' && input[2] == 'R' {
        return 3;
    }
    let has_two_consecutive = (input[0] == 'R' && input[1] == 'R') || (input[1] == 'R' && input[2] == 'R');
    if has_two_consecutive {
        return 2;
    }
    let has_single_r = input[0] == 'R' || input[1] == 'R' || input[2] == 'R';
    if has_single_r {
        return 1;
    }
    return 0;
}
}
fn main() {}