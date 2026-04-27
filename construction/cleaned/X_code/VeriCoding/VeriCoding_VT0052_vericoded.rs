use vstd::prelude::*;
verus! {
pub enum ExpandedVector<T> {
    RowVector(Vec<T>),
    ColumnVector(Vec<T>),
}
fn expand_dims<T>(a: Vec<T>, axis: usize) -> (result: ExpandedVector<T>){
    if axis == 0 {
        ExpandedVector::RowVector(a)
    } else {
        ExpandedVector::ColumnVector(a)
    }
}
}
fn main() {}