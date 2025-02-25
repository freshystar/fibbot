use std::env;

fn main() {
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
}
