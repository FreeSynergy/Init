//! FTL key constants with hardcoded English fallback values.
//!
//! Every user-facing string must be declared here.
//! The FTL key name is documented in the comment above each constant.
//! The value is the English fallback embedded in the binary.
//! When fs-i18n is available at runtime, the FTL key is used instead.

// ── General ───────────────────────────────────────────────────────────────────

/// FTL key: `init-title`
pub const INIT_TITLE: &str = "FreeSynergy Init";

/// FTL key: `init-version`
pub const INIT_DIVIDER: &str = "────────────────────────────────────────────────";

/// FTL key: `init-abort-hint`
pub const INIT_ABORT_HINT: &str = "Press Ctrl+C at any time to abort.";

/// FTL key: `init-prompt-continue`
pub const INIT_PROMPT_CONTINUE: &str = "Press Enter to continue…";

/// FTL key: `init-prompt-choice`
pub const INIT_PROMPT_CHOICE: &str = "Enter number: ";

/// FTL key: `init-invalid-choice`
pub const INIT_INVALID_CHOICE: &str = "Invalid choice. Please try again.";

// ── Capability detection ──────────────────────────────────────────────────────

/// FTL key: `init-detecting-capabilities`
pub const INIT_DETECTING_CAPABILITIES: &str = "Detecting system capabilities…";

/// FTL key: `init-capability-os`
pub const INIT_CAPABILITY_OS: &str = "  OS:             ";

/// FTL key: `init-capability-arch`
pub const INIT_CAPABILITY_ARCH: &str = "  Architecture:   ";

// ── Wizard steps ──────────────────────────────────────────────────────────────

/// FTL key: `init-step-welcome-title`
pub const INIT_STEP_WELCOME_TITLE: &str = "Welcome";

/// FTL key: `init-step-welcome-body`
pub const INIT_STEP_WELCOME_BODY: &str =
    "This wizard will guide you through installing FreeSynergy on this node.\n\
     It will clone the official store and help you choose what to install.";

/// FTL key: `init-step-capability-title`
pub const INIT_STEP_CAPABILITY_TITLE: &str = "System Capabilities";

/// FTL key: `init-step-engine-title`
pub const INIT_STEP_ENGINE_TITLE: &str = "Render Engine";

/// FTL key: `init-step-engine-prompt`
pub const INIT_STEP_ENGINE_PROMPT: &str = "Choose the render engine for the desktop UI.\n\
     (Only relevant if you install a bundle with a desktop.)";

/// FTL key: `init-step-bundle-title`
pub const INIT_STEP_BUNDLE_TITLE: &str = "Bundle Selection";

/// FTL key: `init-step-bundle-prompt`
pub const INIT_STEP_BUNDLE_PROMPT: &str = "Choose a bundle to install:";

/// FTL key: `init-step-confirm-title`
pub const INIT_STEP_CONFIRM_TITLE: &str = "Confirm Installation";

/// FTL key: `init-step-confirm-bundle`
pub const INIT_STEP_CONFIRM_BUNDLE: &str = "  Bundle:          ";

/// FTL key: `init-step-confirm-engine`
pub const INIT_STEP_CONFIRM_ENGINE: &str = "  Render engine:   ";

/// FTL key: `init-step-confirm-target`
pub const INIT_STEP_CONFIRM_TARGET: &str = "  Install target:  ";

/// FTL key: `init-step-confirm-question`
pub const INIT_STEP_CONFIRM_QUESTION: &str = "Proceed? [y/N]: ";

/// FTL key: `init-step-progress-title`
pub const INIT_STEP_PROGRESS_TITLE: &str = "Installing";

/// FTL key: `init-step-progress-cloning-store`
pub const INIT_STEP_PROGRESS_CLONING_STORE: &str = "  Cloning store catalog…";

/// FTL key: `init-step-progress-clone-ok`
pub const INIT_STEP_PROGRESS_CLONE_OK: &str = "  Store ready.";

/// FTL key: `init-step-progress-clone-exists`
pub const INIT_STEP_PROGRESS_CLONE_EXISTS: &str = "  Store already present — skipping clone.";

/// FTL key: `init-step-progress-install-pending`
pub const INIT_STEP_PROGRESS_INSTALL_PENDING: &str =
    "  Package install pipeline not yet available (Phase 2).";

/// FTL key: `init-step-done-title`
pub const INIT_STEP_DONE_TITLE: &str = "Done";

/// FTL key: `init-step-done-body`
pub const INIT_STEP_DONE_BODY: &str = "FreeSynergy has been bootstrapped.\n\
     The store is available locally. Start the Store service to continue.";

/// FTL key: `init-step-done-store-path`
pub const INIT_STEP_DONE_STORE_PATH: &str = "  Store path:      ";

// ── Target ────────────────────────────────────────────────────────────────────

/// FTL key: `init-target-container`
pub const INIT_TARGET_CONTAINER: &str = "Container (Podman / Docker)";

/// FTL key: `init-target-rpm`
pub const INIT_TARGET_RPM: &str = "RPM package";

/// FTL key: `init-target-deb`
pub const INIT_TARGET_DEB: &str = "DEB package";

/// FTL key: `init-target-appimage`
pub const INIT_TARGET_APPIMAGE: &str = "AppImage";
