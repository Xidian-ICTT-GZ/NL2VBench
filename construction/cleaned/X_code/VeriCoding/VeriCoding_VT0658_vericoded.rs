use vstd::prelude::*;
verus! {
enum OpType {
    Add,
    Multiply,
    LogicalAnd,
    LogicalOr,
}
fn ufunc_identity(op: OpType) -> (result: Option<i8>){
    match op {
        OpType::Add => {
            Some(0i8)
        }
        OpType::Multiply => {
            Some(1i8)
        }
        OpType::LogicalAnd => {
            None
        }
        OpType::LogicalOr => {
            None
        }
    }
}
}
fn main() {}