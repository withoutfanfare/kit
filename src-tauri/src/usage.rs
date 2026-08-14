//! Reads the skill-usage logs written by the `Skill` PreToolUse hook.
//!
//! Kit used to keep its own counter in `~/.kit/state.json`. Nothing ever
//! incremented it, so every "never used" figure the app showed was derived from
//! an empty map — absence presented as evidence. The hook's logs are the real
//! record, and they carry the working directory of each invocation, so usage can
//! be attributed to a project rather than only counted globally.
//!
//! Each line of `<library>/.skill-tracking/logs/skill-usage/*.jsonl` looks like:
//!
//! ```json
//! {"event":"invoke","skill":"code-review","timestamp":"2026-08-12T07:02:00Z",
//!  "project":"kit","cwd":"/Users/x/code/kit","session":"…"}
//! ```

use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;

use crate::domain::{SkillUsage, UsageProjectCount, UsageReport, UsageSkillRow};

#[derive(Debug, Deserialize)]
struct RawEvent {
    skill: Option<String>,
    timestamp: Option<DateTime<Utc>>,
    cwd: Option<String>,
    /// The folder the run happened in, as the hook recorded it. Good enough to
    /// label a row; `cwd` is what attribution actually matches on.
    project: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UsageEvent {
    pub skill: String,
    pub at: DateTime<Utc>,
    pub cwd: Option<PathBuf>,
    pub project: Option<String>,
}

/// Every recorded invocation, ready to be queried.
#[derive(Debug, Clone, Default)]
pub struct UsageIndex {
    events: Vec<UsageEvent>,
    /// `false` when there is no usable record to read: no log directory, or log
    /// files that yielded not one valid event. Distinct from "read fine, and
    /// there is nothing in them" — no data must never be reported as zero uses,
    /// because "never used" is a recommendation to delete the skill.
    available: bool,
}

/// Where the hook writes its logs, relative to the library root.
pub fn log_dir(library_root: &Path) -> PathBuf {
    library_root
        .join(".skill-tracking")
        .join("logs")
        .join("skill-usage")
}

impl UsageIndex {
    pub fn load(library_root: &Path) -> Self {
        let dir = log_dir(library_root);
        let Ok(entries) = fs::read_dir(&dir) else {
            return Self::default();
        };

        let mut files: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "jsonl"))
            .collect();
        files.sort();

        let mut events = Vec::new();
        // Whether there was anything to fail at. Log files that all failed to
        // read or parse are a broken record, not an empty one.
        let mut had_content = false;
        for file in files {
            let Ok(handle) = fs::File::open(&file) else {
                had_content = true;
                continue;
            };
            // Read a line at a time: these logs grow without bound, and pulling a
            // whole one into memory to count rows in it is a needless risk.
            for line in BufReader::new(handle).lines() {
                let Ok(line) = line else {
                    // An unreadable line (bad UTF-8, I/O error) ends this file.
                    had_content = true;
                    break;
                };
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                had_content = true;
                // A malformed line is skipped rather than failing the whole read;
                // a truncated final write should not blank the history.
                let Ok(raw) = serde_json::from_str::<RawEvent>(line) else {
                    continue;
                };
                let (Some(skill), Some(at)) = (raw.skill, raw.timestamp) else {
                    continue;
                };
                events.push(UsageEvent {
                    skill,
                    at,
                    cwd: raw.cwd.map(PathBuf::from),
                    project: raw.project,
                });
            }
        }

        // Logs that held something but produced no event at all are evidence of
        // a broken hook, not of unused skills. Reporting them as "available with
        // zero uses" would list every skill in the library as never used.
        let unreadable_record = events.is_empty() && had_content;

