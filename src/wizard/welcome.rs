//! Wizard step 1 — Welcome.

use crate::error::FsInitError;
use crate::keys;
use crate::wizard::{prompt, StepResult, WizardState, WizardStep};

pub struct WelcomeStep;

impl WizardStep for WelcomeStep {
    fn title(&self) -> &'static str {
        keys::INIT_STEP_WELCOME_TITLE
    }

    fn run(&self, state: &mut WizardState) -> Result<StepResult, FsInitError> {
        println!();
        println!("{}", keys::INIT_TITLE);
        println!("{}", keys::INIT_STEP_WELCOME_BODY);
        println!();
        println!("  Mode: {}", state.capability.mode.label());
        println!();
        println!("{}", keys::INIT_ABORT_HINT);

        let input = prompt(keys::INIT_PROMPT_CONTINUE)?;
        if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
            return Ok(StepResult::Abort);
        }
        Ok(StepResult::Next)
    }
}
