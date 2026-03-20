use std::fs;
use std::io::{Read as _, Write as _};
use std::path::PathBuf;
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::scanner;
use crate::state::SharedState;

/// Export selected skills (or a set's skills) as a .zip bundle.
#[tauri::command]
pub fn export_skill_bundle(
    skill_ids: Vec<String>,
    bundle_name: String,
    description: Option<String>,
    output_path: String,
    state: State<'_, SharedState>,
) -> Result<String, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let output = PathBuf::from(&output_path);
    let zip_path = output.join(format!("{}.kit-bundle.zip", bundle_name));

    let file = fs::File::create(&zip_path)
        .map_err(|e| AppError::new(format!("Failed to create zip file: {}", e)))?;
    let mut zip = zip::ZipWriter::new(file);

    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    // Write the export manifest
    let manifest = ExportManifest {
        name: bundle_name.clone(),
        description,
        exported_at: chrono::Utc::now(),
        skills: skill_ids.clone(),
        set_definition: None,
    };
    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    zip.start_file("kit-bundle.json", options)
        .map_err(|e| AppError::new(format!("Failed to write manifest: {}", e)))?;
    zip.write_all(manifest_json.as_bytes())
        .map_err(|e| AppError::new(format!("Failed to write manifest: {}", e)))?;

    // Copy each skill's SKILL.md into the zip
    for sid in &skill_ids {
        let skill_dir = library_root.join(sid);
        let skill_md = skill_dir.join("SKILL.md");
        if skill_md.is_file() {
            let content = fs::read_to_string(&skill_md)
                .map_err(|e| AppError::new(format!("Failed to read {}/SKILL.md: {}", sid, e)))?;
            let zip_name = format!("{}/SKILL.md", sid);
            zip.start_file(&zip_name, options)
                .map_err(|e| AppError::new(format!("Failed to write to zip: {}", e)))?;
            zip.write_all(content.as_bytes())
                .map_err(|e| AppError::new(format!("Failed to write to zip: {}", e)))?;
        }
    }

    zip.finish()
        .map_err(|e| AppError::new(format!("Failed to finalise zip: {}", e)))?;

    Ok(zip_path.to_string_lossy().to_string())
}

/// Preview what an import would do, without actually importing.
#[tauri::command]
pub fn preview_import_bundle(
    bundle_path: String,
    state: State<'_, SharedState>,
) -> Result<ImportPreview, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);
    let library_skills = scanner::scan_library_skills(&library_root);

    let existing_ids: std::collections::HashSet<String> = library_skills
        .iter()
        .map(|s| s.folder_name.clone())
        .collect();

    let file = fs::File::open(&bundle_path)
        .map_err(|e| AppError::new(format!("Failed to open bundle: {}", e)))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| AppError::new(format!("Invalid zip bundle: {}", e)))?;

    // Read the manifest
    let manifest: ExportManifest = {
        let mut manifest_file = archive
            .by_name("kit-bundle.json")
            .map_err(|_| AppError::new("Bundle is missing kit-bundle.json manifest"))?;
        let mut content = String::new();
        manifest_file
            .read_to_string(&mut content)
            .map_err(|e| AppError::new(format!("Failed to read manifest: {}", e)))?;
        serde_json::from_str(&content)?
    };

    let mut skills = Vec::new();
    let mut conflict_count = 0;

    for sid in &manifest.skills {
        let already_exists = existing_ids.contains(sid);
        if already_exists {
            conflict_count += 1;
        }
        // Try to get a name from the bundle's SKILL.md
        let name = archive
            .by_name(&format!("{}/SKILL.md", sid))
            .ok()
            .and_then(|mut f| {
                let mut content = String::new();
                f.read_to_string(&mut content).ok()?;
                scanner::parse_skill_md(&content).map(|fm| fm.name)
            })
            .unwrap_or_else(|| sid.clone());

        skills.push(ImportSkillEntry {
            id: sid.clone(),
            name,
            already_exists,
        });
    }

    Ok(ImportPreview {
        skills,
        set_definition: manifest.set_definition,
        conflict_count,
    })
}

/// Import a .zip bundle into the library root.
#[tauri::command]
pub fn import_skill_bundle(
    bundle_path: String,
    overwrite_existing: bool,
    state: State<'_, SharedState>,
) -> Result<usize, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let prefs = guard.preferences().clone();
    let library_root = PathBuf::from(&prefs.library_root);

    let file = fs::File::open(&bundle_path)
        .map_err(|e| AppError::new(format!("Failed to open bundle: {}", e)))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| AppError::new(format!("Invalid zip bundle: {}", e)))?;

    // Read the manifest
    let manifest: ExportManifest = {
        let mut manifest_file = archive
            .by_name("kit-bundle.json")
            .map_err(|_| AppError::new("Bundle is missing kit-bundle.json manifest"))?;
        let mut content = String::new();
        manifest_file
            .read_to_string(&mut content)
            .map_err(|e| AppError::new(format!("Failed to read manifest: {}", e)))?;
        serde_json::from_str(&content)?
    };

    let mut imported_count = 0;

    for sid in &manifest.skills {
        let target_dir = library_root.join(sid);
        if target_dir.exists() && !overwrite_existing {
            continue;
        }

        // Extract SKILL.md
        let zip_name = format!("{}/SKILL.md", sid);
        if let Ok(mut entry) = archive.by_name(&zip_name) {
            fs::create_dir_all(&target_dir)
                .map_err(|e| AppError::new(format!("Failed to create {}: {}", sid, e)))?;
            let mut content = String::new();
            entry
                .read_to_string(&mut content)
                .map_err(|e| AppError::new(format!("Failed to read {}: {}", sid, e)))?;
            fs::write(target_dir.join("SKILL.md"), &content)
                .map_err(|e| AppError::new(format!("Failed to write {}/SKILL.md: {}", sid, e)))?;
            imported_count += 1;
        }
    }

    Ok(imported_count)
}
