use clap::Parser;
use colored::Colorize;
use gitlab_auditor::cli::Args;
use gitlab_auditor::cli::print_banner;
use gitlab_auditor::cli::return_args;
use gitlab_auditor::scans::full::fetch_groups;
use gitlab_auditor::scans::full::fetch_pipelines_from_projects;
use gitlab_auditor::scans::full::fetch_projects_from_groups;

#[tokio::main]
async fn main() {
    print_banner();
    let args = return_args(Args::parse());

    println!("{}", "Current configuration:".bold());
    println!("  GitLab Token: {:.13}...(Masked)", args.gitlab_token);
    println!("  Instance URL: {}", args.instance_url);
    println!("  Scan type: {:?}", args.scan_type);

    let groups = match fetch_groups(&args.gitlab_token, &args.instance_url).await {
        Ok(groups) => groups,
        Err(e) => {
            println!("Error fetching groups: {:?}", e);
            return;
        }
    };

    let projects =
        match fetch_projects_from_groups(&args.gitlab_token, &args.instance_url, &groups).await {
            Ok(projects) => projects,
            Err(e) => {
                println!("Error fetching projects: {:?}", e);
                return;
            }
        };

    let pipelines = match fetch_pipelines_from_projects(
        &args.gitlab_token,
        &args.instance_url,
        &projects,
    )
    .await
    {
        Ok(pipelines) => pipelines,
        Err(e) => {
            println!("Error fetching pipelines: {:?}", e);
            return;
        }
    };
}
