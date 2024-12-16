use wasm_bindgen::prelude::*;
use js_sys::BigInt;
use std::ops::Add;

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> BigInt {
    let mut a = BigInt::from(0u32);
    let mut b = BigInt::from(1u32);

    for _ in 0..n {
        let temp = &a + &b; // Use references to avoid moving ownership
        a = b;
        b = temp;
    }

    a
}
