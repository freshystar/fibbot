use num_bigint::BigInt;

use crate::{extract::extract_nums, fibbonacci::fibo};

pub fn extract_number(content: &str) -> Vec<BigInt> {
    let extract_var = extract_nums(content);
    let mut number_fib: Vec<BigInt> = Vec::new();
    for i in extract_var {
        let num = fibo(i.into());
        number_fib.push(num);
    }
    number_fib
}
