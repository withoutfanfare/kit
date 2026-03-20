use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

use crate::domain::{DefaultView, Preferences, SavedLocation};

// ---------------------------------------------------------------------------
// Persisted state (written to ~/.kit/state.json)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedState {
    pub preferences: Preferences,
    pub locations: Vec<SavedLocation>,
    /// Lightweight per-skill usage counters keyed by skill folder name.
    #[serde(default)]
    pub usage: HashMap<String, UsageRecord>,
    /// Timestamp of the last skills repository status check.
    #[serde(default)]
    pub last_repo_check_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsageRecord {
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub use_count_30d: usize,
}

impl Default for PersistedState {
    fn default() -> Self {
        let library_root = default_library_root()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        Self {
            preferences: Preferences {
                library_root,
                editor_command: String::from("code"),
                default_view: DefaultView::Locations,
                show_archived: false,
            },
            locations: Vec::new(),
            usage: HashMap::new(),
            last_repo_check_at: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Tauri managed state wrapper
// ---------------------------------------------------------------------------

pub struct AppState {
    pub inner: PersistedState,
    state_path: PathBuf,
}

impl AppState {
    /// Load from disk or create default.
    pub fn load() -> Self {
        let state_path = state_file_path();
        let inner = if state_path.exists() {
            match fs::read_to_string(&state_path) {
                Ok(json) => serde_json::from_str::<PersistedState>(&json)
                    .unwrap_or_default(),
                Err(_) => PersistedState::default(),
            }
        } else {
            PersistedState::default()
        };
        Self { inner, state_path }
    }

    /// Persist current state to disk.
    pub fn save(&self) -> Result<(), String> {
        if let Some(parent) = self.state_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create state directory: {e}"))?;
        }
        let json = serde_json::to_string_pretty(&self.inner)
            .map_err(|e| format!("Failed to serialise state: {e}"))?;
        atomic_write(&self.state_path, &json)
            .map_err(|e| format!("Failed to write state file: {e}"))?;
        Ok(())
    }

    pub fn preferences(&self) -> &Preferences {
        &self.inner.preferences
    }

    pub fn locations(&self) -> &[SavedLocation] {
        &self.inner.locations
    }

    pub fn locations_mut(&mut self) -> &mut Vec<SavedLocation> {
        &mut self.inner.locations
    }

    pub fn find_location(&self, id: &str) -> Option<&SavedLocation> {
        self.inner.locations.iter().find(|l| l.id == id)
    }

    pub fn find_location_mut(&mut self, id: &str) -> Option<&mut SavedLocation> {
        self.inner.locations.iter_mut().find(|l| l.id == id)
    }
}

/// Wrapped in a Mutex for Tauri managed state.
pub type SharedState = Mutex<AppState>;

pub fn new_shared_state() -> SharedState {
    Mutex::new(AppState::load())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Write content to a file atomically by writing to a .tmp sibling then renaming.
pub fn atomic_write(path: &Path, content: &str) -> Result<(), std::io::Error> {
    let mut tmp_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "file".to_string());
    tmp_name.push_str(".tmp");
    let tmp_path = path.with_file_name(&tmp_name);
    fs::write(&tmp_path, content)?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}

fn state_file_path() -> PathBuf {
    let home = dirs::home_dir().expect("Could not determine home directory");
    home.join(".kit").join("state.json")
}

fn default_library_root() -> Option<PathBuf> {
    // Try to auto-detect the library root by following symlinks in ~/.claude/skills/.
    // The library root is the SOURCE repository where SKILL.md files live,
    // NOT ~/.claude/skills/ itself (which is a destination for symlinks).
    let home = dirs::home_dir()?;
    let claude_skills = home.join(".claude").join("skills");
    if claude_skills.is_dir() {
        // Check the first symlink to find the common source directory
        if let Ok(entries) = std::fs::read_dir(&claude_skills) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_symlink() {
                    if let Ok(target) = std::fs::read_link(&path) {
                        let resolved = if target.is_relative() {
                            claude_skills.join(&target)
                        } else {
                            target
                        };
                        // The parent of the symlink target is the library root
                        if let Some(parent) = resolved.parent() {
                            if parent.is_dir() {
                                return Some(parent.to_path_buf());
                            }
                        }
                    }
                }
            }
        }
    }
    // No auto-detection possible — user must configure in Settings
    None
}
