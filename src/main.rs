use clap::Parser;
use gitlab_auditor::cli::Args;
use gitlab_auditor::cli::return_args;
use gitlab_auditor::scans::group::fetch_groups;

#[tokio::main]
async fn main() {
    let args = return_args(Args::parse());

    println!("GitLab Token: {}", args.gitlab_token);
    println!("Instance URL: {}", args.instance_url);
    println!("Scan type: {:?}", args.scan_type);

    let groups = fetch_groups(&args.gitlab_token, &args.instance_url).await;
    println!("Groups: {:?}", groups);
}
