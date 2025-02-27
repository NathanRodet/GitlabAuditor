use serde::{Deserialize, Serialize};
use url::Url;

use super::shared::{MIN_ACCESS_LEVEL_GUEST, PRIVATE_TOKEN_HEADER};

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    id: u64,
    name: String,
    // Full response format: https://docs.gitlab.com/ee/api/groups.html#list-groups
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Group {} (ID: {})", self.name, self.id)
    }
}

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
                ("per_page", "1"),
                ("page", &page.to_string()),
                ("statistics", "false"),
                ("min_access_level", MIN_ACCESS_LEVEL_GUEST),
            ])
            .send()
            .await
            .map_err(|e| e)?;

        if !response.status().is_success() {
            return Err(response.error_for_status().unwrap_err());
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

    Ok(groups)
}
