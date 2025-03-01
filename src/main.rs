use std::env;

use dotenv::dotenv;
use extract::Extract;
//use extract::Extract;
use fibbonacci::Fibonacci;
use get_pull::ExtractNumbers;
// use get_pull::GettingPr;
use num_bigint::BigInt;
use pull::PullRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args: Vec<String> = env::args().collect();
    // if args.len() != 3 {
    //     println!("Enter exactly two arguments.Invalid input!!!");
    // } else {
    //     let argument_1 = &args[1];
    //     let argument_2 = &args[2];
    //     println!("Hello, world!");
    //     // println!("{}", argument_1);
    //     // println!("{}", argument_2);
    // }
    let _ = dotenv().is_ok();
   // let github_token = env::var("GITHUB_TOKEN").unwrap_or_else(|_| "".to_string());
    let github_repository =
        env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "freshystar/fibbot".to_string());
    let github_repository = github_repository.split("/").collect::<Vec<&str>>();
    let owner = github_repository[0];
    let repo = github_repository[1];

    let pr_number = env::var("PR_number")
        .unwrap_or_else(|_| "2".to_string())
        .parse::<u64>()
        .expect("Invalid PR_number");

    // let value = octocrab::instance()
    //     .pulls("freshystar", "fibbot")
    //     .list_files(1)
    //     .await?;

    //let value = &value.items.first().unwrap().patch.clone().unwrap();

    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string());

    
    if enable_fib.eq("true") {
        println!("fibbot enabled...");

        let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());
        println!("Max-threshold: {} ", max_threshold);

        let pull_request_file = pull::PullRequest::get_pr(&owner, &repo, pr_number).await?;

        for file in pull_request_file {
            if let Some(patch) = file.patch {
                println!("{:?}", Extract::from(&patch));

                let extracted_numbers = Extract::from(&patch);

                let fib_of_extracted_numbers: Vec<BigInt> = extracted_numbers.clone()
                    .into_iter()
                    .filter(|x| x < &max_threshold.parse::<u128>().unwrap())
                    .map(|x| Fibonacci::fibo(x.into()))
                    .collect();

                let pr_file_name = &file.filename;

                println!(
                    "fibonacci of found numbers {:?} in {} is: {:?}",
                    extracted_numbers, pr_file_name, fib_of_extracted_numbers
                );

                
                let comment_body = format!("The fibonacci of {:?} is: {:?}", extracted_numbers, fib_of_extracted_numbers);

                PullRequest::post_comment_to_pr(
                    owner,
                    repo,
                    comment_body.as_str(),
                    pr_number,
                )
                .await?;
            }
        }
    } else {
        println!("Fibbot disabled...");
    }

    // let pr = GettingPr::get_pr_body(1);
    // println!("{:?}", pr);

    Ok(())
}



mod extract;
mod fibbonacci;
mod get_pull;
mod pull;
