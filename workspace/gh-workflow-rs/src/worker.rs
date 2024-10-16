use derive_setters::Setters;

use crate::error::{Error, Result};
use crate::Workflow;

#[derive(Setters, Debug)]
pub struct Worker {
    workflow: Workflow,
    file: String,
}

impl Worker {
    pub fn new(workflow: Workflow) -> Self {
        Self {
            workflow,
            file: "./.github/workflows/ci.yml".to_string(),
        }
    }

    fn modify(&self, workflow: Workflow) -> Workflow {
        workflow
    }

    pub fn generate(&self) -> Result<String> {
        let workflow = self.modify(self.workflow.clone());
        Ok(serde_yaml::to_string(&workflow)?)
    }

    pub async fn compare(&self, actual: Workflow) -> Result<()> {
        let expected = self.generate()?;
        let actual = serde_yaml::to_string(&actual)?;

        if actual != expected {
            Err(Error::GitHubWorkflowMismatch)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Event, EventConfig, Job, Runner, Step, Workflow};
    use insta::assert_snapshot;
    use tokio;

    fn create_test_workflow() -> Workflow {
        Workflow {
            name: Some("Test Workflow".to_string()),
            on: vec![EventConfig {
                event: Event::Push,
                branches: Some(vec!["main".to_string()]),
                types: None,
                tags: None,
                paths: None,
                branches_ignore: None,
                tags_ignore: None,
                paths_ignore: None,
            }],
            jobs: {
                let mut jobs = std::collections::HashMap::new();
                jobs.insert(
                    "build".to_string(),
                    Job {
                        runs_on: vec![Runner::Windows],
                        steps: vec![Step::new("echo Hello World".to_string())],
                        name: None,
                        container: None,
                        needs: None,
                        permissions: None,
                        strategy: None,
                        environment: None,
                        outputs: None,
                        concurrency: None,
                        timeout_minutes: None,
                        if_condition: None,
                        services: None,
                        secrets: None,
                        defaults: None,
                        env: None,
                        continue_on_error: None,
                        retry: None,
                        artifacts: None,
                    },
                );
                jobs
            },
            ..Default::default()
        }
    }

    #[test]
    fn test_worker_new() {
        let workflow = create_test_workflow();
        let worker = Worker::new(workflow.clone());
        let generated = worker.generate().unwrap();
        assert_snapshot!(generated);
    }

    #[test]
    fn test_worker_generate() {
        let workflow = create_test_workflow();
        let worker = Worker::new(workflow.clone());
        let generated = worker.generate().unwrap();
        assert_snapshot!(generated);
    }

    #[tokio::test]
    async fn test_worker_compare_mismatch() {
        let workflow_1 = create_test_workflow();

        let mut workflow_2 = workflow_1.clone();
        workflow_2.name = Some("Different Workflow".to_string());

        let worker = Worker::new(workflow_1.clone());
        let result = worker.compare(workflow_2).await;
        assert!(result.is_err());
    }
}
