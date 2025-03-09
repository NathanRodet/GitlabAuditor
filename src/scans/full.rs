use colored::Colorize;
use url::Url;

use super::shared::{Group, MIN_ACCESS_LEVEL_GUEST, PRIVATE_TOKEN_HEADER, Pipeline, Project};

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

pub async fn fetch_pipelines_from_projects(
    token: &str,
    url: &Url,
    projects: &Vec<Project>,
) -> Result<Vec<Pipeline>, reqwest::Error> {
    let mut all_pipelines = Vec::new();

    let futures = projects
        .iter()
        .map(|project| fetch_pipelines_for_single_project(token, url, project));

    let results = futures::future::join_all(futures).await;

    for result in results {
        match result {
            Ok(mut pipelines) => all_pipelines.append(&mut pipelines),
            Err(e) => return Err(e),
        }
    }

    println!(
        "{}",
        format!(
            "Fetched {} pipelines across all projects.",
            all_pipelines.len(),
        )
        .blue()
        .bold()
    );

    Ok(all_pipelines)
}

pub async fn fetch_pipelines_for_single_project(
    token: &str,
    url: &Url,
    project: &Project,
) -> Result<Vec<Pipeline>, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut all_pipelines: Vec<Pipeline> = Vec::new();

    let mut project_pipelines = Vec::new();
    let mut page = 1;

    loop {
        let response = client
            .get(format!("{}/projects/{}/pipelines", url, project.id))
            .header(PRIVATE_TOKEN_HEADER, token)
            .query(&[
                ("per_page", "100"),
                ("page", &page.to_string()),
                ("scope", "finished"),
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

        match response.json::<Vec<Pipeline>>().await {
            Ok(mut page_pipelines) => {
                project_pipelines.append(&mut page_pipelines);

                match next_page {
                    Some(next) if page_pipelines.len() >= 100 => {
                        page = next;
                    }
                    _ => break,
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    all_pipelines.extend(project_pipelines);

    println!(
        "{}",
        format!(
            "   Fetched {} pipelines for project {} ID {}",
            all_pipelines.len(),
            project.name,
            project.id
        )
        .blue()
    );

    Ok(all_pipelines)
}
