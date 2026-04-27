use vstd::prelude::*;
verus! {
fn is_prime_impl(n: u8) -> (res: bool){
    if n < 2 {
        return false;
    }
    let mut k = 2;
    let mut res = true;
    while (k < n)
    {
        res = res && n % k != 0;
        k = k + 1;
    }
    return res;
}
} 
fn main() {
    print!("6 is prime? {}\n", is_prime_impl(6));
    print!("101 is prime? {}\n", is_prime_impl(101));
    print!("11 is prime? {}\n", is_prime_impl(11));
    print!("61 is prime? {}\n", is_prime_impl(61));
    print!("4 is prime? {}\n", is_prime_impl(4));
    print!("1 is prime? {}\n", is_prime_impl(1));
}