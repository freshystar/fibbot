
#[cfg(test)]
mod tests {
    use crate::{extract::extract_nums, fibbonacci::fibo};

    use super::*;
    use num_bigint::ToBigInt;

    #[test]
    fn test_fib() {
        let num_1 = 30.to_bigint().unwrap();
        let fib_num_1 = 832040.to_bigint().unwrap();
        assert_eq!(fibo(num_1), fib_num_1);

        let num_2 = 50.to_bigint().unwrap();
        let fib_num_2: u64 = 12586269025;
        let fib_num_2 = fib_num_2.to_bigint().unwrap();
        assert_eq!(fibo(num_2), fib_num_2);

        let num_3 = 100.to_bigint().unwrap();
        let fib_num_3: u128 = 354224848179261915075;
        let fib_num_3 = fib_num_3.to_bigint().unwrap();
        assert_eq!(fibo(num_3), fib_num_3);
    }

    #[test]
    fn test_extract_numbers() {
        assert_eq!(
            extract_nums("pr_ d 888 escription 888 67 4b 66"),
            [888, 888, 67, 4, 66]
        );
        assert_eq!(extract_nums(" "), []);
    }
}

mod extract;
mod fibbonacci;
