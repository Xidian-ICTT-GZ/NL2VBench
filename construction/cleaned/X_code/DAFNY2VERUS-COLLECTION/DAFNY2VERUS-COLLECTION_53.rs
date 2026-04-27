use vstd::prelude::*;
use vstd::seq::*;
use vstd::map::*;
verus! {
fn reverse(a: Vec<char>) -> (b: Vec<char>){
    let mut b: Vec<char> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        b.push(a[a.len() - i - 1]);
        i += 1;
    }
    b
}
} 
fn main() {
    let a = vec!['s', 'k', 'r', 'o', 'w', 't', 'i'];
    let b = reverse(a);
    assert_eq!(b.as_slice(), &['i', 't', 'w', 'o', 'r', 'k', 's']);
    println!("{:?}", b);
    let a = vec!['!'];
    let b = reverse(a);
    assert_eq!(b.as_slice(), &['!']);
    println!("{}", b.iter().collect::<String>());
}