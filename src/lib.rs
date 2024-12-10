use wasm_bindgen::prelude::*;
use num_bigint::BigUint;
use std::ops::Add;

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> String {
    let mut a = BigUint::from(0u32);
    let mut b = BigUint::from(1u32);

    for _ in 0..n {
        let temp = &a + &b; // Use references to avoid moving ownership
        a = b;
        b = temp;
    }

    a.to_string()
}
