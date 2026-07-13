use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::env;
use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};

/// kit — Claude Code skill loadout CLI.
#[derive(Parser)]
#[command(name = "kit", version, about = "Claude Code skill loadout CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Emit JSON instead of human-readable output.
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Apply a named skill set to a project.
    Apply {
        /// Set name (resolved against --sets-dir).
        #[arg(long)]
        set: String,
        /// Project path.
        #[arg(long)]
        project: PathBuf,
        /// Override sets directory.
        #[arg(long)]
        sets_dir: Option<PathBuf>,
        /// Override skills library root.
        #[arg(long)]
        library: Option<PathBuf>,
    },
    /// List active skills and team binding at a project.
    List {
        /// Project path.
        #[arg(long)]
        project: PathBuf,
    },
    /// List available sets.
    Sets {
        /// Override sets directory.
        #[arg(long)]
        sets_dir: Option<PathBuf>,
    },
    /// Symlink one or more individual skills into a project.
    Link {
        /// Skill names to link (space-separated).
        skills: Vec<String>,
        /// Project path.
        #[arg(long)]
        project: PathBuf,
        /// Override skills library root.
        #[arg(long)]
        library: Option<PathBuf>,
    },
    /// Remove one or more skill symlinks from a project.
    Unlink {
        /// Skill names to unlink (space-separated).
        skills: Vec<String>,
        /// Project path.
        #[arg(long)]
        project: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Apply { set, project, sets_dir, library } => {
            cmd_apply(&set, &project, sets_dir.as_deref(), library.as_deref(), cli.json)
        }
        Command::List { project } => cmd_list(&project, cli.json),
        Command::Sets { sets_dir } => cmd_sets(sets_dir.as_deref(), cli.json),
        Command::Link { skills, project, library } => {
            cmd_link(&skills, &project, library.as_deref(), cli.json)
        }
        Command::Unlink { skills, project } => cmd_unlink(&skills, &project, cli.json),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn expand_tilde(p: &Path) -> PathBuf {
    if let Some(rest) = p.to_str().and_then(|s| s.strip_prefix("~/")) {
        if let Some(home) = dirs_home() {
            return home.join(rest);
        }
    }
    p.to_path_buf()
}

fn dirs_home() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}

fn default_sets_dir() -> PathBuf {
    if let Ok(v) = env::var("KIT_SETS_DIR") {
        return expand_tilde(Path::new(&v));
    }
    dirs_home()
        .map(|h| h.join(".kit/sets"))
        .unwrap_or_else(|| PathBuf::from("./sets"))
}

fn default_library() -> PathBuf {
    // 1. Explicit env var override
    if let Ok(v) = env::var("KIT_LIBRARY") {
        return expand_tilde(Path::new(&v));
    }
    // 2. Kit GUI's configured library root (~/.kit/state.json)
    if let Some(from_state) = read_kit_library_root() {
        return from_state;
    }
    // 3. Claude Code convention
    dirs_home()
        .map(|h| h.join(".claude/skills"))
        .unwrap_or_else(|| PathBuf::from("./skills"))
}

/// Read `preferences.libraryRoot` from `~/.kit/state.json` if present.
/// Ignores everything else — we only care about one field and accept
/// Kit's JSON schema evolving freely.
fn read_kit_library_root() -> Option<PathBuf> {
    let state_path = dirs_home()?.join(".kit/state.json");
    let content = fs::read_to_string(&state_path).ok()?;
    let parsed: serde_json::Value = serde_json::from_str(&content).ok()?;
    let root = parsed.get("preferences")?.get("libraryRoot")?.as_str()?;
    if root.is_empty() {
        return None;
    }
    Some(expand_tilde(Path::new(root)))
}

fn resolve_project(project: &Path) -> Result<PathBuf> {
    let p = expand_tilde(project);
    fs::canonicalize(&p)
        .with_context(|| format!("Project path not found: {}", p.display()))
}

