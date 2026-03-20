use std::path::PathBuf;
use std::process::Command;

use tauri::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};

use crate::scanner;
use crate::state::SharedState;

/// Get the path of the frontmost Finder window via AppleScript.
/// Always checks Finder's front window regardless of which app is currently active,
/// since clicking the tray icon changes the frontmost app away from Finder.
fn get_finder_path() -> Option<String> {
    let output = Command::new("osascript")
        .args([
            "-e",
            r#"tell application "Finder"
    if (count of windows) > 0 then
        set currentDir to (target of front window) as alias
        return POSIX path of currentDir
    end if
end tell
return "none""#,
        ])
        .output()
        .ok()?;

    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path == "none" || path.is_empty() {
        None
    } else {
        Some(path.trim_end_matches('/').to_string())
    }
}

/// Build the tray menu dynamically based on the current Finder path.
fn build_tray_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    let menu = Menu::new(app)?;

    // Get Finder path
    let finder_path = get_finder_path();

    if let Some(ref dir) = finder_path {
        // Header: show the current directory
        let folder_name = PathBuf::from(dir)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(dir)
            .to_string();

        let header = MenuItem::new(app, &folder_name, false, None::<&str>)?;
        menu.append(&header)?;

        let separator = PredefinedMenuItem::separator(app)?;
        menu.append(&separator)?;

        // Check for skills in this location — clone state quickly, release lock before scanning
        let state = app.state::<SharedState>();
        let (library_root, saved_locations) = {
            let guard = match state.lock() {
                Ok(g) => g,
                Err(poisoned) => poisoned.into_inner(),
            };
            let prefs = guard.preferences().clone();
            (PathBuf::from(&prefs.library_root), guard.locations().to_vec())
        };
        let library_skills = scanner::scan_library_skills(&library_root);

        let location_path = PathBuf::from(dir);
        let skills_dir = scanner::find_skills_dir(&location_path);

        if let Some(ref sd) = skills_dir {
            if let Ok(entries) = std::fs::read_dir(sd) {
                let mut skill_count = 0;
                for entry in entries.flatten() {
                    let name = match entry.file_name().to_str() {
                        Some(n) => n.to_string(),
                        None => continue,
                    };
                    if name.starts_with('.') {
                        continue;
                    }
                    if !entry.path().is_dir() {
                        continue;
                    }

                    // Get display name from library if available
                    let display_name = library_skills
                        .iter()
                        .find(|s| s.folder_name == name)
                        .map(|s| s.name.clone())
                        .unwrap_or_else(|| name.clone());

                    let is_symlink = std::fs::symlink_metadata(entry.path())
                        .map(|m| m.file_type().is_symlink())
                        .unwrap_or(false);

                    let label = if is_symlink {
                        format!("  {} (linked)", display_name)
                    } else {
                        format!("  {} (local)", display_name)
                    };

                    let item = MenuItem::new(app, &label, false, None::<&str>)?;
                    menu.append(&item)?;
                    skill_count += 1;
                }

                if skill_count == 0 {
                    let none = MenuItem::new(app, "  No skills installed", false, None::<&str>)?;
                    menu.append(&none)?;
                }
            }
        } else {
            let none = MenuItem::new(app, "  No skills directory found", false, None::<&str>)?;
            menu.append(&none)?;
        }

        let sep2 = PredefinedMenuItem::separator(app)?;
        menu.append(&sep2)?;

        // Check if this location is already saved (using already-cloned data)
        let loc_id = saved_locations.iter().find(|l| l.path == *dir).map(|l| l.id.clone());

        if let Some(id) = loc_id {
            let manage_item = MenuItem::with_id(
                app,
                format!("open_location:{}", id),
                "Manage Skills...",
                true,
                None::<&str>,
            )?;
            menu.append(&manage_item)?;
        } else {
            let add_item = MenuItem::with_id(
                app,
                format!("add_location:{}", dir),
                "Add to Kit...",
                true,
                None::<&str>,
            )?;
            menu.append(&add_item)?;
        }
    } else {
        let no_finder = MenuItem::new(app, "No Finder window active", false, None::<&str>)?;
        menu.append(&no_finder)?;
    }

    let sep3 = PredefinedMenuItem::separator(app)?;
    menu.append(&sep3)?;

    let open_kit = MenuItem::with_id(app, "open_kit", "Open Kit", true, None::<&str>)?;
    menu.append(&open_kit)?;

    let quit = MenuItem::with_id(app, "quit", "Quit Kit", true, None::<&str>)?;
    menu.append(&quit)?;

    Ok(menu)
}

/// Handle tray menu item clicks.
fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
    let id = event.id().as_ref();

    if id == "open_kit" {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.show();
            let _ = window.set_focus();
        }
    } else if id == "quit" {
        app.exit(0);
    } else if let Some(loc_id) = id.strip_prefix("open_location:") {
        // Open the app and navigate to this location
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.show();
            let _ = window.set_focus();
            let _ = window.emit("navigate", format!("/locations/{}", loc_id));
        }
    } else if let Some(dir) = id.strip_prefix("add_location:") {
        // Add as a location and open the app
        let state = app.state::<SharedState>();
        let mut guard = match state.lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        let path = PathBuf::from(dir);
        let canonical = std::fs::canonicalize(&path)
            .unwrap_or_else(|_| path.clone());
        let canonical_str = canonical.to_string_lossy().to_string();

        // Check not already added
        if !guard.locations().iter().any(|l| l.path == canonical_str) {
            let label = canonical
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();

            let id = uuid::Uuid::new_v4().to_string();
            let loc = crate::domain::SavedLocation {
                id: id.clone(),
                label,
                path: canonical_str,
                notes: None,
                last_synced_at: Some(chrono::Utc::now()),
            };
            guard.locations_mut().push(loc);
            let _ = guard.save();

            drop(guard);

            // Open the app at this location
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
                let _ = window.emit("navigate", format!("/locations/{}", id));
            }
        }
    }
}

/// Set up the system tray for the app.
pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let menu = build_tray_menu(app)?;

    let icon = app
        .default_window_icon()
        .ok_or_else(|| tauri::Error::AssetNotFound("default window icon".into()))?
        .clone();

    let tray = TrayIconBuilder::new()
        .tooltip("Kit — Skill Manager")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(move |app, event| {
            handle_menu_event(app, event);
        })
        .build(app)?;

    // Refresh the tray menu every 3 seconds so it reflects the current Finder window.
    // This avoids rebuilding during the click event which causes snap-shut on macOS.
    let app_handle = app.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3));
            if let Ok(new_menu) = build_tray_menu(&app_handle) {
                let _ = tray.set_menu(Some(new_menu));
            }
        }
    });

    Ok(())
}
