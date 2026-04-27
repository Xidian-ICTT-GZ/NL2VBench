use vstd::prelude::*;
verus! {
enum DataType {
    ScalarInt,
    ScalarFloat,
    ScalarComplex,
    ScalarBool,
    ScalarString,
    ArrayType,
    CompositeType,
    UnknownType,
}
fn issctype(rep: DataType) -> (result: bool){
    let result = match rep {
        DataType::ScalarInt
        | DataType::ScalarFloat
        | DataType::ScalarComplex
        | DataType::ScalarBool
        | DataType::ScalarString => true,
        DataType::ArrayType
        | DataType::CompositeType
        | DataType::UnknownType => false,
    };
    result
}
}
fn main() {}