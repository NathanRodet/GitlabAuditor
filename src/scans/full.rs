use colored::Colorize;
use regex::Regex;
use url::Url;

use crate::scans::shared::Job;

use super::shared::{Group, MIN_ACCESS_LEVEL_GUEST, PRIVATE_TOKEN_HEADER, Project};

pub async fn fetch_groups(token: &str, url: &Url) -> Result<Vec<Group>, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut groups = Vec::new();
    let mut page = 1;

    loop {
        let response = client
            .get(format!("{}/groups", url))
            .header(PRIVATE_TOKEN_HEADER, token)
            .query(&[
                ("all_available", "true"),
                ("per_page", "100"),
                ("page", &page.to_string()),
                ("min_access_level", MIN_ACCESS_LEVEL_GUEST),
                ("include_subgroups", "true"),
            ])
            .send()
            .await
            .map_err(|e| e)?;

        if let Err(err) = response.error_for_status_ref() {
            return Err(err);
        }

        let next_page = response
            .headers()
            .get("x-next-page")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let mut page_groups: Vec<Group> = response.json().await.map_err(|e| e)?;
        groups.append(&mut page_groups);

        match next_page {
            Some(next) if page_groups.len() >= 100 => {
                page = next;
            }
            _ => break,
        }
    }

    println!(
        "{}",
        format!(
            "Fetched {} groups: {:?}.",
            groups.len(),
            groups.iter().map(|g| &g.name).collect::<Vec<_>>()
        )
        .blue()
        .bold()
    );

    Ok(groups)
}

pub async fn fetch_projects_from_groups(
    token: &str,
    url: &Url,
    groups: &Vec<Group>,
) -> Result<Vec<Project>, reqwest::Error> {
    let mut all_projects = Vec::new();

    let futures = groups
        .iter()
        .map(|group| fetch_projects_for_single_group(token, url, group));

    let results = futures::future::join_all(futures).await;

    for result in results {
        match result {
            Ok(mut projects) => all_projects.append(&mut projects),
            Err(e) => return Err(e),
        }
    }

    println!(
        "{}",
        format!(
            "Fetched {} projects across all groups: {:?}.",
            all_projects.len(),
            all_projects.iter().map(|g| &g.name).collect::<Vec<_>>()
        )
        .blue()
        .bold()
    );

    Ok(all_projects)
}

async fn fetch_projects_for_single_group(
    token: &str,
    url: &Url,
    group: &Group,
) -> Result<Vec<Project>, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut projects = Vec::new();
    let mut page = 1;

    loop {
        let response = client
            .get(format!("{}/groups/{}/projects", url, group.id))
            .header(PRIVATE_TOKEN_HEADER, token)
            .query(&[
                ("all_available", "true"),
                ("per_page", "100"),
                ("page", &page.to_string()),
                ("min_access_level", MIN_ACCESS_LEVEL_GUEST),
                ("include_subgroups", "true"),
            ])
            .send()
            .await
            .map_err(|e| e)?;

        if let Err(err) = response.error_for_status_ref() {
            return Err(err);
        }

        let next_page = response
            .headers()
            .get("x-next-page")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let mut page_projects: Vec<Project> = response.json().await.map_err(|e| e)?;
        projects.append(&mut page_projects);

        match next_page {
            Some(next) if page_projects.len() >= 100 => {
                page = next;
            }
            _ => break,
        }
    }

    println!(
        "{}",
        format!(
            "   Fetched {} projects for group: {}, id: {}.",
            projects.len(),
            group.name,
            group.id
        )
        .blue()
    );

    Ok(projects)
}

