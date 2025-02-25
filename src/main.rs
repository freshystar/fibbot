use std::env;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().skip(1).collect();
     
     let argument_1 = &args[0];
     let argument_2 = &args[1];

     println!("{}", argument_1);
     println!("{}", argument_2);

    
}