        Self {
            events,
            available: !unreadable_record,
        }
    }

    pub fn is_available(&self) -> bool {
        self.available
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn earliest(&self) -> Option<DateTime<Utc>> {
        self.events.iter().map(|e| e.at).min()
    }

    /// Totals for one skill across everywhere it has been used.
    pub fn for_skill(&self, skill: &str) -> SkillUsage {
        let cutoff = Utc::now() - Duration::days(30);
        let mut last_used_at: Option<DateTime<Utc>> = None;
        let mut use_count_30d = 0;

        for event in self.events.iter().filter(|e| e.skill == skill) {
            if event.at >= cutoff {
                use_count_30d += 1;
            }
            if last_used_at.is_none_or(|current| event.at > current) {
                last_used_at = Some(event.at);
            }
        }

        SkillUsage {
            last_used_at,
            use_count_30d,
        }
    }

    /// Counts for one skill within a single directory tree.
    ///
    /// Matching is by working directory rather than the log's `project` field,
    /// which is only a folder name and collides across worktrees.
    pub fn for_skill_in(&self, skill: &str, root: &Path) -> usize {
        let root = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
        self.events
            .iter()
            .filter(|e| e.skill == skill)
            .filter(|e| {
                e.cwd.as_ref().is_some_and(|cwd| {
                    let cwd = fs::canonicalize(cwd).unwrap_or_else(|_| cwd.clone());
                    cwd.starts_with(&root)
                })
            })
            .count()
    }

    /// Every skill used inside a directory tree, with counts, most-used first.
    pub fn skills_used_in(&self, root: &Path) -> Vec<(String, usize)> {
        let root = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
        let mut counts: HashMap<&str, usize> = HashMap::new();

        for event in &self.events {
            let Some(cwd) = event.cwd.as_ref() else {
                continue;
            };
            let cwd = fs::canonicalize(cwd).unwrap_or_else(|_| cwd.clone());
            if cwd.starts_with(&root) {
                *counts.entry(event.skill.as_str()).or_default() += 1;
            }
        }

        let mut rows: Vec<(String, usize)> = counts
            .into_iter()
            .map(|(skill, count)| (skill.to_string(), count))
            .collect();
        rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        rows
    }

    /// Every skill that has ever run, ranked, with where it ran.
    ///
    /// This is the whole log reduced to the shape a decision needs: how often,
    /// how recently, and in which project.
    pub fn report(&self, library_skills: &[String]) -> UsageReport {
        let mut by_skill: HashMap<&str, Vec<&UsageEvent>> = HashMap::new();
        for event in &self.events {
            by_skill.entry(event.skill.as_str()).or_default().push(event);
        }

        let mut rows: Vec<UsageSkillRow> = by_skill
            .iter()
            .map(|(skill, events)| {
                let mut per_project: HashMap<&str, usize> = HashMap::new();
                for e in events {
                    let label = e.project.as_deref().unwrap_or("elsewhere");
                    *per_project.entry(label).or_default() += 1;
                }
                let mut projects: Vec<UsageProjectCount> = per_project
                    .into_iter()
                    .map(|(name, runs)| UsageProjectCount {
                        name: name.to_string(),
                        runs,
                    })
                    .collect();
                projects.sort_by(|a, b| b.runs.cmp(&a.runs).then_with(|| a.name.cmp(&b.name)));

                UsageSkillRow {
                    skill: (*skill).to_string(),
                    runs: events.len(),
                    last_used_at: events.iter().map(|e| e.at).max(),
                    in_library: library_skills.iter().any(|s| s == *skill),
                    projects,
                }
            })
            .collect();
        rows.sort_by(|a, b| b.runs.cmp(&a.runs).then_with(|| a.skill.cmp(&b.skill)));

        // A library skill with no event at all is the list worth acting on.
        let ever_used: std::collections::HashSet<&str> =
            by_skill.keys().copied().collect();
        let mut never_used: Vec<String> = library_skills
            .iter()
            .filter(|s| !ever_used.contains(s.as_str()))
            .cloned()
            .collect();
        never_used.sort();

        UsageReport {
            available: self.available,
            event_count: self.events.len(),
            recorded_since: self.earliest(),
            distinct_skills: rows.len(),
            rows,
            never_used,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(tag: &str, lines: &[&str]) -> PathBuf {
        let root = std::env::temp_dir().join(format!("kit-usage-{tag}-{}", std::process::id()));
        fs::remove_dir_all(&root).ok();
        let dir = log_dir(&root);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("2026-08-12.jsonl"), lines.join("\n")).unwrap();
        root
    }

    fn event(skill: &str, at: &str, cwd: &str) -> String {
        format!(
            r#"{{"event":"invoke","skill":"{skill}","timestamp":"{at}","cwd":"{cwd}"}}"#
        )
    }

    /// Environment smoke test against the real logs, for comparing with `jq`.
    /// `cargo test -- --ignored usage_on_this_machine --nocapture`
    #[test]
    #[ignore]
    fn usage_on_this_machine() {
        let state = crate::state::AppState::load();
        let root = PathBuf::from(&state.preferences().library_root);
        let index = UsageIndex::load(&root);

        println!("available: {}", index.is_available());
        println!("events: {}", index.event_count());
        println!("earliest: {:?}", index.earliest().map(|d| d.to_rfc3339()));

        for loc in state.locations() {
            if !Path::new(&loc.path).is_dir() {
                continue;
            }
            let used = index.skills_used_in(Path::new(&loc.path));
            println!(
                "  {:<24} {} distinct skills used here",
                loc.label,
                used.len()
            );
            for (skill, count) in used.iter().take(3) {
                println!("       {skill} x{count}");
            }
        }
    }

    /// No logs is not the same as no usage. The old counter conflated the two
    /// and told the user skills were unused when it simply had no record.
    #[test]
    fn a_missing_log_directory_reports_unavailable_not_zero() {
        let root = std::env::temp_dir().join("kit-usage-absent-does-not-exist");
        let index = UsageIndex::load(&root);

        assert!(!index.is_available());
        assert_eq!(index.event_count(), 0);
    }

    #[test]
    fn an_empty_log_directory_is_available_with_no_events() {
        let root = fixture("empty", &[]);
        let index = UsageIndex::load(&root);

        assert!(index.is_available(), "the logs exist, they are just quiet");
        assert_eq!(index.event_count(), 0);

        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn counts_are_attributed_to_the_directory_they_happened_in() {
        let base = std::env::temp_dir().join(format!("kit-usage-cwd-{}", std::process::id()));
        let alpha = base.join("alpha");
        let beta = base.join("beta");
        fs::create_dir_all(alpha.join("nested")).unwrap();
        fs::create_dir_all(&beta).unwrap();

        let now = Utc::now().to_rfc3339();
        let root = fixture(
            "cwd",
            &[
                &event("code-review", &now, alpha.to_str().unwrap()),
                &event("code-review", &now, alpha.join("nested").to_str().unwrap()),
                &event("code-review", &now, beta.to_str().unwrap()),
                &event("test-writer", &now, beta.to_str().unwrap()),
            ],
        );
        let index = UsageIndex::load(&root);

        assert_eq!(index.for_skill_in("code-review", &alpha), 2, "includes nested");
        assert_eq!(index.for_skill_in("code-review", &beta), 1);
        assert_eq!(index.for_skill("code-review").use_count_30d, 3);

        let in_beta = index.skills_used_in(&beta);
        assert_eq!(in_beta.len(), 2);
        assert!(in_beta.iter().any(|(s, c)| s == "test-writer" && *c == 1));

        fs::remove_dir_all(&root).ok();
        fs::remove_dir_all(&base).ok();
    }

    /// Old events still count towards "last used" but not the 30-day figure.
    #[test]
    fn the_thirty_day_count_excludes_older_events() {
        let old = (Utc::now() - Duration::days(90)).to_rfc3339();
        let recent = (Utc::now() - Duration::days(2)).to_rfc3339();
        let root = fixture(
            "window",
            &[
                &event("planner", &old, "/tmp"),
                &event("planner", &recent, "/tmp"),
            ],
        );
        let index = UsageIndex::load(&root);
        let usage = index.for_skill("planner");

        assert_eq!(usage.use_count_30d, 1);
        assert!(usage.last_used_at.is_some());

        fs::remove_dir_all(&root).ok();
    }

    /// Logs full of nothing Kit can read are a broken hook, not proof that no
    /// skill has ever run. Calling that "available" would put every skill in
    /// the library on the "never used, consider deleting" list.
    #[test]
    fn logs_that_yield_no_valid_event_are_unavailable_not_empty() {
        let root = fixture(
            "unparseable",
            &["{ not json", "also not json", r#"{"event":"invoke"}"#],
        );
        let index = UsageIndex::load(&root);

        assert!(
            !index.is_available(),
            "no valid event was read, so there is no evidence either way"
        );
        assert_eq!(index.event_count(), 0);

        let report = index.report(&["some-skill".to_string()]);
        assert!(!report.available);

        fs::remove_dir_all(&root).ok();
    }

    /// A half-written final line must not blank the history before it.
    #[test]
    fn a_malformed_line_is_skipped_not_fatal() {
        let now = Utc::now().to_rfc3339();
        let root = fixture(
            "malformed",
            &[
                &event("good", &now, "/tmp"),
                "{ this is not json",
                &event("also-good", &now, "/tmp"),
            ],
        );
        let index = UsageIndex::load(&root);

        assert_eq!(index.event_count(), 2);

        fs::remove_dir_all(&root).ok();
    }
}
