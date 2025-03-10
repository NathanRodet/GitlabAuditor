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
    pub project_id: u64,
    pub branch_ref: String,
    pub status: String,
    // Full response format: https://docs.gitlab.com/api/pipelines/#list-project-pipelines
}

impl std::fmt::Display for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pipeline for project: {} (ID: {}, Branch: {}, Status: {})",
            self.project_id, self.id, self.branch_ref, self.status
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Job {
    pub id: u64,
    pub name: String,

    pub web_url: String,
    // Full response format: https://docs.gitlab.com/api/jobs/#list-project-jobs
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Job {} (ID: {})", self.name, self.id)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Artifact {
    pub file_type: String,
    pub size: u64,
    pub filename: String,
    pub file_format: String,
}

impl std::fmt::Display for Artifact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Artifact {} (Size: {})", self.filename, self.size)
    }
}
