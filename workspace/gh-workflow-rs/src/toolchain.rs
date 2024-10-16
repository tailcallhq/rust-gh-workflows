use derive_setters::Setters;

use crate::schema::*;

pub enum Version {
    Stable,
    Beta,
    Nightly,
}

#[derive(Setters)]
pub struct RustToolchain {
    version: Version,
    rust_fmt: bool,
    rust_clippy: bool,
    // TODO: add more rust tool chain components
}

impl RustToolchain {
    pub fn to_job(&self) -> Job {
        Job {
            name: Some(format!("Setup Rust Toolchain")),
            runs_on: vec![Runner::default()],
            steps: vec![

                // TODO: expose as typed methods
                Step::default().uses("actions/checkout@v2".to_string()),
                Step::default().uses("actions-rs/toolchain@v1".to_string()),

                //
            ],
            ..Default::default()
        }
    }
}
