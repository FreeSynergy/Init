//! Bootstrap capability detection via fs-info.
//!
//! Determines which display environment and terminal is available,
//! and selects the appropriate `BootstrapMode` for the current system.

use fs_info::{DetectedFeatures, Feature, FeatureDetect, OsInfo};

// ── Display environment ───────────────────────────────────────────────────────

/// Which graphical display server (if any) is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayEnv {
    /// Wayland compositor is running.
    Wayland,
    /// X11 display server is running.
    X11,
    /// No display server — headless or SSH-only session.
    None,
}

impl DisplayEnv {
    fn from_features(features: &DetectedFeatures) -> Self {
        if features.has(Feature::WaylandDisplay) {
            DisplayEnv::Wayland
        } else if features.has(Feature::X11Display) {
            DisplayEnv::X11
        } else {
            DisplayEnv::None
        }
    }

    /// Human-readable label for this display environment.
    pub fn label(self) -> &'static str {
        match self {
            DisplayEnv::Wayland => "Wayland",
            DisplayEnv::X11 => "X11",
            DisplayEnv::None => "none (headless / SSH)",
        }
    }
}

// ── Bootstrap mode ────────────────────────────────────────────────────────────

/// The bootstrap mode determines which UI the wizard runs in.
///
/// All modes currently use the same CLI-based wizard (plain text output +
/// stdin input) because the render engine has not been installed yet.
/// The mode controls which bundles are pre-selected and what is launched
/// after installation completes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootstrapMode {
    /// Display server is available — GUI engine will be installed.
    Gui,
    /// No display server, but an interactive terminal is present — TUI mode.
    Tui,
    /// No display server and no interactive terminal — API + CLI only.
    Headless,
}

impl BootstrapMode {
    /// Human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            BootstrapMode::Gui => "GUI",
            BootstrapMode::Tui => "TUI",
            BootstrapMode::Headless => "Headless (API + CLI only)",
        }
    }
}

// ── Container runtime ─────────────────────────────────────────────────────────

/// Which container runtime is available.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerRuntime {
    Podman,
    Docker,
    None,
}

impl ContainerRuntime {
    fn from_features(features: &DetectedFeatures) -> Self {
        if features.has(Feature::Podman) {
            ContainerRuntime::Podman
        } else if features.has(Feature::Docker) {
            ContainerRuntime::Docker
        } else {
            ContainerRuntime::None
        }
    }

    /// Human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            ContainerRuntime::Podman => "Podman",
            ContainerRuntime::Docker => "Docker",
            ContainerRuntime::None => "none detected",
        }
    }
}

// ── Bootstrap capability ──────────────────────────────────────────────────────

/// All system capabilities relevant to bootstrapping.
pub struct BootstrapCapability {
    /// OS information (name, version, arch, hostname).
    pub os: OsInfo,
    /// Active display environment.
    pub display: DisplayEnv,
    /// Whether stdin is an interactive terminal.
    pub has_terminal: bool,
    /// Available container runtime.
    pub container: ContainerRuntime,
    /// Derived bootstrap mode.
    pub mode: BootstrapMode,
}

impl BootstrapCapability {
    /// Detect all capabilities from the live system.
    pub fn detect() -> Self {
        let features = FeatureDetect::run();
        let os = OsInfo::detect();
        let display = DisplayEnv::from_features(&features);
        let has_terminal = features.has_terminal();
        let container = ContainerRuntime::from_features(&features);
        let mode = derive_mode(display, has_terminal);

        BootstrapCapability {
            os,
            display,
            has_terminal,
            container,
            mode,
        }
    }
}

fn derive_mode(display: DisplayEnv, has_terminal: bool) -> BootstrapMode {
    if display != DisplayEnv::None {
        BootstrapMode::Gui
    } else if has_terminal {
        BootstrapMode::Tui
    } else {
        BootstrapMode::Headless
    }
}
