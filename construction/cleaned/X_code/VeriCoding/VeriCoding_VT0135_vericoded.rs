use vstd::prelude::*;
verus! {
pub enum ScalarType {
    Int32,
    Int64,
    Float32,
    Float64,
    Complex64,
    Complex128,
    Bytes,
    Object,
}
fn code_of(sctype: ScalarType) -> (result: &'static str){
    match sctype {
        ScalarType::Int32 => "l",
        ScalarType::Int64 => "q",
        ScalarType::Float32 => "f",
        ScalarType::Float64 => "d",
        ScalarType::Complex64 => "F",
        ScalarType::Complex128 => "D",
        ScalarType::Bytes => "S",
        ScalarType::Object => "O",
    }
}
fn sctype2char(sctype: ScalarType) -> (result: &'static str){
    let r = code_of(sctype);
    r
}
}
fn main() {}