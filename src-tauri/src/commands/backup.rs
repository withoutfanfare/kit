use std::collections::HashMap;
use std::fs;
use std::io::{Read as _, Write as _};
use std::path::PathBuf;

use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::scanner;
use crate::state::{self, SharedState};

/// Compute a simple checksum (SHA-256 hex) for integrity verification.
fn checksum(content: &[u8]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Backup the entire skill library to a .zip archive.
#[tauri::command]
pub fn backup_library(
    output_path: String,
    state: State<'_, SharedState>,
) -> Result<BackupResult, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    if !library_root.is_dir() {
        return Err(AppError::new("Library root directory does not exist"));
    }

    let output = PathBuf::from(&output_path);
    let timestamp = chrono::Utc::now().format("%Y-%m-%d");
    let zip_path = output.join(format!("kit-library-backup-{}.zip", timestamp));

    let file = fs::File::create(&zip_path)
        .map_err(|e| AppError::new(format!("Failed to create backup file: {}", e)))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let mut checksums: HashMap<String, String> = HashMap::new();
    let mut skill_count = 0usize;
    let mut set_count = 0usize;

    // Archive all skill folders (each containing SKILL.md)
    let skills = scanner::scan_library_skills(&library_root);
    for skill in &skills {
        let skill_dir = library_root.join(&skill.folder_name);
        let skill_md = skill_dir.join("SKILL.md");
        if skill_md.is_file() {
            let content = fs::read_to_string(&skill_md)
                .map_err(|e| AppError::new(format!("Failed to read {}/SKILL.md: {}", skill.folder_name, e)))?;
            let zip_name = format!("skills/{}/SKILL.md", skill.folder_name);
            let hash = checksum(content.as_bytes());
            checksums.insert(zip_name.clone(), hash);
            zip.start_file(&zip_name, options)
                .map_err(|e| AppError::new(format!("Failed to write to zip: {}", e)))?;
            zip.write_all(content.as_bytes())
                .map_err(|e| AppError::new(format!("Failed to write to zip: {}", e)))?;
            skill_count += 1;
        }
    }

    // Archive all set definitions (*.set.json in sets/)
    let sets = scanner::scan_library_sets(&library_root);
    let sets_dir = library_root.join("sets");
    for (set_id, _) in &sets {
        let set_file = sets_dir.join(format!("{}.set.json", set_id));
        if set_file.is_file() {
            let content = fs::read_to_string(&set_file)
                .map_err(|e| AppError::new(format!("Failed to read set {}: {}", set_id, e)))?;
            let zip_name = format!("sets/{}.set.json", set_id);
            let hash = checksum(content.as_bytes());
            checksums.insert(zip_name.clone(), hash);
            zip.start_file(&zip_name, options)
                .map_err(|e| AppError::new(format!("Failed to write to zip: {}", e)))?;
            zip.write_all(content.as_bytes())
                .map_err(|e| AppError::new(format!("Failed to write to zip: {}", e)))?;
            set_count += 1;
        }
    }

    // Archive state.json (usage counters, preferences)
    let state_path = dirs::home_dir()
        .map(|h| h.join(".kit").join("state.json"))
        .unwrap_or_default();
    if state_path.is_file() {
        let content = fs::read_to_string(&state_path)
            .map_err(|e| AppError::new(format!("Failed to read state.json: {}", e)))?;
        let hash = checksum(content.as_bytes());
        checksums.insert("state.json".to_string(), hash);
        zip.start_file("state.json", options)
            .map_err(|e| AppError::new(format!("Failed to write state.json to zip: {}", e)))?;
        zip.write_all(content.as_bytes())
            .map_err(|e| AppError::new(format!("Failed to write state.json to zip: {}", e)))?;
    }

    // Write the backup manifest
    let manifest = BackupManifest {
        version: 1,
        created_at: chrono::Utc::now(),
        library_root: prefs.library_root.clone(),
        skill_count,
        set_count,
        checksums,
    };
    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    zip.start_file("kit-backup-manifest.json", options)
        .map_err(|e| AppError::new(format!("Failed to write manifest: {}", e)))?;
    zip.write_all(manifest_json.as_bytes())
        .map_err(|e| AppError::new(format!("Failed to write manifest: {}", e)))?;

    zip.finish()
        .map_err(|e| AppError::new(format!("Failed to finalise zip: {}", e)))?;

    let metadata = fs::metadata(&zip_path)
        .map_err(|e| AppError::new(format!("Failed to read backup file size: {}", e)))?;

    Ok(BackupResult {
        path: zip_path.to_string_lossy().to_string(),
        skill_count,
        set_count,
        size_bytes: metadata.len(),
    })
}

