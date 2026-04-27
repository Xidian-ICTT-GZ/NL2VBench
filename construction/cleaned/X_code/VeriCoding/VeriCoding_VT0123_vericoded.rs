use vstd::prelude::*;
verus! {
#[derive(PartialEq, Eq)]
pub enum NumpyDType {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float16,
    Float32,
    Float64,
    Complex64,
    Complex128,
}
#[derive(PartialEq, Eq)]
pub enum DTypeKind {
    Bool,
    SignedInteger,
    UnsignedInteger,
    Integral,
    RealFloating,
    ComplexFloating,
    Numeric,
}
fn numpy_isdtype(dtype: NumpyDType, kind: DTypeKind) -> (result: bool){
    let result: bool = match kind {
        DTypeKind::Bool => match dtype {
            NumpyDType::Bool => true,
            _ => false,
        },
        DTypeKind::SignedInteger => match dtype {
            NumpyDType::Int8 | NumpyDType::Int16 | NumpyDType::Int32 | NumpyDType::Int64 => true,
            _ => false,
        },
        DTypeKind::UnsignedInteger => match dtype {
            NumpyDType::UInt8 | NumpyDType::UInt16 | NumpyDType::UInt32 | NumpyDType::UInt64 => true,
            _ => false,
        },
        DTypeKind::Integral => match dtype {
            NumpyDType::Int8 | NumpyDType::Int16 | NumpyDType::Int32 | NumpyDType::Int64
            | NumpyDType::UInt8 | NumpyDType::UInt16 | NumpyDType::UInt32 | NumpyDType::UInt64 => true,
            _ => false,
        },
        DTypeKind::RealFloating => match dtype {
            NumpyDType::Float16 | NumpyDType::Float32 | NumpyDType::Float64 => true,
            _ => false,
        },
        DTypeKind::ComplexFloating => match dtype {
            NumpyDType::Complex64 | NumpyDType::Complex128 => true,
            _ => false,
        },
        DTypeKind::Numeric => match dtype {
            NumpyDType::Bool
            | NumpyDType::Int8 | NumpyDType::Int16 | NumpyDType::Int32 | NumpyDType::Int64
            | NumpyDType::UInt8 | NumpyDType::UInt16 | NumpyDType::UInt32 | NumpyDType::UInt64
            | NumpyDType::Float16 | NumpyDType::Float32 | NumpyDType::Float64
            | NumpyDType::Complex64 | NumpyDType::Complex128 => true,
        },
    };
    result
}
}
fn main() {}