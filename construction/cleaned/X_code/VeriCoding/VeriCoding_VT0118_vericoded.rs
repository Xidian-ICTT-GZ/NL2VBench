use vstd::prelude::*;
verus! {
pub struct DType {
    pub kind: &'static str,
    pub itemsize: usize,
    pub alignment: usize,
    pub name: &'static str,
    pub signed: bool,
}
fn numpy_dtype(type_spec: &str) -> (result: DType){
    match type_spec {
        "int8" => DType {
            kind: "i",
            itemsize: 1,
            alignment: 1,
            name: "int8",
            signed: true,
        },
        "int16" => DType {
            kind: "i",
            itemsize: 2,
            alignment: 2,
            name: "int16",
            signed: true,
        },
        "int32" => DType {
            kind: "i",
            itemsize: 4,
            alignment: 4,
            name: "int32",
            signed: true,
        },
        "int64" => DType {
            kind: "i",
            itemsize: 8,
            alignment: 8,
            name: "int64",
            signed: true,
        },
        "float32" => DType {
            kind: "f",
            itemsize: 4,
            alignment: 4,
            name: "float32",
            signed: false,
        },
        "float64" => DType {
            kind: "f",
            itemsize: 8,
            alignment: 8,
            name: "float64",
            signed: false,
        },
        "bool" => DType {
            kind: "b",
            itemsize: 1,
            alignment: 1,
            name: "bool",
            signed: false,
        },
        _ => {
            DType {
                kind: "i",
                itemsize: 1,
                alignment: 1,
                name: "unreachable",
                signed: true,
            }
        }
    }
}
}
fn main() {}