/// Preview what a restore would do, without actually restoring.
#[tauri::command]
pub fn preview_restore(
    backup_path: String,
    state: State<'_, SharedState>,
) -> Result<RestorePreview, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let file = fs::File::open(&backup_path)
        .map_err(|e| AppError::new(format!("Failed to open backup: {}", e)))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| AppError::new(format!("Invalid backup archive: {}", e)))?;

    // Verify this is a valid Kit backup by checking for the manifest
    archive
        .by_name("kit-backup-manifest.json")
        .map_err(|_| AppError::new("Backup is missing kit-backup-manifest.json — not a valid Kit backup"))?;

    // Check which skills already exist
    let existing_skills = scanner::scan_library_skills(&library_root);
    let existing_skill_ids: std::collections::HashSet<String> = existing_skills
        .iter()
        .map(|s| s.folder_name.clone())
        .collect();

    // Check which sets already exist
    let existing_sets = scanner::scan_library_sets(&library_root);
    let existing_set_ids: std::collections::HashSet<String> = existing_sets
        .iter()
        .map(|(id, _)| id.clone())
        .collect();

    let mut conflicts = Vec::new();

    // Scan archive for skills
    let mut skill_count = 0usize;
    let mut set_count = 0usize;
    for i in 0..archive.len() {
        let entry = archive.by_index(i)
            .map_err(|e| AppError::new(format!("Failed to read archive entry: {}", e)))?;
        let name = entry.name().to_string();

        if let Some(rest) = name.strip_prefix("skills/") {
            if let Some(folder) = rest.strip_suffix("/SKILL.md") {
                skill_count += 1;
                if existing_skill_ids.contains(folder) {
                    conflicts.push(RestoreConflict {
                        name: folder.to_string(),
                        kind: RestoreConflictKind::SkillExists,
                    });
                }
            }
        } else if let Some(rest) = name.strip_prefix("sets/") {
            if let Some(set_id) = rest.strip_suffix(".set.json") {
                set_count += 1;
                if existing_set_ids.contains(set_id) {
                    conflicts.push(RestoreConflict {
                        name: set_id.to_string(),
                        kind: RestoreConflictKind::SetExists,
                    });
                }
            }
        }
    }

    let has_state = archive.by_name("state.json").is_ok();

    Ok(RestorePreview {
        skill_count,
        set_count,
        has_state,
        conflicts,
    })
}

/// Restore a library from a backup archive.
#[tauri::command]
pub fn restore_library(
    backup_path: String,
    overwrite_existing: bool,
    restore_state: bool,
    state: State<'_, SharedState>,
) -> Result<RestoreResult, AppError> {
    let mut guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let file = fs::File::open(&backup_path)
        .map_err(|e| AppError::new(format!("Failed to open backup: {}", e)))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| AppError::new(format!("Invalid backup archive: {}", e)))?;

    let mut skills_restored = 0usize;
    let mut skills_skipped = 0usize;
    let mut sets_restored = 0usize;
    let mut sets_skipped = 0usize;

    // Collect entry names first to avoid borrow issues
    let entry_names: Vec<String> = (0..archive.len())
        .filter_map(|i| archive.by_index(i).ok().map(|e| e.name().to_string()))
        .collect();

    // Restore skills
    for name in &entry_names {
        if let Some(rest) = name.strip_prefix("skills/") {
            if let Some(folder) = rest.strip_suffix("/SKILL.md") {
                let target_dir = library_root.join(folder);
                if target_dir.exists() && !overwrite_existing {
                    skills_skipped += 1;
                    continue;
                }
                let mut entry = archive.by_name(name)
                    .map_err(|e| AppError::new(format!("Failed to read {}: {}", name, e)))?;
                let mut content = String::new();
                entry.read_to_string(&mut content)
                    .map_err(|e| AppError::new(format!("Failed to read {}: {}", name, e)))?;
                fs::create_dir_all(&target_dir)
                    .map_err(|e| AppError::new(format!("Failed to create {}: {}", folder, e)))?;
                fs::write(target_dir.join("SKILL.md"), &content)
                    .map_err(|e| AppError::new(format!("Failed to write {}/SKILL.md: {}", folder, e)))?;
                skills_restored += 1;
            }
        }
    }

    // Restore sets
    let sets_dir = library_root.join("sets");
    for name in &entry_names {
        if let Some(rest) = name.strip_prefix("sets/") {
            if rest.ends_with(".set.json") {
                let target_file = sets_dir.join(rest);
                if target_file.exists() && !overwrite_existing {
                    sets_skipped += 1;
                    continue;
                }
                let mut entry = archive.by_name(name)
                    .map_err(|e| AppError::new(format!("Failed to read {}: {}", name, e)))?;
                let mut content = String::new();
                entry.read_to_string(&mut content)
                    .map_err(|e| AppError::new(format!("Failed to read {}: {}", name, e)))?;
                fs::create_dir_all(&sets_dir)
                    .map_err(|e| AppError::new(format!("Failed to create sets directory: {}", e)))?;
                fs::write(&target_file, &content)
                    .map_err(|e| AppError::new(format!("Failed to write {}: {}", rest, e)))?;
                sets_restored += 1;
            }
        }
    }

    // Optionally restore state.json
    if restore_state {
        if let Ok(mut entry) = archive.by_name("state.json") {
            let mut content = String::new();
            entry.read_to_string(&mut content)
                .map_err(|e| AppError::new(format!("Failed to read state.json: {}", e)))?;

            // Parse the backed-up state and merge into current
            if let Ok(backed_up) = serde_json::from_str::<state::PersistedState>(&content) {
                // Merge usage counters from backup (prefer higher counts)
                for (key, record) in backed_up.usage {
                    let existing = guard.inner.usage.get(&key);
                    let should_insert = match existing {
                        Some(ex) => record.use_count_30d > ex.use_count_30d,
                        None => true,
                    };
                    if should_insert {
                        guard.inner.usage.insert(key, record);
                    }
                }

                // Merge skill hashes (only add missing ones)
                for (key, record) in backed_up.skill_hashes {
                    guard.inner.skill_hashes.entry(key).or_insert(record);
                }

                // Merge snapshots (only add missing ones)
                for (key, snapshot) in backed_up.skill_snapshots {
                    guard.inner.skill_snapshots.entry(key).or_insert(snapshot);
                }

                guard.save().map_err(|e| AppError::new(format!("Failed to save state: {}", e)))?;
            }
        }
    }

    Ok(RestoreResult {
        skills_restored,
        sets_restored,
        skills_skipped,
        sets_skipped,
    })
}
