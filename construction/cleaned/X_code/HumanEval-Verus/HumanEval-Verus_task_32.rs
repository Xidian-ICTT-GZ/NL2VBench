use vstd::arithmetic::div_mod::{
    lemma_fundamental_div_mod, lemma_fundamental_div_mod_converse_div,
};
use vstd::prelude::*;
verus! {
fn largest_divisor(n: u32) -> (ret: u32){
    let mut i = n - 1;
    while i >= 2
    {
        if n % i == 0 {
            return i;
        }
        i -= 1;
    }
    1
}
} 