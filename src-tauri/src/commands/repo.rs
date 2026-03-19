use std::path::Path;
use std::process::Command;
use std::sync::mpsc;
use std::time::Duration;

use chrono::Utc;
use tauri::State;

use crate::commands::AppError;
use crate::domain::*;
use crate::state::SharedState;

// ---------------------------------------------------------------------------
// Helper: run a git command with a timeout
// ---------------------------------------------------------------------------

fn run_git(repo_path: &Path, args: &[&str], timeout_secs: u64) -> Result<String, String> {
    let repo = repo_path.to_path_buf();
    let owned_args: Vec<String> = args.iter().map(|a| a.to_string()).collect();

    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let result = Command::new("git")
            .arg("-C")
            .arg(&repo)
            .args(&owned_args)
            .output();
        let _ = tx.send(result);
    });

    let output = rx
        .recv_timeout(Duration::from_secs(timeout_secs))
        .map_err(|_| "Git command timed out".to_string())?
        .map_err(|e| format!("Failed to run git: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn validate_skills_repository(path: String) -> Result<SkillsRepoValidation, AppError> {
    let dir = Path::new(&path);
    let mut issues: Vec<String> = Vec::new();

    // Check directory exists and is readable
    if !dir.is_dir() {
        return Ok(SkillsRepoValidation {
            valid: false,
            path: path.clone(),
            is_git_repo: false,
            detected_branch: None,
            skill_count: 0,
            issues: vec!["Directory does not exist or is not readable".to_string()],
        });
    }

    // Check if it's a git repo
    let is_git_repo = dir.join(".git").exists()
        || run_git(dir, &["rev-parse", "--git-dir"], 5).is_ok();

    if !is_git_repo {
        issues.push("Git metadata was not found here".to_string());
    }

    // Get current branch name
    let detected_branch = if is_git_repo {
        run_git(dir, &["rev-parse", "--abbrev-ref", "HEAD"], 5).ok()
    } else {
        None
    };

    // Count skills by scanning for subdirectories containing SKILL.md
    let mut skill_count: usize = 0;
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if !entry_path.is_dir() {
                continue;
            }
            let folder_name = match entry_path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            if folder_name.starts_with('.') {
                continue;
            }
            if entry_path.join("SKILL.md").exists() {
                skill_count += 1;
            }
        }
    }

    if skill_count == 0 {
        issues.push("No skills were found in this folder".to_string());
    }

    let valid = is_git_repo && skill_count > 0;

    Ok(SkillsRepoValidation {
        valid,
        path: path.clone(),
        is_git_repo,
        detected_branch,
        skill_count,
        issues,
    })
}

