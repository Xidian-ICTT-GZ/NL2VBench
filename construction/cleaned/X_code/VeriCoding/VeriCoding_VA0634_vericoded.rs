use vstd::prelude::*;
verus! {
fn solve(input: Vec<char>) -> (result: Vec<char>){
    let a = input[0];
    let b = input[1];
    let c = input[2];
    let d = input[3];
    let a_val = (a as u32 - '0' as u32) as i32;
    let b_val = (b as u32 - '0' as u32) as i32;
    let c_val = (c as u32 - '0' as u32) as i32;
    let d_val = (d as u32 - '0' as u32) as i32;
    let mut op1: char = '+';
    let mut op2: char = '+';
    let mut op3: char = '+';
    if a_val + b_val + c_val + d_val == 7 {
        op1 = '+';
        op2 = '+';
        op3 = '+';
    } else if a_val + b_val + c_val - d_val == 7 {
        op1 = '+';
        op2 = '+';
        op3 = '-';
    } else if a_val + b_val - c_val + d_val == 7 {
        op1 = '+';
        op2 = '-';
        op3 = '+';
    } else if a_val + b_val - c_val - d_val == 7 {
        op1 = '+';
        op2 = '-';
        op3 = '-';
    } else if a_val - b_val + c_val + d_val == 7 {
        op1 = '-';
        op2 = '+';
        op3 = '+';
    } else if a_val - b_val + c_val - d_val == 7 {
        op1 = '-';
        op2 = '+';
        op3 = '-';
    } else if a_val - b_val - c_val + d_val == 7 {
        op1 = '-';
        op2 = '-';
        op3 = '+';
    } else {
        op1 = '-';
        op2 = '-';
        op3 = '-';
    }
    let result = vec![a, op1, b, op2, c, op3, d, '=', '7', '\n'];
    result
}
}
fn main() {}