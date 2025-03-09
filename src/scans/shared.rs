pub const PRIVATE_TOKEN_HEADER: &str = "PRIVATE-TOKEN";
pub const MIN_ACCESS_LEVEL_GUEST: &str = "10";

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: u64,
    pub name: String,
    // Full response format: https://docs.gitlab.com/api/groups/#list-projects
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Project {} (ID: {})", self.name, self.id)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    pub id: u64,
    pub name: String,
    // Full response format: https://docs.gitlab.com/ee/api/groups.html#list-groups
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Group {} (ID: {})", self.name, self.id)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pipeline {
    pub id: u64,
    pub iid: u64,
    pub name: String,
    pub web_url: String,
    // Full response format: https://docs.gitlab.com/api/pipelines/#list-project-pipelines
}

impl std::fmt::Display for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pipeline {} (ID: {}, IID: {}, URL: {})",
            self.name, self.id, self.iid, self.web_url
        )
    }
}