/// Build a SkillsRepoStatus by inspecting the git state of the configured
/// skills repository. If `do_fetch` is true, run `git fetch` first (with a
/// 10 s timeout); otherwise only run a dry-run fetch (5 s timeout) to probe
/// reachability.
fn build_repo_status(
    repo_path: &Path,
    do_fetch: bool,
) -> SkillsRepoStatus {
    let path_str = repo_path.to_string_lossy().to_string();

    // Branch
    let branch = run_git(repo_path, &["rev-parse", "--abbrev-ref", "HEAD"], 5).ok();

    // Upstream
    let upstream = run_git(repo_path, &["rev-parse", "--abbrev-ref", "@{u}"], 5).ok();

    // Fetch (real or dry-run)
    let fetch_ok = if do_fetch {
        run_git(repo_path, &["fetch"], 10).is_ok()
    } else {
        run_git(repo_path, &["fetch", "--dry-run"], 5).is_ok()
    };

    // Ahead / behind
    let (ahead_by, behind_by) = if upstream.is_some() {
        run_git(repo_path, &["rev-list", "--left-right", "--count", "HEAD...@{u}"], 5)
            .ok()
            .and_then(|output| {
                let parts: Vec<&str> = output.split_whitespace().collect();
                if parts.len() == 2 {
                    let ahead = parts[0].parse::<usize>().unwrap_or(0);
                    let behind = parts[1].parse::<usize>().unwrap_or(0);
                    Some((ahead, behind))
                } else {
                    None
                }
            })
            .unwrap_or((0, 0))
    } else {
        (0, 0)
    };

    // Uncommitted changes
    let has_uncommitted_changes = run_git(repo_path, &["status", "--porcelain"], 5)
        .map(|output| !output.is_empty())
        .unwrap_or(false);

    // Determine state and message
    let (state, message) = if !fetch_ok && upstream.is_none() {
        (RepoState::Unavailable, "Could not check repository status".to_string())
    } else if has_uncommitted_changes {
        (RepoState::Dirty, "Skills repository has local changes".to_string())
    } else if ahead_by > 0 && behind_by > 0 {
        (
            RepoState::Diverged,
            format!(
                "Skills repository has diverged ({} ahead, {} behind {})",
                ahead_by,
                behind_by,
                upstream.as_deref().unwrap_or("upstream"),
            ),
        )
    } else if behind_by > 0 {
        (
            RepoState::Behind,
            format!(
                "Skills repository is {} commit{} behind {}",
                behind_by,
                if behind_by == 1 { "" } else { "s" },
                upstream.as_deref().unwrap_or("upstream"),
            ),
        )
    } else if ahead_by > 0 {
        (
            RepoState::Ahead,
            format!(
                "Skills repository is {} commit{} ahead of {}",
                ahead_by,
                if ahead_by == 1 { "" } else { "s" },
                upstream.as_deref().unwrap_or("upstream"),
            ),
        )
    } else {
        (RepoState::UpToDate, "Skills repository is up to date".to_string())
    };

    let now = Utc::now().to_rfc3339();

    SkillsRepoStatus {
        path: path_str,
        branch,
        upstream,
        state,
        ahead_by,
        behind_by,
        has_uncommitted_changes,
        last_checked_at: Some(now),
        message,
    }
}

#[tauri::command]
pub fn get_skills_repo_status(
    state: State<'_, SharedState>,
) -> Result<SkillsRepoStatus, AppError> {
    let library_root = {
        let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
        guard.preferences().library_root.clone()
    };

    if library_root.is_empty() {
        return Err(AppError::new("No skills repository configured"));
    }

    let repo_path = Path::new(&library_root);
    if !repo_path.is_dir() {
        return Err(AppError::new(format!(
            "Skills repository path does not exist: {}",
            library_root
        )));
    }

    // Run git commands without holding the lock
    let status = build_repo_status(repo_path, false);

    // Re-acquire lock to persist the check timestamp
    let mut guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    guard.inner.last_repo_check_at = Some(Utc::now());
    guard.save().map_err(AppError::new)?;

    Ok(status)
}

#[tauri::command]
pub fn recheck_skills_repo_status(
    state: State<'_, SharedState>,
) -> Result<SkillsRepoStatus, AppError> {
    let library_root = {
        let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
        guard.preferences().library_root.clone()
    };

    if library_root.is_empty() {
        return Err(AppError::new("No skills repository configured"));
    }

    let repo_path = Path::new(&library_root);
    if !repo_path.is_dir() {
        return Err(AppError::new(format!(
            "Skills repository path does not exist: {}",
            library_root
        )));
    }

    // Run git fetch without holding the lock
    let status = build_repo_status(repo_path, true);

    // Re-acquire lock to persist the check timestamp
    let mut guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    guard.inner.last_repo_check_at = Some(Utc::now());
    guard.save().map_err(AppError::new)?;

    Ok(status)
}

#[tauri::command]
pub fn copy_repo_pull_command(
    state: State<'_, SharedState>,
) -> Result<String, AppError> {
    let guard = state.lock().map_err(|e| AppError::new(e.to_string()))?;
    let library_root = guard.preferences().library_root.clone();

    if library_root.is_empty() {
        return Err(AppError::new("No skills repository configured"));
    }

    let quoted = library_root.replace('\'', "'\\''");
    Ok(format!("git -C '{}' pull", quoted))
}
