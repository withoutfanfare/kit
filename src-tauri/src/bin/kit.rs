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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Apply { set, project, sets_dir, library } => {
            cmd_apply(&set, &project, sets_dir.as_deref(), library.as_deref(), cli.json)
        }
        Command::List { project } => cmd_list(&project, cli.json),
        Command::Sets { sets_dir } => cmd_sets(sets_dir.as_deref(), cli.json),
    }
}

// Function stubs — body added in Task 2
fn cmd_apply(_set: &str, _project: &Path, _sets_dir: Option<&Path>, _library: Option<&Path>, _json: bool) -> Result<()> { bail!("not implemented") }
fn cmd_list(_project: &Path, _json: bool) -> Result<()> { bail!("not implemented") }
fn cmd_sets(_sets_dir: Option<&Path>, _json: bool) -> Result<()> { bail!("not implemented") }
