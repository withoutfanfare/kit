use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

use crate::domain::{DefaultView, LocationKind, Preferences, SavedLocation};

/// Stable id for the Global location. Reserved — user locations never use it.
pub const GLOBAL_LOCATION_ID: &str = "__global__";

// ---------------------------------------------------------------------------
// Persisted state (written to ~/.kit/state.json)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedState {
    pub preferences: Preferences,
    pub locations: Vec<SavedLocation>,
    /// Timestamp of the last skills repository status check.
    #[serde(default)]
    pub last_repo_check_at: Option<DateTime<Utc>>,
    /// Content hashes of skills at the time they were assigned, keyed by
    /// "locationId:skillId".
    #[serde(default)]
    pub skill_hashes: HashMap<String, SkillHashRecord>,
    /// Skills temporarily disabled at specific locations, keyed by
    /// "locationId:skillId". Disabled skills keep their symlink but are
    /// removed from the manifest so Claude won't load them.
    #[serde(default)]
    pub disabled_skills: HashSet<String>,
    /// Snapshots of SKILL.md content captured at assignment time, keyed by
    /// "locationId:skillId". Used by the diff viewer to show what changed.
    #[serde(default)]
    pub skill_snapshots: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SkillHashRecord {
    pub hash: String,
    pub assigned_at: Option<DateTime<Utc>>,
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
                track_skill_versions: true,
            },
            locations: Vec::new(),
            last_repo_check_at: None,
            skill_hashes: HashMap::new(),
            disabled_skills: HashSet::new(),
            skill_snapshots: HashMap::new(),
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
        let mut inner = if state_path.exists() {
            match fs::read_to_string(&state_path) {
                Ok(json) => serde_json::from_str::<PersistedState>(&json)
                    .unwrap_or_default(),
                Err(_) => PersistedState::default(),
            }
        } else {
            PersistedState::default()
        };
        ensure_global_location(&mut inner);
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

/// Make sure the Global location (`~/.claude/skills`) is present and correct.
///
/// It is synthesised rather than stored by the user, so it survives an old state
/// file, a hand-edited one, or a home directory that has moved. Any user location
/// that happens to sit on the same path is folded into it so the folder is never
/// listed twice.
fn ensure_global_location(state: &mut PersistedState) {
    let Some(path) = global_skills_path() else {
        return;
    };
    ensure_global_location_at(state, &path);
}

/// The body of [`ensure_global_location`], with the path injected so it can be
/// tested without depending on the real home directory.
fn ensure_global_location_at(state: &mut PersistedState, path: &Path) {
    let path_str = path.to_string_lossy().to_string();

    state
        .locations
        .retain(|l| l.id != GLOBAL_LOCATION_ID && l.path != path_str);

    state.locations.insert(
        0,
        SavedLocation {
            id: GLOBAL_LOCATION_ID.to_string(),
            label: "Global".to_string(),
            path: path_str,
            notes: Some("Loads in every session, in every project.".to_string()),
            last_synced_at: None,
            kind: LocationKind::Global,
        },
    );
}

/// `~/.claude/skills` — the folder Claude Code reads for always-on skills.
pub fn global_skills_path() -> Option<PathBuf> {
    Some(dirs::home_dir()?.join(".claude").join("skills"))
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn project(id: &str, path: &str) -> SavedLocation {
        SavedLocation {
            id: id.to_string(),
            label: id.to_string(),
            path: path.to_string(),
            notes: None,
            last_synced_at: None,
            kind: LocationKind::Project,
        }
    }

    #[test]
    fn global_location_is_added_first_and_marked_global() {
        let mut state = PersistedState {
            locations: vec![project("a", "/tmp/a")],
            ..Default::default()
        };
        ensure_global_location_at(&mut state, Path::new("/home/x/.claude/skills"));

        assert_eq!(state.locations.len(), 2);
        assert_eq!(state.locations[0].id, GLOBAL_LOCATION_ID);
        assert_eq!(state.locations[0].kind, LocationKind::Global);
        assert_eq!(state.locations[1].id, "a");
    }

    /// Called on every load, so it must not stack up duplicates.
    #[test]
    fn ensuring_global_twice_leaves_one() {
        let mut state = PersistedState::default();
        let path = Path::new("/home/x/.claude/skills");
        ensure_global_location_at(&mut state, path);
        ensure_global_location_at(&mut state, path);

        assert_eq!(
            state
                .locations
                .iter()
                .filter(|l| l.kind == LocationKind::Global)
                .count(),
            1
        );
    }

    /// A user who had already added `~/.claude/skills` by hand must not end up
    /// with the same folder listed twice, once as a project and once as Global.
    #[test]
    fn a_user_location_on_the_global_path_is_folded_in() {
        let mut state = PersistedState {
            locations: vec![
                project("hand-added", "/home/x/.claude/skills"),
                project("keep-me", "/tmp/other"),
            ],
            ..Default::default()
        };
        ensure_global_location_at(&mut state, Path::new("/home/x/.claude/skills"));

        assert_eq!(state.locations.len(), 2);
        assert!(!state.locations.iter().any(|l| l.id == "hand-added"));
        assert!(state.locations.iter().any(|l| l.id == "keep-me"));
        assert_eq!(state.locations[0].kind, LocationKind::Global);
    }
}
