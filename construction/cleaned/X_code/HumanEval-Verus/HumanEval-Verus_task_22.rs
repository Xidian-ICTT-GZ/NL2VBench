use vstd::prelude::*;
verus! {
fn single_digit_number_to_char_impl(n: u8) -> (output: char){
    if n == 0 {
        '0'
    } else if n == 1 {
        '1'
    } else if n == 2 {
        '2'
    } else if n == 3 {
        '3'
    } else if n == 4 {
        '4'
    } else if n == 5 {
        '5'
    } else if n == 6 {
        '6'
    } else if n == 7 {
        '7'
    } else if n == 8 {
        '8'
    } else {
        '9'
    }
}
pub fn number_to_char_impl(n: u8) -> (char_vec: Vec<char>){
    let mut i = n;
    let mut output = vec![];
    while (i > 0)
    {
        let m = i % 10;
        let current = single_digit_number_to_char_impl(m);
        output.insert(0, current);
        i = i / 10;
    }
    return output;
}
fn string_sequence_impl(n: u8) -> (string_seq: Vec<char>){
    let mut i = n;
    let mut output = vec![];
    while (i > 0)
    {
        let mut next = number_to_char_impl(i);
        next.append(&mut output);
        output = next;
        output.insert(0, ' ');
        i = i - 1;
    }
    output.insert(0, '0');
    return output;
}
} 
fn main() {
    print!("{:?}", string_sequence_impl(0));
    print!("{:?}", string_sequence_impl(5));
    print!("{:?}", string_sequence_impl(20));
}
