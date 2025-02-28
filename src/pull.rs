pub struct PullRequest;
use octocrab::{models::repos::DiffEntry, Page};
impl PullRequest {
    pub async fn get_pr(owner: &str, repo: &str) -> Result<Page<DiffEntry>, octocrab::Error> {
      octocrab::instance().pulls("freshystar", "fibbot").list_files(1).await
    }
}