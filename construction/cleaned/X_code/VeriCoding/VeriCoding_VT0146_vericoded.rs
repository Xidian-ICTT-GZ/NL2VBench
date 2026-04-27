use vstd::prelude::*;
verus! {
#[derive(PartialEq, Eq, Structural)]
pub enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
    Picosecond,
    Femtosecond,
    Attosecond,
}
#[derive(PartialEq, Eq, Structural)]
pub struct TimeDelta64 {
    pub value: i64,
    pub unit: TimeUnit,
}
fn timedelta64(value: i64, unit: TimeUnit) -> (result: TimeDelta64){
    TimeDelta64 { value, unit }
}
}
fn main() {}