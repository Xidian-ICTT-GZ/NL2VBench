use vstd::prelude::*;
verus! {
fn calculate_loss(cost_price: i32, selling_price: i32) -> (loss: i32){
  if cost_price > selling_price {
    let d = cost_price - selling_price;
    d
  } else {
    0
  }
}
fn main() {
}
}