use vstd::prelude::*;

verus! {

spec fn fib(n: nat) -> nat
    decreases n,
{
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib((n - 2) as nat) + fib((n - 1) as nat)
    }
}

fn fib_impl(n: u64) -> (result: u64)
    requires
        fib(n as nat) <= u64::MAX
    ensures
        result == fib(n as nat)
{
    // no implementation
}

}