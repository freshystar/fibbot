use num_bigint::BigInt;

use crate::{extract::Extract, fibonnaci::Fibonacci};

#[derive(Clone, Copy)]
pub struct ExtractNumbers;


impl ExtractNumbers {
    pub fn extract_number(content: &str) -> Vec<BigInt> {
        let extract_var = Extract::from(content);
        let mut number_fib: Vec<BigInt> = Vec::new();
        for i in extract_var {
            let num = Fibonacci::fibo(i.into());
            number_fib.push(num);
        }
        println!("{:?}", number_fib);
        number_fib
    }
}


