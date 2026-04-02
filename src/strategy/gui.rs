//! GUI bootstrap strategy.
//!
//! Selected when a Wayland or X11 display server is detected.
//! After install the user is instructed to launch the desktop.

use crate::error::FsInitError;
use crate::strategy::BootstrapStrategy;
use crate::wizard::WizardResult;

pub struct GuiBootstrap;

impl BootstrapStrategy for GuiBootstrap {
    fn post_install_hint(&self) -> &'static str {
        "After installation, start the FreeSynergy desktop to continue setup."
    }

    fn launch(&self, result: &WizardResult) -> Result<(), FsInitError> {
        println!();
        println!("Next steps:");
        println!("  1. Start fs-store to activate the package catalog.");
        println!("  2. Launch fs-desktop to open the desktop environment.");
        println!("  Bundle:     {}", result.bundle_id);
        println!("  Engine:     {}", result.engine_id);
        println!("  Target:     {}", result.install_target.label());
        println!("  Store path: {}", result.store_path.display());
        Ok(())
    }
}
