//! Bootstrap strategy selection (Strategy Pattern).
//!
//! `BootstrapStrategy` defines what the wizard does *after* installation
//! completes: whether to launch a GUI, a TUI, or stay headless.
//! All strategies share the same CLI-based install wizard because the render
//! engine is not yet installed when fs-init runs.

pub mod gui;
pub mod headless;
pub mod tui;

use crate::capability::{BootstrapCapability, BootstrapMode};
use crate::error::FsInitError;
use crate::wizard::{WizardMachine, WizardResult};

/// What to do after the install wizard completes.
pub trait BootstrapStrategy {
    /// Hint shown to the user during the wizard.
    fn post_install_hint(&self) -> &'static str;

    /// Run the post-install launch sequence.
    ///
    /// In Phase 1 this prints next-step instructions.
    /// In later phases this will launch the appropriate UI.
    fn launch(&self, result: &WizardResult) -> Result<(), FsInitError>;
}

/// Select the appropriate strategy based on detected capabilities.
pub fn select_strategy(cap: &BootstrapCapability) -> Box<dyn BootstrapStrategy> {
    match cap.mode {
        BootstrapMode::Gui => Box::new(gui::GuiBootstrap),
        BootstrapMode::Tui => Box::new(tui::TuiBootstrap),
        BootstrapMode::Headless => Box::new(headless::HeadlessBootstrap),
    }
}

/// Run the full bootstrap for a given capability.
pub fn run(cap: BootstrapCapability) -> Result<(), FsInitError> {
    let strategy = select_strategy(&cap);
    let machine = WizardMachine::new(cap, strategy.post_install_hint());
    let result = machine.run()?;
    strategy.launch(&result)
}
