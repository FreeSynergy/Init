//! Wizard step 7 — Done.

use crate::error::FsInitError;
use crate::keys;
use crate::store_clone;
use crate::wizard::{StepResult, WizardState, WizardStep};

pub struct DoneStep;

impl WizardStep for DoneStep {
    fn title(&self) -> &'static str {
        keys::INIT_STEP_DONE_TITLE
    }

    fn run(&self, _state: &mut WizardState) -> Result<StepResult, FsInitError> {
        println!();
        println!("{}", keys::INIT_STEP_DONE_BODY);
        println!();
        print!("{}", keys::INIT_STEP_DONE_STORE_PATH);
        println!("{}", store_clone::default_store_dir().display());
        println!();
        Ok(StepResult::Next)
    }
}
