use std::env;

use dotenv::dotenv;
use extract::extract_nums;
use fibbonacci::fibo;
use get_pull::extract_number;
use num_bigint::BigInt;
use pull::PullRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string());

    if enable_fib.eq("true") {
        println!("fibbot enabled...");

        let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());
        println!("Max-threshold: {} ", max_threshold);

        let pull_request_file = pull::PullRequest::get_pr(&owner, &repo, pr_number).await;
        let content = pull_request_file
            .unwrap()
            .items
            .first()
            .unwrap()
            .patch
            .clone()
            .unwrap();

        println!("Content:\n{}", content);
        let fib_ext = extract_number(&content);
        println!("{:?}", fib_ext);
        let nums = extract_nums(&content);

        let comment_body = { format!("The fibonacci of {:?} is: {:?}\n", nums, fib_ext) };
        println!("{}", comment_body);
        PullRequest::post_comment_to_pr(owner, repo, comment_body.as_str(), pr_number).await?;
    } else {
        println!("Fibbot disabled...");
    }

    Ok(())
}

mod extract;
mod fibbonacci;
mod get_pull;
mod pull;
