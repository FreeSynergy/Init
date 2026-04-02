//! Store catalog clone logic.
//!
//! Clones the official `FreeSynergy/Store` Git repository to a local path
//! so the node can use packages without requiring a pre-installed `git` binary.

use std::path::{Path, PathBuf};

use crate::error::FsInitError;

pub const DEFAULT_STORE_URL: &str = "https://github.com/FreeSynergy/Store.git";
pub const DEFAULT_BRANCH: &str = "main";

/// Default local path for the cloned store catalog.
pub fn default_store_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".local/share/freesynergy/store")
}

/// Clone `url` at `branch` into `target`.
pub fn clone_store(url: &str, branch: &str, target: &Path) -> Result<(), FsInitError> {
    let mut prepare = gix::clone::PrepareFetch::new(
        url,
        target,
        gix::create::Kind::WithWorktree,
        gix::create::Options::default(),
        gix::open::Options::isolated(),
    )
    .map_err(|e| FsInitError::StoreClone(e.to_string()))?;

    prepare = prepare
        .with_remote_name("origin")
        .map_err(|e| FsInitError::StoreClone(e.to_string()))?;

    let branch = branch.to_owned();
    prepare = prepare.configure_remote(move |remote| {
        let spec = format!("+refs/heads/{branch}:refs/remotes/origin/{branch}");
        Ok(remote.with_refspecs([spec.as_str()], gix::remote::Direction::Fetch)?)
    });

    let (mut checkout, _outcome) = prepare
        .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
        .map_err(|e| FsInitError::StoreClone(e.to_string()))?;

    checkout
        .main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
        .map_err(|e| FsInitError::StoreClone(e.to_string()))?;

    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn default_store_dir_uses_home() {
        env::set_var("HOME", "/tmp/test-home");
        let dir = default_store_dir();
        assert_eq!(
            dir,
            PathBuf::from("/tmp/test-home/.local/share/freesynergy/store")
        );
    }

    #[test]
    fn default_store_dir_fallback_when_no_home() {
        env::remove_var("HOME");
        let dir = default_store_dir();
        assert!(dir.ends_with(".local/share/freesynergy/store"));
    }
}
