use vstd::prelude::*;
verus! {
fn is_month_with_30_days(month: i32) -> (result: bool){
    month == 4 || month == 6 || month == 9 || month == 11
}
fn main() {
}
}