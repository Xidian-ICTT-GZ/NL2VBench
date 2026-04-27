use vstd::prelude::*;
verus! {
fn find_minimum_total_distance(x1: i8, x2: i8, x3: i8) -> (result: i8){
    let max_pos_i8 = if x1 >= x2 && x1 >= x3 { x1 }
                     else if x2 >= x1 && x2 >= x3 { x2 }
                     else { x3 };
    let min_pos_i8 = if x1 <= x2 && x1 <= x3 { x1 }
                     else if x2 <= x1 && x2 <= x3 { x2 }
                     else { x3 };
    let diff_i8 = max_pos_i8 - min_pos_i8;
    diff_i8
}
}
fn main() {}