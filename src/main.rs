use std::env;

use dotenv::dotenv;
use extract::Extract;
use fibbonacci::Fibonacci;
// use get_pull::GettingPr;
use num_bigint::BigInt;
use pull::PullRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Enter exactly two arguments.Invalid input!!!");
    } else {
        let argument_1 = &args[1];
        let argument_2 = &args[2];
        println!("Hello, world!");
        // println!("{}", argument_1);
        // println!("{}", argument_2);
    }

    //let _ = dotenv().is_ok();
    let github_token = env::var("GITHUB_TOKEN").unwrap_or_else(|_| "".to_string());
    let github_repository =
        env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "freshystar/fibbot".to_string());
    let github_repository = github_repository.split("/").collect::<Vec<&str>>();
    let owner = github_repository[0];
    let repo = github_repository[1];

    let value = octocrab::instance()
        .pulls("freshystar", "fibbot")
        .list_files(1)
        .await?;

    let value = &value.items.first().unwrap().patch.clone().unwrap();

    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());

    //println!("{:?}", value);

    //let var: &str = "h3110 23 cat 444 45 bf5  rabbit 11 2 dog";

    if enable_fib.eq("true") {
        println!("fibbot enabled...");
        let extract_var = Extract::from(value);
        let mut number_fib: Vec<BigInt> = Vec::new();
        for i in extract_var {
            let num = Fibonacci::fibo(i.into());
            number_fib.push(num);
        }
        println!("{:?}", number_fib);
    } else {
        println!("Fibbot disabled...");
    }

    // let pr = GettingPr::get_pr_body(1);
    // println!("{:?}", pr);

    Ok(())
}



mod extract;
mod fibbonacci;
mod pull;
mod get_pull;
