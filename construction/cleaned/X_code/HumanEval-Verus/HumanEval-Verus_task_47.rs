use vstd::prelude::*;
verus! {
fn concatenate_impl(strings: Vec<Vec<char>>) -> (joined: Vec<char>){
    let mut i = 0;
    let mut joined = vec![];
    while (i < strings.len())
    {
        let mut copy_str = strings[i].clone();
        joined.append(&mut copy_str);
        i = i + 1;
    }
    return joined;
}
} 
fn main() {
    let test1 = vec![vec!['a'], vec!['b'], vec!['c']];
    let test2: Vec<Vec<char>> = Vec::new();
    let test3 = vec![vec!['a', 'z'], vec!['b'], vec!['c', 'y']];
    print!("concatenation of {:?}:\n", test1);
    print!("{:?}\n", concatenate_impl(test1));
    print!("concatenation of {:?}:\n", test2);
    print!("{:?}\n", concatenate_impl(test2));
    print!("concatenation of {:?}:\n", test3);
    print!("{:?}\n", concatenate_impl(test3));
}