pub mod commands;
pub mod domain;
pub mod linker;
pub mod scanner;
pub mod state;
pub mod tray;

use commands::assignment::{apply_assignment, preview_assignment};
use commands::bootstrap::{get_app_bootstrap, get_app_data_path, update_preferences};
use commands::external::{open_path_in_editor, open_with_default_app, resolve_skill_path, reveal_in_finder};
use commands::library::{archive_skill, get_skill_detail, list_library_items, unarchive_skill};
use commands::locations::{
    add_location, get_location_detail, list_locations, remove_location, sync_location,
    update_location,
};
use commands::manifest::update_manifest_entry;
use commands::repo::{copy_repo_pull_command, get_skills_repo_status, recheck_skills_repo_status, validate_skills_repository};
use commands::sets::{add_skill_to_set, create_set, delete_set, get_set_detail, list_sets, remove_skill_from_set, update_set};
use commands::usage::get_usage_summary;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .manage(state::new_shared_state())
        .invoke_handler(tauri::generate_handler![
            // Bootstrap
            get_app_bootstrap,
            update_preferences,
            get_app_data_path,
            // Locations
            list_locations,
            add_location,
            update_location,
            remove_location,
            get_location_detail,
            sync_location,
            // Library
            list_library_items,
            get_skill_detail,
            archive_skill,
            unarchive_skill,
            // Assignment
            preview_assignment,
            apply_assignment,
            // Sets
            list_sets,
            create_set,
            get_set_detail,
            update_set,
            delete_set,
            add_skill_to_set,
            remove_skill_from_set,
            // Manifest
            update_manifest_entry,
            // Usage
            get_usage_summary,
            // Repository
            validate_skills_repository,
            get_skills_repo_status,
            recheck_skills_repo_status,
            copy_repo_pull_command,
            // External
            open_path_in_editor,
            reveal_in_finder,
            open_with_default_app,
            resolve_skill_path,
        ])
        .setup(|app| {
            tray::setup_tray(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
