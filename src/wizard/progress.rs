//! Wizard step 6 — Installation progress.
//!
//! Phase 1: clones the Store catalog. The full install pipeline (Phase 2)
//! will expand this step to download and install packages.

use crate::error::FsInitError;
use crate::keys;
use crate::store_clone;
use crate::wizard::{StepResult, WizardState, WizardStep};

pub struct ProgressStep;

impl WizardStep for ProgressStep {
    fn title(&self) -> &'static str {
        keys::INIT_STEP_PROGRESS_TITLE
    }

    fn run(&self, _state: &mut WizardState) -> Result<StepResult, FsInitError> {
        println!();
        clone_store_if_needed()?;
        println!();
        println!("{}", keys::INIT_STEP_PROGRESS_INSTALL_PENDING);
        Ok(StepResult::Next)
    }
}

fn clone_store_if_needed() -> Result<(), FsInitError> {
    let target = store_clone::default_store_dir();
    if target.exists() {
        println!("{}", keys::INIT_STEP_PROGRESS_CLONE_EXISTS);
        return Ok(());
    }
    println!("{}", keys::INIT_STEP_PROGRESS_CLONING_STORE);
    store_clone::clone_store(
        store_clone::DEFAULT_STORE_URL,
        store_clone::DEFAULT_BRANCH,
        &target,
    )?;
    println!("{}", keys::INIT_STEP_PROGRESS_CLONE_OK);
    Ok(())
}
