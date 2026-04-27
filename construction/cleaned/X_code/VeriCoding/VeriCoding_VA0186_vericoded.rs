use vstd::prelude::*;
verus! {
exec fn good_digit_count_exec(digit: char) -> (count: i32){
    if digit == '0' { 2 }
    else if digit == '1' { 7 }
    else if digit == '2' { 2 }
    else if digit == '3' { 3 }
    else if digit == '4' { 3 }
    else if digit == '5' { 4 }
    else if digit == '6' { 2 }
    else if digit == '7' { 5 }
    else if digit == '8' { 1 }
    else { 2 } 
}
exec fn int_to_digit_char(n: i32) -> (c: char){
    if n == 0 { '0' }
    else if n == 1 { '1' }
    else if n == 2 { '2' }
    else if n == 3 { '3' }
    else if n == 4 { '4' }
    else if n == 5 { '5' }
    else if n == 6 { '6' }
    else if n == 7 { '7' }
    else if n == 8 { '8' }
    else { '9' }
}
fn solve(input: Vec<char>) -> (result: Vec<char>){
    let d0 = input[0];
    let d1 = input[1];
    let count0 = good_digit_count_exec(d0);
    let count1 = good_digit_count_exec(d1);
    let product = count0 * count1;
    let tens = product / 10;
    let ones = product % 10;
    let mut result = Vec::new();
    if tens > 0 {
        let tens_char = int_to_digit_char(tens);
        result.push(tens_char);
    }
    let ones_char = int_to_digit_char(ones);
    result.push(ones_char);
    result.push('\n');
    result
}
}
fn main() {}