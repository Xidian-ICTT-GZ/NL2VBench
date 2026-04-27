fn loop_triangle(n: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut idx: u32 = 0;

    while idx < n {
        idx += 1;
        sum += idx;
    }

    sum
}
