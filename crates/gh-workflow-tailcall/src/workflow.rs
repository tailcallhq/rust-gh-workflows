//! Workflow is designed to be used for most Rust projects that are built at
//! Tailcall. Though gh-workflow makes it much easier to write workflows you
//! still need to constantly keep referring to the Github documentation to write
//! your own workflows. This module saves all that time by using feature flags
//! to enable or disable features that you want in your workflow. Based on the
//! features enabled or disabled a workflow is generated.

use ctx::Context;
use derive_setters::Setters;
use gh_workflow::error::Result;
use gh_workflow::{Workflow as GHWorkflow, *};
use release_plz::{Command, Release};
use toolchain::Toolchain;

#[derive(Debug, Clone, Setters)]
pub struct Workflow {
    /// When enabled, a release job is added to the workflow.
    /// *IMPORTANT:* Ensure `secrets.CARGO_REGISTRY_TOKEN` is set for your
    /// github action.
    pub auto_release: bool,

    /// Name of the workflow.
    pub name: String,

    /// When enabled, a benchmark job is added to the workflow.
    pub benchmarks: bool,

    /// When enabled, auto-commits lint and fmt fixes on PRs.
    pub auto_fix: bool,
}

impl Default for Workflow {
    fn default() -> Self {
        Self {
            auto_release: false,
            name: "CI".into(),
            benchmarks: false,
            auto_fix: false,
        }
    }
}

impl Workflow {
    /// Generates and tests the workflow file.
    pub fn generate(self) -> Result<()> {
        let workflow: GHWorkflow = self.into();
        workflow.generate()
    }

    /// Converts the workflow into a Github workflow.
    pub fn to_github_workflow(&self) -> GHWorkflow {
        self.clone().into()
    }

    /// Creates the "Build and Test" job for the workflow.
    pub fn build_and_test(&self) -> Job {
        let mut job = Job::new("Build and Test")
            .permissions(Permissions::default().contents(Level::Read))
            .add_step(Step::checkout())
            .add_step(
                Toolchain::default()
                    .add_stable()
                    .add_nightly()
                    .add_clippy()
                    .add_fmt(),
            )
            .add_step(
                Cargo::new("test")
                    .args("--all-features --workspace")
                    .name("Cargo Test"),
            )
            .add_step(
                Cargo::new("fmt")
                    .nightly()
                    .args("--check")
                    .name("Cargo Fmt"),
            )
            .add_step(
                Cargo::new("clippy")
                    .nightly()
                    .args("--all-features --workspace -- -D warnings")
                    .name("Cargo Clippy"),
            );

        if self.benchmarks {
            job = job.add_step(Cargo::new("bench").args("--workspace").name("Cargo Bench"));
        }

        job
    }
}

impl From<Workflow> for GHWorkflow {
    fn from(value: Workflow) -> Self {
        let flags = RustFlags::deny("warnings");

        let event = Event::default()
            .push(Push::default().add_branch("main"))
            .pull_request(
                PullRequest::default()
                    .add_type(PullRequestType::Opened)
                    .add_type(PullRequestType::Synchronize)
                    .add_type(PullRequestType::Reopened)
                    .add_branch("main"),
            );

        let is_main = Context::github().ref_().eq("refs/heads/main".into());
        let is_push = Context::github().event_name().eq("push".into());
        let cond = is_main.and(is_push);

        // Jobs
        let build = value.build_and_test();
        let mut workflow = GHWorkflow::new(value.name)
            .add_env(flags)
            .on(event)
            .add_job("build", build.clone());

        if value.auto_release {
            let permissions = Permissions::default()
                .pull_requests(Level::Write)
                .packages(Level::Write)
                .contents(Level::Write);

            let release = release_job(&cond, &build, &permissions);
            let release_pr = release_pr_job(cond, &build, permissions);
            workflow = workflow
                .add_job("release", release)
                .add_job("release-pr", release_pr);
        }

        // Add auto-fix job if enabled
        if value.auto_fix {
            let is_pr = Context::github().event_name().eq("pull_request".into());
            let lint_and_fmt_fix = lint_and_fmt_fix_job();
            workflow = workflow.add_job("auto-fix-lint-fmt", lint_and_fmt_fix.cond(is_pr));
        }

        workflow
    }
}

fn lint_and_fmt_fix_job() -> Job {
    Job::new("Auto Fix Lint and Fmt")
        .permissions(Permissions::default().contents(Level::Write))
        .cond(Context::github().event_name().eq("pull_request".into())) // Ensure it's a PR
        .add_step(Step::checkout())
        .add_step(Toolchain::default().add_stable().add_nightly().add_fmt())
        .add_step(
            Cargo::new("fmt")
                .nightly()
                .args("") // Run cargo fmt (without --check to fix)
                .name("Cargo Fmt (Fix)"),
        )
        .add_step(Step::run(
            r#"
            git config user.name "github-actions[bot]"
            git config user.email "github-actions[bot]@users.noreply.github.com"
            git add .
            git commit -m "style: Applied automatic formatting fixes via gh-workflow-tailcall"
            git push
        "#,
        ))
}

fn release_pr_job(cond: Context<bool>, build: &Job, permissions: Permissions) -> Job {
    Job::new("Release PR")
        .cond(cond.clone())
        .concurrency(
            Concurrency::new(Expression::new("release-${{github.ref}}")).cancel_in_progress(false),
        )
        .add_needs(build.clone())
        .add_env(Env::github())
        .add_env(Env::new(
            "CARGO_REGISTRY_TOKEN",
            "${{ secrets.CARGO_REGISTRY_TOKEN }}",
        ))
        .permissions(permissions)
        .add_step(Step::checkout())
        .add_step(Release::default().command(Command::ReleasePR))
}

fn release_job(cond: &Context<bool>, build: &Job, permissions: &Permissions) -> Job {
    Job::new("Release")
        .cond(cond.clone())
        .add_needs(build.clone())
        .add_env(Env::github())
        .add_env(Env::new(
            "CARGO_REGISTRY_TOKEN",
            "${{ secrets.CARGO_REGISTRY_TOKEN }}",
        ))
        .permissions(permissions.clone())
        .add_step(Step::checkout())
        .add_step(Release::default().command(Command::Release))
}
