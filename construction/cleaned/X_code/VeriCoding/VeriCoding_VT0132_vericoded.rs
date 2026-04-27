use vstd::prelude::*;
verus! {
#[derive(PartialEq, Eq)]
pub enum NumpyScalarType {
    Int32,
    Int64,
    Float32,
    Float64,
    Complex64,
    Complex128,
    Object,
    String,
    Bool,
}
pub enum NumpyObject {
    IntVal(i64),
    FloatVal(f64),
    ArrayInt(Vec<i64>),
    ArrayFloat(Vec<f64>),
    ArrayComplex(Vec<(f64, f64)>),
    GenericObj,
    StringVal(String),
    BoolVal(bool),
}
fn obj2sctype(rep: NumpyObject, default: Option<NumpyScalarType>) -> (result: Option<NumpyScalarType>){
    let res = match rep {
        NumpyObject::IntVal(_) => Some(NumpyScalarType::Int64),
        NumpyObject::FloatVal(_) => Some(NumpyScalarType::Float64),
        NumpyObject::ArrayInt(_) => Some(NumpyScalarType::Int64),
        NumpyObject::ArrayFloat(_) => Some(NumpyScalarType::Float64),
        NumpyObject::ArrayComplex(_) => Some(NumpyScalarType::Complex128),
        NumpyObject::StringVal(_) => Some(NumpyScalarType::String),
        NumpyObject::BoolVal(_) => Some(NumpyScalarType::Bool),
        NumpyObject::GenericObj => default,
    };
    res
}
}
fn main() {}