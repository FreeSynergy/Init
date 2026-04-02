//! TUI bootstrap strategy.
//!
//! Selected when no display server is present but an interactive terminal
//! (TTY) is available. After install the user is instructed to use the CLI.

use crate::error::FsInitError;
use crate::strategy::BootstrapStrategy;
use crate::wizard::WizardResult;

pub struct TuiBootstrap;

impl BootstrapStrategy for TuiBootstrap {
    fn post_install_hint(&self) -> &'static str {
        "After installation, use the CLI or TUI to manage FreeSynergy."
    }

    fn launch(&self, result: &WizardResult) -> Result<(), FsInitError> {
        println!();
        println!("Next steps:");
        println!("  1. Start fs-store to activate the package catalog.");
        println!("  2. Use `fs-store list` or `fs-store install <pkg>` to manage packages.");
        println!("  Bundle:     {}", result.bundle_id);
        println!("  Engine:     {}", result.engine_id);
        println!("  Target:     {}", result.install_target.label());
        println!("  Store path: {}", result.store_path.display());
        Ok(())
    }
}
