use std::collections::HashMap;

use indexmap::IndexMap;

use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{Error, Result};

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Event {
    BranchProtectionRule,
    CheckRun,
    CheckSuite,
    Create,
    Delete,
    Deployment,
    DeploymentStatus,
    Discussion,
    DiscussionComment,
    Fork,
    Gollum,
    IssueComment,
    Issues,
    Label,
    MergeGroup,
    Milestone,
    PageBuild,
    Project,
    ProjectCard,
    ProjectColumn,
    Public,
    PullRequest,
    PullRequestReview,
    PullRequestReviewComment,
    PullRequestTarget,
    #[default]
    Push,
    RegistryPackage,
    Release,
    Status,
    Watch,
    WorkflowCall,
    WorkflowDispatch,
    WorkflowRun,
    RepositoryDispatch,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Workflow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<WorkflowOn>,
    pub jobs: IndexMap<String, Job>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<IndexMap<String, Secret>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum WorkflowOn {
    // TODO: use type-safe enum instead of string
    Single(String),
    // TODO: use type-safe enum instead of string
    Multiple(Vec<String>),
    // TODO: use type-safe enum instead of string
    Map(IndexMap<String, IndexMap<String, Vec<String>>>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct EventAction {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    branches: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    branches_ignore: Vec<String>,
}

impl Workflow {
    pub fn new(name: String) -> Self {
        Self {
            name: Some(name),
            on: Default::default(),
            run_name: Default::default(),
            jobs: Default::default(),
            concurrency: Default::default(),
            defaults: Default::default(),
            secrets: Default::default(),
            env: Default::default(),
            timeout_minutes: Default::default(),
        }
    }
    pub fn to_string(&self) -> Result<String> {
        let yaml_output = serde_yaml::to_string(self)?;
        // Normalize line endings to Unix-style before returning the string
        let normalized_output = yaml_output.replace("\r\n", "\n").trim_end().to_string();
        Ok(normalized_output)
    }

    pub fn add_job(mut self, id: String, job: crate::Job) -> Result<Self> {
        if self.jobs.contains_key(&id) {
            return Err(Error::JobIdAlreadyExists(id));
        }

        self.jobs.insert(id, job);
        Ok(self)
    }

    pub fn parse(yml: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(yml)?)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityType {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Default, Setters, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct EventConfig {
    pub event: Event,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<ActivityType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_ignore: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_ignore: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths_ignore: Option<Vec<String>>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Job {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs_on: Option<JobRunsOn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<Step>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_condition: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<IndexMap<String, Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<IndexMap<String, Secret>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum JobRunsOn {
    Single(String),
    Multiple(Vec<String>),
    KeyValue(IndexMap<String, String>),
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Step {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<IndexMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_condition: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
}

impl Step {
    pub fn new(step: String) -> Self {
        Self { run: Some(step), name: None, ..Default::default() }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Runner {
    #[default]
    Linux,
    MacOS,
    Windows,
    Custom(String),
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Container {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Port>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Port {
    Number(u16),
    Name(String),
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Volume {
    pub source: String,
    pub destination: String,
}

impl Volume {
    pub fn new(volume_str: &str) -> Option<Self> {
        let parts: Vec<&str> = volume_str.split(':').collect();
        if parts.len() == 2 {
            Some(Volume {
                source: parts[0].to_string(),
                destination: parts[1].to_string(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Concurrency {
    pub group: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_in_progress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Permissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<PermissionLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_specific: Option<IndexMap<Event, PermissionLevel>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionLevel {
    #[default]
    Read,
    Write,
    None,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Strategy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<IndexMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel: Option<u32>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Environment {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Defaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<RunDefaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryDefaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct RunDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct RetryDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Expression {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed: Option<ParsedExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result: Option<bool>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct ParsedExpression {
    pub variables: Vec<String>, // Represents variables used within the expression
    pub functions: Vec<String>, // Represents functions or operators used within the expression
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Secret {
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct RetryStrategy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Artifacts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Vec<Artifact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<Vec<Artifact>>,
}

#[derive(Debug, Setters, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option)]
pub struct Artifact {
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<u32>,
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    fn split(content: &str) -> (String, String) {
        let parsed = Workflow::parse(content).unwrap();
        let actual = parsed.to_string().unwrap();
        // Normalize line endings to Unix format
        let expected = content.replace("\r\n", "\n").trim_end().to_string();
        (actual, expected)
    }

    #[test]
    fn test_workflow_bench() {
        let (actual, expected) = split(include_str!("./fixtures/workflow-bench.yml"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_workflow_ci() {
        let (actual, expected) = split(include_str!("./fixtures/workflow-ci.yml"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_workflow_demo() {
        let (actual, expected) = split(include_str!("./fixtures/workflow-demo.yml"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_workflow_rust() {
        let (actual, expected) = split(include_str!("./fixtures/workflow-rust.yml"));
        assert_eq!(actual, expected);
    }
}
