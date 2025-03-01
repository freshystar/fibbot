use num_bigint::{BigInt, ToBigInt};
pub fn fibo(num: BigInt) -> BigInt {
    let mut a: BigInt = 0.to_bigint().unwrap();
    let mut b: BigInt = 1.to_bigint().unwrap();

    let mut i: BigInt = 0.to_bigint().unwrap();
    let num = num.to_bigint().unwrap();
    while i < num {
        let temp = b;
        b = &temp + a;
        a = temp;
        i += 1;
        // vect.push(a);
    }
    // println!("{}", a);
    a
}
