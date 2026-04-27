use vstd::prelude::*;
verus! {
fn sort_three_exec(x: i8, y: i8, z: i8) -> (result: (i8, i8, i8)){
    if x <= y && x <= z {
        if y <= z { (x, y, z) } else { (x, z, y) }
    } else if y <= x && y <= z {
        if x <= z { (y, x, z) } else { (y, z, x) }
    } else {
        if x <= y { (z, x, y) } else { (z, y, x) }
    }
}
fn make_yes() -> (result: Vec<char>){
    let mut v: Vec<char> = Vec::new();
    v.push('Y');
    v.push('E');
    v.push('S');
    v
}
fn make_no() -> (result: Vec<char>){
    let mut v: Vec<char> = Vec::new();
    v.push('N');
    v.push('O');
    v
}
fn solve(nums: Vec<i8>) -> (result: Vec<char>){
    let x_i8 = nums[0];
    let y_i8 = nums[1];
    let z_i8 = nums[2];
    let (a, b, c) = sort_three_exec(x_i8, y_i8, z_i8);
    let result: Vec<char>;
    if a > 3i8 {
        result = make_no();
    } else if a == 3i8 {
        if b > 3i8 {
            result = make_no();
        } else if b == 3i8 {
            if c > 3i8 {
                result = make_no();
            } else {
                result = make_yes();
            }
        } else {
            result = make_no();
        }
    } else if a == 1i8 {
        result = make_yes();
    } else {
        if b == 2i8 {
            result = make_yes();
        } else if b > 4i8 {
            result = make_no();
        } else if b == 4i8 {
            if c == 4i8 {
                result = make_yes();
            } else {
                result = make_no();
            }
        } else {
            result = make_no();
        }
    }
    result
}
}
fn main() {}