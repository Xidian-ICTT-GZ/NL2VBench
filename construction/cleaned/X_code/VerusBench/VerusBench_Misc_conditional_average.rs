use vstd::prelude::*;
fn main() {}

verus!{
fn conditional_average(vals_1: &Vec<u64>, vals_2: &Vec<u64>, conds_1: &Vec<bool>, conds_2: &Vec<bool>, avgs: &mut Vec<u64>){
    let mut k: usize = 0;
    let common_len = vals_1.len();
    avgs.clear();
    while (k < common_len)
    {
        let mut new_avg: u64 = 0;
        if (conds_1[k]) {
            if (conds_2[k]) {
                new_avg = (vals_1[k] + vals_2[k]) / 2;
            }
            else {
                new_avg = vals_1[k];
            }
        }
        else {
            new_avg = vals_2[k];
        }
        avgs.push(new_avg);
        k = k + 1;
    }
}
}