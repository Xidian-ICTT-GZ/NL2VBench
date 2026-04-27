use vstd::prelude::*;
verus! {
#[derive(PartialEq, Eq, Structural)]
pub enum NumpyDType {
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    Complex64,
    Complex128,
    Bool,
}
pub enum NumpyOperand {
    Scalar(NumpyDType),
    Array(NumpyDType, Vec<i32>),
}
fn exec_operand_type(operand: &NumpyOperand) -> (result: NumpyDType){
    let dtype = match operand {
        NumpyOperand::Scalar(dtype) => dtype,
        NumpyOperand::Array(dtype, _) => dtype,
    };
    match dtype {
        NumpyDType::Bool => NumpyDType::Bool,
        NumpyDType::Int8 => NumpyDType::Int8,
        NumpyDType::Int16 => NumpyDType::Int16,
        NumpyDType::Int32 => NumpyDType::Int32,
        NumpyDType::Int64 => NumpyDType::Int64,
        NumpyDType::Float32 => NumpyDType::Float32,
        NumpyDType::Float64 => NumpyDType::Float64,
        NumpyDType::Complex64 => NumpyDType::Complex64,
        NumpyDType::Complex128 => NumpyDType::Complex128,
    }
}
fn exec_type_rank(dtype: &NumpyDType) -> (rank: u8){
    match *dtype {
        NumpyDType::Bool => 0,
        NumpyDType::Int8 => 1,
        NumpyDType::Int16 => 2,
        NumpyDType::Int32 => 3,
        NumpyDType::Int64 => 4,
        NumpyDType::Float32 => 5,
        NumpyDType::Float64 => 6,
        NumpyDType::Complex64 => 7,
        NumpyDType::Complex128 => 8,
    }
}
fn exec_promote_types(t1: NumpyDType, t2: NumpyDType) -> (result: NumpyDType){
    if exec_type_rank(&t1) >= exec_type_rank(&t2) {
        t1
    } else {
        t2
    }
}
fn result_type(operands: Vec<NumpyOperand>) -> (result: NumpyDType){
    let mut result = exec_operand_type(&operands[0]);
    let mut i: usize = 1;
    while i < operands.len()
    {
        let current_op_type = exec_operand_type(&operands[i]);
        let old_result = result;
        result = exec_promote_types(old_result, current_op_type);
        i = i + 1;
    }

    result
}
}
fn main() {}