//! Headless bootstrap strategy.
//!
//! Selected when neither a display server nor an interactive terminal is
//! present (e.g. SSH pipe, CI, container build). After install the user is
//! directed to the gRPC / REST API.

use crate::error::FsInitError;
use crate::strategy::BootstrapStrategy;
use crate::wizard::WizardResult;

pub struct HeadlessBootstrap;

impl BootstrapStrategy for HeadlessBootstrap {
    fn post_install_hint(&self) -> &'static str {
        "After installation, manage FreeSynergy via the gRPC / REST API."
    }

    fn launch(&self, result: &WizardResult) -> Result<(), FsInitError> {
        println!();
        println!("Next steps:");
        println!("  1. Start fs-store to activate the package catalog.");
        println!("  2. Interact via gRPC (port 50051) or REST (port 8080).");
        println!("  Bundle:     {}", result.bundle_id);
        println!("  Engine:     {}", result.engine_id);
        println!("  Target:     {}", result.install_target.label());
        println!("  Store path: {}", result.store_path.display());
        Ok(())
    }
}
