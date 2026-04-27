use vstd::prelude::*;
verus! {
#[derive(PartialEq, Eq, Clone, Copy)]
enum DType {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Complex64,
    Complex128,
    Bool,
    Object,
}
fn get_dtype_precedence(dt: DType) -> (res: u8){
    match dt {
        DType::Bool => 0,
        DType::Int8 => 1,
        DType::Int16 => 2,
        DType::Int32 => 3,
        DType::Int64 => 4,
        DType::UInt8 => 5,
        DType::UInt16 => 6,
        DType::UInt32 => 7,
        DType::UInt64 => 8,
        DType::Float32 => 9,
        DType::Float64 => 10,
        DType::Complex64 => 11,
        DType::Complex128 => 12,
        DType::Object => 13,
    }
}
fn get_dtype_kind(dt: DType) -> (res: char){
    match dt {
        DType::Bool => 'b',
        DType::Int8 | DType::Int16 | DType::Int32 | DType::Int64 => 'i',
        DType::UInt8 | DType::UInt16 | DType::UInt32 | DType::UInt64 => 'u',
        DType::Float32 | DType::Float64 => 'f',
        DType::Complex64 | DType::Complex128 => 'c',
        DType::Object => 'O',
    }
}
fn find_max_precedence_type(types: Vec<DType>) -> (result: DType){
    let mut max_type = types[0];
    let mut i: usize = 1;
    while i < types.len()
    {
        if get_dtype_precedence(types[i]) > get_dtype_precedence(max_type) {
            max_type = types[i];
        }
        i = i + 1;
    }
    max_type
}
fn find_common_type(array_types: Vec<DType>, scalar_types: Vec<DType>) -> (result: Option<DType>){
    if array_types.len() == 0 {
        let max_scalar_type = find_max_precedence_type(scalar_types);
        Some(max_scalar_type)
    } else if scalar_types.len() == 0 {
        let max_array_type = find_max_precedence_type(array_types);
        Some(max_array_type)
    } else {
        let max_array_type = find_max_precedence_type(array_types);
        let max_scalar_type = find_max_precedence_type(scalar_types);
        if get_dtype_kind(max_array_type) == get_dtype_kind(max_scalar_type) {
            Some(max_array_type)
        } else {
            Some(max_scalar_type)
        }
    }
}
}
fn main() {}