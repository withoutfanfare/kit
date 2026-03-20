use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};

/// Status of the filesystem watcher.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WatcherStatus {
    Active,
    Paused,
    Error,
    Stopped,
}

/// Managed watcher state, held via Tauri's manage().
pub struct ManagedWatcher {
    inner: Mutex<WatcherInner>,
}

struct WatcherInner {
    status: WatcherStatus,
    _debouncer: Option<notify_debouncer_mini::Debouncer<notify::RecommendedWatcher>>,
    watched_path: Option<PathBuf>,
}

impl Default for ManagedWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ManagedWatcher {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(WatcherInner {
                status: WatcherStatus::Stopped,
                _debouncer: None,
                watched_path: None,
            }),
        }
    }

    /// Start watching the given library root. Emits "library-changed" events
    /// to the Tauri window whenever a SKILL.md file changes.
    pub fn start(
        &self,
        library_root: PathBuf,
        app_handle: tauri::AppHandle,
    ) -> Result<(), String> {
        use notify::RecursiveMode;
        use tauri::Emitter;

        let mut guard = self.inner.lock().map_err(|e| e.to_string())?;

        // Stop existing watcher if any
        guard._debouncer = None;
        guard.watched_path = None;
        guard.status = WatcherStatus::Stopped;

        if !library_root.is_dir() {
            return Err(format!(
                "Library root is not a directory: {}",
                library_root.display()
            ));
        }

        let handle = Arc::new(app_handle);
        let handle_clone = handle.clone();

        let debouncer = new_debouncer(Duration::from_secs(2), move |result: Result<Vec<notify_debouncer_mini::DebouncedEvent>, notify::Error>| {
            if let Ok(events) = result {
                let has_skill_change = events.iter().any(|e| {
                    e.kind == DebouncedEventKind::Any
                        && e.path
                            .file_name()
                            .map(|n| n == "SKILL.md")
                            .unwrap_or(false)
                });
                if has_skill_change {
                    let _ = handle_clone.emit("library-changed", ());
                }
            }
        })
        .map_err(|e| format!("Failed to create watcher: {}", e))?;

        // Clone the debouncer's watcher to add a path
        let mut debouncer = debouncer;
        debouncer
            .watcher()
            .watch(&library_root, RecursiveMode::Recursive)
            .map_err(|e| format!("Failed to watch {}: {}", library_root.display(), e))?;

        guard._debouncer = Some(debouncer);
        guard.watched_path = Some(library_root);
        guard.status = WatcherStatus::Active;

        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        let mut guard = self.inner.lock().map_err(|e| e.to_string())?;
        guard._debouncer = None;
        guard.watched_path = None;
        guard.status = WatcherStatus::Stopped;
        Ok(())
    }

    pub fn status(&self) -> WatcherStatus {
        self.inner
            .lock()
            .map(|g| g.status.clone())
            .unwrap_or(WatcherStatus::Error)
    }

    pub fn watched_path(&self) -> Option<String> {
        self.inner
            .lock()
            .ok()
            .and_then(|g| g.watched_path.as_ref().map(|p| p.to_string_lossy().to_string()))
    }
}
