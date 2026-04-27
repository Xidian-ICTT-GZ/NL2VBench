use vstd::prelude::*;
verus! {
pub enum LinAlgError {
    NonConvergence(String),
    SingularMatrix(String),
    NonSquareMatrix(String),
    IncompatibleDimensions(String),
    InvalidInput(String),
    NumericalInstability(String),
    Other(String),
}
fn select_error(error_variant: u8, message: String) -> LinAlgError {
    match error_variant {
        0 => LinAlgError::NonConvergence(message),
        1 => LinAlgError::SingularMatrix(message),
        2 => LinAlgError::NonSquareMatrix(message),
        3 => LinAlgError::IncompatibleDimensions(message),
        4 => LinAlgError::InvalidInput(message),
        5 => LinAlgError::NumericalInstability(message),
        _ => LinAlgError::Other(message),
    }
}
fn check_lin_alg_error(condition: bool, error_variant: u8, message: String) -> (result: Option<LinAlgError>){
    if condition {
        let err = select_error(error_variant, message);
        Some(err)
    } else {
        None
    }
}
}
fn main() {}