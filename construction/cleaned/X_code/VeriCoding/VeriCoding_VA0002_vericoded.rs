use vstd::prelude::*;
verus! {
fn solve(n: i8) -> (result: Vec<i8>){
  let complete_weeks = n / 7_i8;
  let remaining_days = n % 7_i8;
  let min_additional = if remaining_days > 5_i8 { remaining_days - 5_i8 } else { 0_i8 };
  let max_additional = if remaining_days < 2_i8 { remaining_days } else { 2_i8 };
  let min_val = 2_i8 * complete_weeks + min_additional;
  let max_val = 2_i8 * complete_weeks + max_additional;
  let mut result = Vec::new();
  result.push(min_val);
  result.push(max_val);
  result
}
}
fn main() {}