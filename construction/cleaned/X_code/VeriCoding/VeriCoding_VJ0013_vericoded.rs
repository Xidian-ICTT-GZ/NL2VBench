use vstd::prelude::*;
verus!{
fn conditional_average(vals_1: &Vec<u64>, vals_2: &Vec<u64>, conds_1: &Vec<bool>, conds_2: &Vec<bool>, avgs: &mut Vec<u64>){
    avgs.clear();
    let mut i = 0;
    while i < vals_1.len()
    {
        let avg_val = if conds_1[i] && conds_2[i] {
            (vals_1[i] + vals_2[i]) / 2
        } else if conds_1[i] && !conds_2[i] {
            vals_1[i]
        } else {
            vals_2[i]
        };
        avgs.push(avg_val);
        i = i + 1;
    }
}
}
fn main() {}