fn read_set_file(sets_dir: &Path, set_name: &str) -> Result<Vec<String>> {
    let path = sets_dir.join(format!("{set_name}.txt"));
    let body = fs::read_to_string(&path)
        .with_context(|| format!("Set not found: {}", path.display()))?;
    let skills: Vec<String> = body
        .lines()
        .map(|l| l.split('#').next().unwrap_or("").trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    Ok(skills)
}

// ---------------------------------------------------------------------------
// apply
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct ApplyReport {
    set: String,
    project: String,
    applied: Vec<String>,
    missing: Vec<String>,
}

fn cmd_apply(
    set: &str,
    project: &Path,
    sets_dir: Option<&Path>,
    library: Option<&Path>,
    json: bool,
) -> Result<()> {
    let project = resolve_project(project)?;
    let sets_dir = sets_dir.map(PathBuf::from).unwrap_or_else(default_sets_dir);
    let library = library.map(PathBuf::from).unwrap_or_else(default_library);

    let skills = read_set_file(&sets_dir, set)?;

    let skills_dir = project.join(".claude/skills");
    fs::create_dir_all(&skills_dir)
        .with_context(|| format!("Cannot create {}", skills_dir.display()))?;

    let mut applied = Vec::new();
    let mut missing = Vec::new();

    for skill in &skills {
        let src = library.join(skill);
        let dst = skills_dir.join(skill);

        if !src.is_dir() {
            missing.push(skill.clone());
            if !json {
                eprintln!("  ! missing skill: {} ({})", skill, src.display());
            }
            continue;
        }

        // Idempotent replace: remove any existing entry (symlink, file, or dir)
        // before creating a fresh symlink.
        if dst.exists() || fs::symlink_metadata(&dst).is_ok() {
            if let Ok(meta) = fs::symlink_metadata(&dst) {
                if meta.file_type().is_symlink() || meta.is_file() {
                    fs::remove_file(&dst).ok();
                } else if meta.is_dir() {
                    fs::remove_dir_all(&dst).ok();
                }
            }
        }
        unix_fs::symlink(&src, &dst)
            .with_context(|| format!("Failed to symlink {} -> {}", dst.display(), src.display()))?;
        applied.push(skill.clone());
        if !json {
            println!("  + {skill}");
        }
    }

    if json {
        let report = ApplyReport {
            set: set.to_string(),
            project: project.display().to_string(),
            applied,
            missing: missing.clone(),
        };
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!(
            "Applied set '{}' to {}: {} symlinked, {} missing.",
            set,
            project.display(),
            applied.len(),
            missing.len()
        );
    }

    if !missing.is_empty() {
        std::process::exit(1);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// list
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct ListEntry {
    name: String,
    target: Option<String>,
    is_symlink: bool,
}

#[derive(Serialize)]
struct ListReport {
    project: String,
    team_badge: Option<String>,
    skills: Vec<ListEntry>,
}

fn cmd_list(project: &Path, json: bool) -> Result<()> {
    let project = resolve_project(project)?;
    let badge_path = project.join(".claude/team.md");
    let badge = if badge_path.exists() {
        Some(fs::read_to_string(&badge_path)?)
    } else {
        None
    };

    let skills_dir = project.join(".claude/skills");
    let mut entries: Vec<ListEntry> = Vec::new();
    if skills_dir.is_dir() {
        let mut items: Vec<_> = fs::read_dir(&skills_dir)?
            .filter_map(|e| e.ok())
            .collect();
        items.sort_by_key(|e| e.file_name());
        for entry in items {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let is_symlink = fs::symlink_metadata(&path)?.file_type().is_symlink();
            let target = if is_symlink {
                Some(fs::read_link(&path)?.display().to_string())
            } else {
                None
            };
            entries.push(ListEntry { name, target, is_symlink });
        }
    }

    if json {
        let report = ListReport {
            project: project.display().to_string(),
            team_badge: badge,
            skills: entries,
        };
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("Project: {}", project.display());
        println!();
        match &badge {
            Some(b) => {
                println!("Team badge:");
                for line in b.lines() {
                    println!("  {line}");
                }
            }
            None => println!("Team badge: (none — not registered)"),
        }
        println!();
        if entries.is_empty() {
            println!("Active skills: (none)");
        } else {
            println!("Active skills:");
            for e in &entries {
                match &e.target {
                    Some(t) => println!("  - {} -> {}", e.name, t),
                    None => println!("  - {} (not a symlink)", e.name),
                }
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// sets
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct SetEntry {
    name: String,
    skill_count: usize,
    path: String,
}

fn cmd_sets(sets_dir: Option<&Path>, json: bool) -> Result<()> {
    let sets_dir = sets_dir.map(PathBuf::from).unwrap_or_else(default_sets_dir);
    if !sets_dir.is_dir() {
        bail!(
            "Sets directory does not exist: {}\n\nTo get started:\n  mkdir -p {}\n\nOr point Kit at an existing sets directory:\n  export KIT_SETS_DIR=/path/to/sets   # persistent\n  kit sets --sets-dir /path/to/sets   # one-off",
            sets_dir.display(),
            sets_dir.display()
        );
    }

    let mut entries: Vec<SetEntry> = Vec::new();
    for dirent in fs::read_dir(&sets_dir)? {
        let dirent = dirent?;
        let path = dirent.path();
        if path.extension().and_then(|s| s.to_str()) != Some("txt") {
            continue;
        }
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let body = fs::read_to_string(&path)?;
        let skill_count = body
            .lines()
            .filter_map(|l| {
                let s = l.split('#').next().unwrap_or("").trim();
                if s.is_empty() { None } else { Some(s) }
            })
            .count();
        entries.push(SetEntry {
            name,
            skill_count,
            path: path.display().to_string(),
        });
    }
    entries.sort_by(|a, b| a.name.cmp(&b.name));

    if json {
        println!("{}", serde_json::to_string_pretty(&entries)?);
    } else {
        println!("Sets directory: {}", sets_dir.display());
        if entries.is_empty() {
            println!("No sets found.");
        } else {
            for e in &entries {
                println!("  - {} ({} skills) [{}]", e.name, e.skill_count, e.path);
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// link
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct LinkReport {
    project: String,
    added: Vec<String>,
    missing: Vec<String>,
}

fn cmd_link(skills: &[String], project: &Path, library: Option<&Path>, json: bool) -> Result<()> {
    if skills.is_empty() {
        bail!("At least one skill name is required. Usage: kit link <skill> [<skill> ...] --project <path>");
    }
    let project = resolve_project(project)?;
    let library = library.map(PathBuf::from).unwrap_or_else(default_library);

    let skills_dir = project.join(".claude/skills");
    fs::create_dir_all(&skills_dir)
        .with_context(|| format!("Cannot create {}", skills_dir.display()))?;

    let mut added = Vec::new();
    let mut missing = Vec::new();

    for skill in skills {
        let src = library.join(skill);
        let dst = skills_dir.join(skill);

        if !src.is_dir() {
            missing.push(skill.clone());
            if !json {
                eprintln!("  ! not in library: {} ({})", skill, src.display());
            }
            continue;
        }

        // Idempotent replace: wipe anything already at dst.
        if let Ok(meta) = fs::symlink_metadata(&dst) {
            if meta.file_type().is_symlink() || meta.is_file() {
                fs::remove_file(&dst).ok();
            } else if meta.is_dir() {
                fs::remove_dir_all(&dst).ok();
            }
        }
        unix_fs::symlink(&src, &dst)
            .with_context(|| format!("Failed to symlink {} -> {}", dst.display(), src.display()))?;
        added.push(skill.clone());
        if !json {
            println!("  + {skill}");
        }
    }

    if json {
        let report = LinkReport {
            project: project.display().to_string(),
            added,
            missing: missing.clone(),
        };
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!(
            "Added {} skill(s) to {}.",
            added.len(),
            project.display()
        );
        if !missing.is_empty() {
            println!("Missing from library: {}", missing.join(", "));
        }
    }

    if !missing.is_empty() {
        std::process::exit(1);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// unlink
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct UnlinkReport {
    project: String,
    removed: Vec<String>,
    not_found: Vec<String>,
    refused: Vec<String>,
}

fn cmd_unlink(skills: &[String], project: &Path, json: bool) -> Result<()> {
    if skills.is_empty() {
        bail!("At least one skill name is required. Usage: kit unlink <skill> [<skill> ...] --project <path>");
    }
    let project = resolve_project(project)?;

    let skills_dir = project.join(".claude/skills");
    let mut removed = Vec::new();
    let mut not_found = Vec::new();
    let mut refused = Vec::new();

    for skill in skills {
        let dst = skills_dir.join(skill);
        match fs::symlink_metadata(&dst) {
            Ok(meta) => {
                if meta.file_type().is_symlink() {
                    fs::remove_file(&dst)
                        .with_context(|| format!("Failed to remove {}", dst.display()))?;
                    removed.push(skill.clone());
                    if !json {
                        println!("  - {skill}");
                    }
                } else {
                    refused.push(skill.clone());
                    if !json {
                        eprintln!(
                            "  ! refusing to delete non-symlink: {} ({})",
                            skill,
                            dst.display()
                        );
                    }
                }
            }
            Err(_) => {
                not_found.push(skill.clone());
                if !json {
                    eprintln!("  ! not linked in this project: {}", skill);
                }
            }
        }
    }

    if json {
        let report = UnlinkReport {
            project: project.display().to_string(),
            removed,
            not_found: not_found.clone(),
            refused: refused.clone(),
        };
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!(
            "Removed {} skill(s) from {}.",
            removed.len(),
            project.display()
        );
        if !not_found.is_empty() {
            println!("Not linked: {}", not_found.join(", "));
        }
        if !refused.is_empty() {
            println!("Refused (not a symlink): {}", refused.join(", "));
        }
    }

    if !refused.is_empty() || !not_found.is_empty() {
        std::process::exit(1);
    }
    Ok(())
}
