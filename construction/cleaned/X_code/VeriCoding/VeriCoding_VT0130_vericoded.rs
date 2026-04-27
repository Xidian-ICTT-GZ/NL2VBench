use vstd::prelude::*;
verus! {
#[derive(PartialEq, Eq, Structural)]
pub enum NumpyDType {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float16,
    Float32,
    Float64,
    Complex64,
    Complex128,
}
fn min_scalar_type(value: i8) -> (result: NumpyDType){
    if value >= 0 {
        NumpyDType::UInt8
    } else {
        NumpyDType::Int8
    }
}
}
fn main() {}