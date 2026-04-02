//! Wizard step 4 — Bundle selection.

use crate::capability::BootstrapMode;
use crate::error::FsInitError;
use crate::keys;
use crate::wizard::{prompt, BundleChoice, StepResult, WizardState, WizardStep, BUNDLES};

pub struct BundleStep;

impl WizardStep for BundleStep {
    fn title(&self) -> &'static str {
        keys::INIT_STEP_BUNDLE_TITLE
    }

    fn run(&self, state: &mut WizardState) -> Result<StepResult, FsInitError> {
        let available = available_bundles(state.capability.mode);

        println!();
        println!("{}", keys::INIT_STEP_BUNDLE_PROMPT);
        println!();
        for (i, bundle) in available.iter().enumerate() {
            println!("  [{}] {}", i + 1, bundle.name);
            println!("      {}", bundle.description);
        }
        println!();

        let default_idx = default_bundle_idx(state.capability.mode, &available);
        if let Some(d) = default_idx {
            println!("  Default: [{}] {}", d + 1, available[d].name);
        }
        println!("  [b] Back   [q] Quit");
        println!();

        loop {
            let input = prompt(keys::INIT_PROMPT_CHOICE)?;
            match input.as_str() {
                "b" | "back" => return Ok(StepResult::Back),
                "q" | "quit" => return Ok(StepResult::Abort),
                "" => {
                    if let Some(d) = default_idx {
                        state.selected_bundle = Some((*available[d]).clone());
                        return Ok(StepResult::Next);
                    }
                    println!("{}", keys::INIT_INVALID_CHOICE);
                }
                s => match parse_choice(s, available.len()) {
                    Some(idx) => {
                        state.selected_bundle = Some((*available[idx]).clone());
                        return Ok(StepResult::Next);
                    }
                    None => println!("{}", keys::INIT_INVALID_CHOICE),
                },
            }
        }
    }
}

fn available_bundles(mode: BootstrapMode) -> Vec<&'static BundleChoice> {
    BUNDLES
        .iter()
        .filter(|b| mode == BootstrapMode::Gui || !b.requires_display)
        .collect()
}

fn default_bundle_idx(mode: BootstrapMode, available: &[&BundleChoice]) -> Option<usize> {
    let preferred = match mode {
        BootstrapMode::Gui => "workstation",
        BootstrapMode::Tui | BootstrapMode::Headless => "server",
    };
    available.iter().position(|b| b.id == preferred)
}

fn parse_choice(s: &str, len: usize) -> Option<usize> {
    s.parse::<usize>().ok().and_then(|n| {
        if n >= 1 && n <= len {
            Some(n - 1)
        } else {
            None
        }
    })
}
