fn fib_impl(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut prev: u64 = 0;
    let mut cur: u64 = 1;
    let mut i: u64 = 1;

    while i < n {
        i += 1;
        let new_cur = cur + prev;
        prev = cur;
        cur = new_cur;
    }

    cur
}