pub async fn fetch_jobs_for_single_project(
    token: &str,
    url: &Url,
    project: &Project,
) -> Result<Vec<Job>, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut jobs = Vec::new();
    let mut page = 1;

    loop {
        let response = client
            .get(format!("{}/projects/{}/jobs", url, project.id))
            .header(PRIVATE_TOKEN_HEADER, token)
            .query(&[
                ("per_page", "100"),
                ("page", &page.to_string()),
                ("scope[]", "success"),
                ("scope[]", "failed"),
                ("scope[]", "canceled"),
            ])
            .send()
            .await
            .map_err(|e| e)?;

        if let Err(err) = response.error_for_status_ref() {
            return Err(err);
        }

        let next_page = response
            .headers()
            .get("x-next-page")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let mut page_jobs: Vec<Job> = response.json().await.map_err(|e| e)?;
        jobs.append(&mut page_jobs);

        match next_page {
            Some(next) if page_jobs.len() >= 100 => {
                page = next;
            }
            _ => break,
        }
    }

    println!(
        "{}",
        format!(
            "   Fetched {} jobs for project: {}, id: {}.",
            jobs.len(),
            project.name,
            project.id
        )
        .blue()
    );

    Ok(jobs)
}

pub async fn fetch_job_traces_for_projects(
    token: &str,
    url: &Url,
    projects: &[Project],
) -> Result<(), reqwest::Error> {
    let futures = projects
        .iter()
        .map(|project| fetch_jobs_for_single_project(token, url, project));

    let results = futures::future::join_all(futures).await;

    println!("{}", "Clearing old log traces...".bold().blue());
    if std::path::Path::new("results/log_traces").exists() {
        std::fs::remove_dir_all("results/log_traces").expect("Failed to clear old log traces");
    }

    println!("{}", "Starting Fetching job traces...".bold().blue());

    for (project, result) in projects.iter().zip(results) {
        match result {
            Ok(jobs) => fetch_job_traces_for_single_project(token, url, project, &jobs).await?,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

async fn fetch_job_traces_for_single_project(
    token: &str,
    url: &Url,
    project: &Project,
    jobs: &Vec<Job>,
) -> Result<(), reqwest::Error> {
    let project_dir = format!("results/log_traces/{}", project.name.replace("/", "_"));
    std::fs::create_dir_all(&project_dir).expect("Failed to create project directory");

    let mut success_count = 0;

    for job in jobs {
        match fetch_job_trace(token, url, project.id, job.id).await {
            Ok(trace) => {
                let clean_trace = clean_ansi_codes(&trace);
                let filename = format!("{}/{}.txt", project_dir, job.id);

                if let Err(e) = std::fs::write(&filename, &clean_trace) {
                    println!(
                        "{}",
                        format!("   Failed to write trace for job {} to file: {}", job.id, e).red()
                    );
                } else {
                    success_count += 1;
                    let percentage = (success_count as f32 / jobs.len() as f32 * 100.0) as usize;
                    let completed_bars = percentage / 4; // 25 total bars for 100%
                    let bar = "■".repeat(completed_bars) + &"□".repeat(25 - completed_bars);

                    print!(
                        "\r{}",
                        format!(
                            "   Saved {}/{} traces for project: {} [{}] {}%",
                            success_count,
                            jobs.len(),
                            project.name,
                            bar,
                            percentage
                        )
                        .blue()
                    );
                }
            }
            Err(e) => {
                println!(
                    "{}",
                    format!("   Failed to fetch trace for job {}: {}", job.id, e).red()
                );
            }
        }
    }

    Ok(())
}

fn clean_ansi_codes(trace: &str) -> String {
    // Supported Regex to match ANSI escape sequences: https://gitlab.com/gitlab-com/support/toolbox/dotfiles/-/blob/main/aliases/ansi
    let re = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").expect("Invalid regex pattern");
    re.replace_all(trace, "").to_string()
}

async fn fetch_job_trace(
    token: &str,
    url: &Url,
    project_id: u64,
    job_id: u64,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "{}/projects/{}/jobs/{}/trace",
            url, project_id, job_id
        ))
        .header(PRIVATE_TOKEN_HEADER, token)
        .send()
        .await?;

    response.error_for_status()?.text().await
}
