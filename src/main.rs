use std::env;

use extract::Extract;
use fibbonacci::Fibonacci;
use num_bigint::BigInt;

#[tokio::main]
 async  fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Enter exactly two arguments.Invalid input!!!");
    } else {
        let argument_1 = &args[1];
        let argument_2 = &args[2];
        println!("Hello, world!");
        println!("{}", argument_1);
        println!("{}", argument_2);
    }

    // let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string());
    // let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());

//let value =  octocrab::instance().pulls("freshystar", "fibbot").list_files(1).await?;

    let var: &str  = "h3110 23 cat 444 45 bf5  rabbit 11 2 dog";
    let extract_var = Extract::from(var);
    let mut number_fib: Vec<BigInt> = Vec::new();
    for i in extract_var {
        let num = Fibonacci::fibo(i.into());
        number_fib.push(num);
    }
    println!("{:?}", number_fib);
    Ok(())

}

#[cfg(test)]
mod tests { 
    use crate::{extract::Extract, fibbonacci::Fibonacci};

    use super::*;
    use num_bigint::ToBigInt;

    #[test]
    fn test_fib() {
        let num_1 = 30.to_bigint().unwrap();
        let fib_num_1 = 832040.to_bigint().unwrap();
        assert_eq!(Fibonacci::fibo(num_1), fib_num_1);
        
        let num_2 = 50.to_bigint().unwrap();
        let fib_num_2: u64 = 12586269025;
        let fib_num_2 = fib_num_2.to_bigint().unwrap();
        assert_eq!(Fibonacci::fibo(num_2), fib_num_2);
        
        let num_3 = 100.to_bigint().unwrap();
        let fib_num_3: u128 = 354224848179261915075;
        let fib_num_3 = fib_num_3.to_bigint().unwrap();
        assert_eq!(Fibonacci::fibo(num_3), fib_num_3);
    }
    
    #[test]
    fn test_extract_numbers(){
        assert_eq!(Extract::from("pr_ d 888 escription 888 67 4b 66"), [888, 888, 67, 66]);
        assert_eq!(Extract::from(" "), [ ]);
    }
}
mod fibbonacci;
mod extract;
mod get_from;
mod pull;