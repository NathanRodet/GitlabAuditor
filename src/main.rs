use clap::Parser;
use gitlab_auditor::cli::Args;
use gitlab_auditor::cli::ArgsValidation;

fn main() {
    let args = Args::parse();

    let gitlab_token = args
        .gitlab_token()
        .map_err(|e| e)
        .expect("Invalid Gitlab token");
    let instance_url = args
        .instance_url()
        .map_err(|e| e)
        .expect("Invalid Gitlab instance URL");

    println!("Token: {}", gitlab_token);
    println!("Instance URL: {}", instance_url);
}
