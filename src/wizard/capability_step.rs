//! Wizard step 2 — Capability display.
//!
//! Shows what fs-info detected. Purely informational — no user choice.

use crate::error::FsInitError;
use crate::keys;
use crate::wizard::{prompt, StepResult, WizardState, WizardStep};

pub struct CapabilityStep;

impl WizardStep for CapabilityStep {
    fn title(&self) -> &'static str {
        keys::INIT_STEP_CAPABILITY_TITLE
    }

    fn run(&self, state: &mut WizardState) -> Result<StepResult, FsInitError> {
        let cap = &state.capability;
        println!();
        println!("{}{}", keys::INIT_CAPABILITY_OS, cap.os.version);
        println!("{}{}", keys::INIT_CAPABILITY_ARCH, cap.os.arch);
        println!("  Display:         {}", cap.display.label());
        println!(
            "  Terminal:        {}",
            if cap.has_terminal { "yes" } else { "no" }
        );
        println!("  Container:       {}", cap.container.label());
        println!("  Mode:            {}", cap.mode.label());
        println!();

        let input = prompt(keys::INIT_PROMPT_CONTINUE)?;
        match input.to_lowercase().as_str() {
            "b" | "back" => Ok(StepResult::Back),
            "q" | "quit" => Ok(StepResult::Abort),
            _ => Ok(StepResult::Next),
        }
    }
}
