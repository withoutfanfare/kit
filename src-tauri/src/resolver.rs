//! Works out what actually reaches a Claude Code session, and what it costs.
//!
//! Skills arrive from four places, and only two of them are folders Kit
//! manages. Answering "what is loading here and where did it come from" means
//! reading all four and applying the same rules Claude Code does:
//!
//! * a symlink or folder present in `~/.claude/skills` loads in every session;
//! * one in `<project>/.claude/skills` loads in that project as well;
//! * `skillOverrides` in `~/.claude/settings.json` is a **global veto** — a
//!   skill switched off there does not load even where a project links it;
//! * `disable-model-invocation: true` keeps a skill out of the model's list
//!   entirely, leaving it reachable only as a slash command.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::*;
use crate::scanner;

/// The one marketplace inside the Codex plugin cache whose packs are delivered
/// to Claude sessions at account level.
const COWORK_MARKETPLACE: &str = "claude-cowork";

/// The parts of `~/.claude/settings.json` that decide what loads.
#[derive(Debug, Default, Clone)]
pub struct ClaudeSettings {
    /// Keys set to `"off"`.
    pub overrides_off: BTreeSet<String>,
    /// `<plugin>@<marketplace>` → enabled.
    pub enabled_plugins: BTreeMap<String, bool>,
    /// The file is there but could not be read or parsed. Everything derived
    /// from it — vetoes, plugin enablement — is then a guess, and saying so is
    /// the only honest option.
    pub unreadable: bool,
}

/// Read the settings that govern loading. A missing file means "nothing
/// configured", never "everything off" — guessing the stricter reading would
/// have Kit report skills as absent when they are in fact loading. A file that
/// exists but cannot be read is a different thing again, and is flagged.
pub fn read_claude_settings(home: &Path) -> ClaudeSettings {
    let path = home.join(".claude").join("settings.json");
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return ClaudeSettings::default(),
        Err(_) => {
            return ClaudeSettings {
                unreadable: true,
                ..Default::default()
            }
        }
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) else {
        return ClaudeSettings {
            unreadable: true,
            ..Default::default()
        };
    };

    let overrides_off = value
        .get("skillOverrides")
        .and_then(|v| v.as_object())
        .map(|map| {
            map.iter()
                .filter(|(_, v)| v.as_str() == Some("off"))
                .map(|(k, _)| k.clone())
                .collect()
        })
        .unwrap_or_default();

    let enabled_plugins = value
        .get("enabledPlugins")
        .and_then(|v| v.as_object())
        .map(|map| {
            map.iter()
                .map(|(k, v)| (k.clone(), v.as_bool().unwrap_or(false)))
                .collect()
        })
        .unwrap_or_default();

    ClaudeSettings {
        overrides_off,
        enabled_plugins,
        unreadable: false,
    }
}

/// What a single SKILL.md contributes.
struct SkillFacts {
    folder_name: String,
    description: String,
    model_facing: bool,
}

fn read_skill_facts(skill_dir: &Path) -> Option<SkillFacts> {
    let folder_name = skill_dir.file_name()?.to_str()?.to_string();
    if folder_name.starts_with('.') {
        return None;
    }
    let content = fs::read_to_string(skill_dir.join("SKILL.md")).ok()?;
    let frontmatter = scanner::parse_skill_md(&content)?;

    // Only the frontmatter block decides this; a body mention must not count.
    let model_facing = !frontmatter_flag(&content, "disable-model-invocation");

    Some(SkillFacts {
        folder_name,
        description: frontmatter.description.unwrap_or_default(),
        model_facing,
    })
}

/// Read a boolean flag from the frontmatter block only.
fn frontmatter_flag(content: &str, key: &str) -> bool {
    let trimmed = content.trim_start();
    let Some(after_first) = trimmed.strip_prefix("---") else {
        return false;
    };
    let Some((end, _)) = scanner::find_closing_fence(after_first) else {
        return false;
    };
    after_first[..end].lines().any(|line| {
        line.trim()
            .strip_prefix(key)
            .and_then(|rest| rest.trim().strip_prefix(':'))
            .is_some_and(|value| value.trim() == "true")
    })
}

/// The model's skill list carries a name and a description per skill, so that
/// pair is what a skill costs simply by being available. Four characters per
/// token is the usual rough conversion, and rough is the honest precision here.
fn estimate_tokens(id: &str, description: &str) -> usize {
    (id.len() + description.len()) / 4
}

/// Every skill folder directly inside `dir`.
fn skill_dirs(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut found: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir() && p.join("SKILL.md").is_file())
        .collect();
    found.sort();
    found
}

fn resolve_folder(
    dir: &Path,
    origin: SkillOrigin,
    settings: &ClaudeSettings,
) -> Vec<ResolvedSkill> {
    skill_dirs(dir)
        .into_iter()
        .filter_map(|p| {
            let facts = read_skill_facts(&p)?;
            let vetoed_by = settings
                .overrides_off
                .contains(&facts.folder_name)
                .then(|| facts.folder_name.clone());
            Some(ResolvedSkill {
                id: facts.folder_name.clone(),
                token_estimate: estimate_tokens(&facts.folder_name, &facts.description),
                folder_name: facts.folder_name,
                origin,
                source_label: String::new(),
                model_facing: facts.model_facing,
                vetoed_by,
                path: p.to_string_lossy().to_string(),
            })
        })
        .collect()
}

/// One plugin Claude Code records as actually installed.
#[derive(Debug, Clone)]
pub struct InstalledPlugin {
    pub plugin: String,
    pub marketplace: String,
    pub install_path: PathBuf,
}

/// What `~/.claude/plugins/installed_plugins.json` says is installed, and where.
///
/// This is the file that settles two questions the cache cannot. The cache
/// keeps every version ever downloaded — often under content hashes, where
/// "newest" is not a thing you can read off the name — and it keeps whole
/// plugins long after they are gone. On a real machine that is 8 installed
/// plugins against 21 leftovers, so scanning the cache reports skills as
/// loading that Claude Code has not seen in months.
///
/// `None` means the file is missing or unreadable: the answer is unknown, and
/// the caller downgrades rather than guessing.
pub fn read_installed_plugins(home: &Path) -> Option<Vec<InstalledPlugin>> {
    let path = home
        .join(".claude")
        .join("plugins")
        .join("installed_plugins.json");
    let content = fs::read_to_string(&path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&content).ok()?;
    let plugins = value.get("plugins")?.as_object()?;

    let mut found = Vec::new();
    for (key, installs) in plugins {
        // Keys are `<plugin>@<marketplace>`.
        let Some((plugin, marketplace)) = key.rsplit_once('@') else {
            continue;
        };
        let Some(installs) = installs.as_array() else {
            continue;
        };
        for install in installs {
            let Some(install_path) = install.get("installPath").and_then(|v| v.as_str()) else {
                continue;
            };
            found.push(InstalledPlugin {
                plugin: plugin.to_string(),
                marketplace: marketplace.to_string(),
                install_path: PathBuf::from(install_path),
            });
        }
    }
    found.sort_by(|a, b| a.plugin.cmp(&b.plugin).then(a.marketplace.cmp(&b.marketplace)));
    Some(found)
}

/// The skills one installed plugin contributes, addressed as `plugin:skill`.
fn resolve_installed_plugin(installed: &InstalledPlugin) -> Vec<ResolvedSkill> {
    let mut skills: Vec<ResolvedSkill> = skill_dirs(&installed.install_path.join("skills"))
        .into_iter()
        .filter_map(|skill_dir| {
            let facts = read_skill_facts(&skill_dir)?;
            let id = format!("{}:{}", installed.plugin, facts.folder_name);
            Some(ResolvedSkill {
                token_estimate: estimate_tokens(&id, &facts.description),
                id,
                folder_name: facts.folder_name,
                origin: SkillOrigin::Plugin,
                source_label: installed.plugin.clone(),
                model_facing: facts.model_facing,
                // A bare-name override never matches `plugin:skill`.
                vetoed_by: None,
                path: skill_dir.to_string_lossy().to_string(),
            })
        })
        .collect();
    skills.sort_by(|a, b| a.id.cmp(&b.id));
    skills
}

/// Skills from a plugin cache laid out as
/// `<cache>/<marketplace>/<plugin>/<version>/skills/<skill>/SKILL.md`.
///
/// Several versions of the same plugin are usually cached side by side and
/// nothing on disk says which is live, so each skill is counted once.
fn resolve_plugin_cache(
    cache: &Path,
    origin: SkillOrigin,
    only_marketplace: Option<&str>,
    is_enabled: &dyn Fn(&str, &str) -> Option<bool>,
) -> Vec<(String, Vec<ResolvedSkill>)> {
    let mut packs: Vec<(String, Vec<ResolvedSkill>)> = Vec::new();

    let Ok(marketplaces) = fs::read_dir(cache) else {
        return packs;
    };
    let mut marketplaces: Vec<PathBuf> = marketplaces.flatten().map(|e| e.path()).collect();
    marketplaces.sort();

    for marketplace_dir in marketplaces {
        let Some(marketplace) = marketplace_dir.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if only_marketplace.is_some_and(|wanted| wanted != marketplace) {
            continue;
        }
        let Ok(plugins) = fs::read_dir(&marketplace_dir) else {
            continue;
        };
        let mut plugins: Vec<PathBuf> = plugins.flatten().map(|e| e.path()).collect();
        plugins.sort();

        for plugin_dir in plugins {
            let Some(plugin) = plugin_dir.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if is_enabled(plugin, marketplace) == Some(false) {
                continue;
            }

            let mut seen: HashSet<String> = HashSet::new();
            let mut skills: Vec<ResolvedSkill> = Vec::new();

            let Ok(versions) = fs::read_dir(&plugin_dir) else {
                continue;
            };
            let mut versions: Vec<PathBuf> = versions.flatten().map(|e| e.path()).collect();
            versions.sort();

            for version_dir in versions {
                for skill_dir in skill_dirs(&version_dir.join("skills")) {
                    let Some(facts) = read_skill_facts(&skill_dir) else {
                        continue;
                    };
                    if !seen.insert(facts.folder_name.clone()) {
                        continue;
                    }
                    let id = format!("{}:{}", plugin, facts.folder_name);
                    skills.push(ResolvedSkill {
                        token_estimate: estimate_tokens(&id, &facts.description),
                        id,
                        folder_name: facts.folder_name,
                        origin,
                        source_label: plugin.to_string(),
                        model_facing: facts.model_facing,
                        // A bare-name override never matches `plugin:skill`.
                        vetoed_by: None,
                        path: skill_dir.to_string_lossy().to_string(),
                    });
                }
            }

            if !skills.is_empty() {
                skills.sort_by(|a, b| a.id.cmp(&b.id));
                packs.push((plugin.to_string(), skills));
            }
        }
    }

    packs
}

/// Account packs are read from a local cache that the desktop app maintains.
/// It can be stale *and* incomplete: skills observed loading in a live session
/// have been found in no local file at all. The listing is therefore indicative,
/// and must never be presented as the definitive set.
const ACCOUNT_CAVEAT: &str = "Delivered by the desktop app, where they are also switched \
     on and off. Kit reads the local cache, which can be stale or incomplete, so treat \
     this as indicative rather than the full list.";

/// Without `installed_plugins.json` the cache is all Kit has, and the cache
/// remembers everything ever downloaded. Listed, but kept out of the totals.
const STALE_CACHE_CAVEAT: &str = "Kit could not read \
     `~/.claude/plugins/installed_plugins.json`, so this is the plugin cache — \
     which keeps old versions and uninstalled plugins. Treat it as what has been \
     downloaded, not what is loading.";

fn group(
    origin: SkillOrigin,
    label: impl Into<String>,
    skills: Vec<ResolvedSkill>,
    controllable: bool,
    enablement_unknown: bool,
    caveat: Option<&str>,
) -> LoadoutGroup {
    let live: Vec<&ResolvedSkill> = skills.iter().filter(|s| s.vetoed_by.is_none()).collect();
    LoadoutGroup {
        origin,
        label: label.into(),
        model_facing_count: live.iter().filter(|s| s.model_facing).count(),
        command_only_count: live.iter().filter(|s| !s.model_facing).count(),
        token_estimate: live
            .iter()
            .filter(|s| s.model_facing)
            .map(|s| s.token_estimate)
            .sum(),
        controllable,
        enablement_unknown,
        caveat: caveat.map(str::to_string),
        skills,
    }
}

/// Build the full picture for one location.
pub fn resolve(location: &SavedLocation, home: &Path) -> SessionLoadout {
    let settings = read_claude_settings(home);
    let mut groups: Vec<LoadoutGroup> = Vec::new();

    // 1. The project's own skills, when a project is selected. Resolved first
    //    because a project skill and a global one of the same name are one
    //    skill in the session, not two, and the project copy is the nearer one.
    let mut project_names: BTreeSet<String> = BTreeSet::new();
    let mut project_group = None;
    if !location.is_global() {
        let project_dir = scanner::skills_dir_for(Path::new(&location.path), location.kind);
        let project = project_dir
            .map(|d| resolve_folder(&d, SkillOrigin::Project, &settings))
            .unwrap_or_default();
        if !project.is_empty() {
            project_names.extend(project.iter().map(|s| s.folder_name.clone()));
            project_group = Some(group(
                SkillOrigin::Project,
                location.label.clone(),
                project,
                true,
                false,
                None,
            ));
        }
    }

    // 2. Global — always in play, whichever location is selected, except where
    //    the project shadows it. Counting both would inflate every total.
    let global_dir = home.join(".claude").join("skills");
    let global: Vec<ResolvedSkill> = resolve_folder(&global_dir, SkillOrigin::Global, &settings)
        .into_iter()
        .filter(|s| !project_names.contains(&s.folder_name))
        .collect();
    groups.push(group(SkillOrigin::Global, "Global", global, true, false, None));
    groups.extend(project_group);

    // 3. Local plugins. `installed_plugins.json` is the record of what is
    //    actually installed and at which version; the cache is only what has
    //    been downloaded. Where the record is unreadable, the cache is listed
    //    but kept out of the totals rather than passed off as the truth.
    match read_installed_plugins(home) {
        Some(installed) => {
            for plugin in installed {
                let key = format!("{}@{}", plugin.plugin, plugin.marketplace);
                if settings.enabled_plugins.get(&key) == Some(&false) {
                    continue;
                }
                let skills = resolve_installed_plugin(&plugin);
                if !skills.is_empty() {
                    groups.push(group(
                        SkillOrigin::Plugin,
                        plugin.plugin.clone(),
                        skills,
                        true,
                        false,
                        None,
                    ));
                }
            }
        }
        None => {
            let enabled = settings.enabled_plugins.clone();
            let plugin_gate = move |plugin: &str, marketplace: &str| -> Option<bool> {
                enabled.get(&format!("{plugin}@{marketplace}")).copied()
            };
            for (plugin, skills) in resolve_plugin_cache(
                &home.join(".claude").join("plugins").join("cache"),
                SkillOrigin::Plugin,
                None,
                &plugin_gate,
            ) {
                groups.push(group(
                    SkillOrigin::Plugin,
                    plugin,
                    skills,
                    true,
                    true,
                    Some(STALE_CACHE_CAVEAT),
                ));
            }
        }
    }

    // 4. Account-level packs. `~/.codex/plugins/cache` is the Codex CLI's own
    //    cache and holds eleven marketplaces, only one of which reaches a Claude
    //    session — scanning the lot would report Codex's plugins as Claude
    //    skills. Nothing on disk records which packs are switched on either, so
    //    these are listed as present and kept out of the totals.
    for (pack, skills) in resolve_plugin_cache(
        &home.join(".codex").join("plugins").join("cache"),
        SkillOrigin::Account,
        Some(COWORK_MARKETPLACE),
        &|_, _| None,
    ) {
        groups.push(group(
            SkillOrigin::Account,
            pack,
            skills,
            false,
            true,
            Some(ACCOUNT_CAVEAT),
        ));
    }

    // Conflicts worth surfacing.
    let vetoed: Vec<ResolvedSkill> = groups
        .iter()
        .flat_map(|g| g.skills.iter())
        .filter(|s| s.vetoed_by.is_some())
        .cloned()
        .collect();

    let on_disk: BTreeSet<&str> = groups
        .iter()
        .flat_map(|g| g.skills.iter())
        .map(|s| s.folder_name.as_str())
        .collect();
    let bare_named: BTreeSet<&str> = groups
        .iter()
        .flat_map(|g| g.skills.iter())
        .filter(|s| matches!(s.origin, SkillOrigin::Global | SkillOrigin::Project))
        .map(|s| s.folder_name.as_str())
        .collect();

    let mut dead_overrides = Vec::new();
    let mut unreachable_overrides = Vec::new();
    for key in &settings.overrides_off {
        if bare_named.contains(key.as_str()) {
            continue; // doing its job
        }
        if on_disk.contains(key.as_str()) {
            // Exists, but only under a `plugin:skill` name the key cannot match.
            unreachable_overrides.push(key.clone());
        } else {
            dead_overrides.push(key.clone());
        }
    }

    let counted = groups.iter().filter(|g| !g.enablement_unknown);
    SessionLoadout {
        location_id: location.id.clone(),
        location_label: location.label.clone(),
        model_facing_count: counted.clone().map(|g| g.model_facing_count).sum(),
        command_only_count: counted.clone().map(|g| g.command_only_count).sum(),
        token_estimate: counted.map(|g| g.token_estimate).sum(),
        vetoed,
        dead_overrides,
        unreachable_overrides,
        settings_unreadable: settings.unreadable,
        groups,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_skill(dir: &Path, name: &str, description: &str, extra: &str) {
        let d = dir.join(name);
        fs::create_dir_all(&d).unwrap();
        fs::write(
            d.join("SKILL.md"),
            format!("---\nname: {name}\ndescription: {description}\n{extra}---\nBody\n"),
        )
        .unwrap();
    }

    fn fixture(tag: &str) -> PathBuf {
        let home = std::env::temp_dir().join(format!("kit-resolver-{tag}-{}", std::process::id()));
        fs::remove_dir_all(&home).ok();
        fs::create_dir_all(home.join(".claude").join("skills")).unwrap();
        home
    }

    fn global_location(home: &Path) -> SavedLocation {
        SavedLocation {
            id: "__global__".into(),
            label: "Global".into(),
            path: home.join(".claude").join("skills").to_string_lossy().to_string(),
            notes: None,
            last_synced_at: None,
            kind: LocationKind::Global,
        }
    }

    /// The veto is the whole point: a skill switched off in `skillOverrides`
    /// must be reported as not loading, even though its folder is right there.
    #[test]
    fn an_overridden_skill_is_reported_as_vetoed_and_left_out_of_the_cost() {
        let home = fixture("veto");
        let skills = home.join(".claude").join("skills");
        write_skill(&skills, "alpha", "Alpha does things.", "");
        write_skill(&skills, "beta", "Beta does other things.", "");
        fs::write(
            home.join(".claude").join("settings.json"),
            r#"{"skillOverrides":{"beta":"off"}}"#,
        )
        .unwrap();

        let out = resolve(&global_location(&home), &home);

        assert_eq!(out.model_facing_count, 1, "only alpha actually loads");
        assert_eq!(out.vetoed.len(), 1);
        assert_eq!(out.vetoed[0].folder_name, "beta");
        assert!(out.dead_overrides.is_empty());

        let global = &out.groups[0];
        assert_eq!(global.model_facing_count, 1);
        assert!(
            global.token_estimate > 0 && global.token_estimate < 40,
            "cost counts alpha only, got {}",
            global.token_estimate
        );

        fs::remove_dir_all(&home).ok();
    }

    /// Command-only skills never reach the model's list, so they must not be
    /// counted as context cost.
    #[test]
    fn command_only_skills_are_separated_from_model_facing_cost() {
        let home = fixture("cmdonly");
        let skills = home.join(".claude").join("skills");
        write_skill(&skills, "visible", "Loads into the model list.", "");
        write_skill(
            &skills,
            "slash-only",
            "Only reachable as a command.",
            "disable-model-invocation: true\n",
        );

        let out = resolve(&global_location(&home), &home);

        assert_eq!(out.model_facing_count, 1);
        assert_eq!(out.command_only_count, 1);
        let only = out.groups[0]
            .skills
            .iter()
            .find(|s| s.folder_name == "slash-only")
            .unwrap();
        assert!(!only.model_facing);

        fs::remove_dir_all(&home).ok();
    }

    /// An override naming nothing on disk is dead weight; one naming a skill
    /// that exists only under a `plugin:skill` name can never bite. Both are
    /// worth telling the user about, and they are different problems.
    #[test]
    fn dead_and_unreachable_overrides_are_told_apart() {
        let home = fixture("overrides");
        let skills = home.join(".claude").join("skills");
        write_skill(&skills, "present", "A real one.", "");

        let plugin = home
            .join(".claude")
            .join("plugins")
            .join("cache")
            .join("market")
            .join("somepack")
            .join("1.0.0")
            .join("skills");
        fs::create_dir_all(&plugin).unwrap();
        write_skill(&plugin, "namespaced", "Only exists inside a plugin.", "");

        fs::write(
            home.join(".claude").join("settings.json"),
            r#"{"skillOverrides":{"present":"off","namespaced":"off","ghost":"off"}}"#,
        )
        .unwrap();

        let out = resolve(&global_location(&home), &home);

        assert_eq!(out.dead_overrides, vec!["ghost".to_string()]);
        assert_eq!(out.unreachable_overrides, vec!["namespaced".to_string()]);
        assert_eq!(
            out.vetoed.iter().map(|s| s.folder_name.as_str()).collect::<Vec<_>>(),
            vec!["present"]
        );

        fs::remove_dir_all(&home).ok();
    }

    /// Write `installed_plugins.json` naming exactly the given installs.
    fn write_installed(home: &Path, installs: &[(&str, &str, PathBuf)]) {
        let plugins: serde_json::Map<String, serde_json::Value> = installs
            .iter()
            .map(|(plugin, marketplace, path)| {
                (
                    format!("{plugin}@{marketplace}"),
                    serde_json::json!([{
                        "scope": "user",
                        "installPath": path.to_string_lossy(),
                    }]),
                )
            })
            .collect();
        fs::write(
            home.join(".claude").join("plugins").join("installed_plugins.json"),
            serde_json::json!({ "version": 2, "plugins": plugins }).to_string(),
        )
        .unwrap();
    }

    /// The cache keeps every plugin ever downloaded. On a real machine that was
    /// 8 installed against 21 leftovers — all of which used to be reported as
    /// loading, because a plugin absent from `enabledPlugins` was read as "on".
    #[test]
    fn cached_plugins_that_are_not_installed_do_not_count_as_loading() {
        let home = fixture("uninstalled");
        let cache = home.join(".claude").join("plugins").join("cache").join("market");
        for plugin in ["live", "leftover"] {
            let dir = cache.join(plugin).join("1.0.0").join("skills");
            fs::create_dir_all(&dir).unwrap();
            write_skill(&dir, &format!("{plugin}-skill"), "A plugin skill.", "");
        }
        write_installed(
            &home,
            &[("live", "market", cache.join("live").join("1.0.0"))],
        );

        let out = resolve(&global_location(&home), &home);
        let labels: Vec<&str> = out.groups.iter().map(|g| g.label.as_str()).collect();

        assert!(labels.contains(&"live"), "installed plugin missing: {labels:?}");
        assert!(
            !labels.contains(&"leftover"),
            "a cached-but-uninstalled plugin was reported as loading: {labels:?}"
        );
        assert_eq!(out.model_facing_count, 1, "totals must count the live one only");

        fs::remove_dir_all(&home).ok();
    }

    /// Cache version folders are often content hashes, where "newest" cannot be
    /// read off the name at all. `installed_plugins.json` names the live one, so
    /// the obsolete version's skills must not leak into the answer.
    #[test]
    fn only_the_installed_version_of_a_plugin_is_read() {
        let home = fixture("version-pick");
        let plugin = home
            .join(".claude")
            .join("plugins")
            .join("cache")
            .join("market")
            .join("pack");
        // `aaa111` sorts first; `zzz999` is the one actually installed.
        for (version, skill) in [("aaa111", "old-skill"), ("zzz999", "current-skill")] {
            let dir = plugin.join(version).join("skills");
            fs::create_dir_all(&dir).unwrap();
            write_skill(&dir, skill, "A plugin skill.", "");
        }
        write_installed(&home, &[("pack", "market", plugin.join("zzz999"))]);

        let out = resolve(&global_location(&home), &home);
        let g = out.groups.iter().find(|g| g.label == "pack").unwrap();
        let ids: Vec<&str> = g.skills.iter().map(|s| s.id.as_str()).collect();

        assert_eq!(ids, vec!["pack:current-skill"], "wrong version won: {ids:?}");

        fs::remove_dir_all(&home).ok();
    }

    /// Without the record of what is installed, the cache is all there is — and
    /// it is not good enough to count. Listed with a caveat, kept out of totals.
    #[test]
    fn without_the_installed_record_plugins_are_listed_but_not_counted() {
        let home = fixture("no-record");
        let dir = home
            .join(".claude")
            .join("plugins")
            .join("cache")
            .join("market")
            .join("pack")
            .join("1.0.0")
            .join("skills");
        fs::create_dir_all(&dir).unwrap();
        write_skill(&dir, "some-skill", "A plugin skill.", "");

        let out = resolve(&global_location(&home), &home);
        let g = out.groups.iter().find(|g| g.label == "pack").unwrap();

        assert!(g.enablement_unknown, "unverified plugins must not be counted");
        assert!(g.caveat.is_some(), "and the UI must be told why");
        assert_eq!(out.model_facing_count, 0);

        fs::remove_dir_all(&home).ok();
    }

    /// The same skill name in Global and in the project is one skill in the
    /// session. Counting both inflated every total on the screen.
    #[test]
    fn a_skill_in_both_global_and_the_project_is_counted_once() {
        let home = fixture("shadow");
        let global = home.join(".claude").join("skills");
        write_skill(&global, "shared", "In both places.", "");
        write_skill(&global, "global-only", "Only global.", "");

        let project = home.join("code").join("app");
        let project_skills = project.join(".claude").join("skills");
        fs::create_dir_all(&project_skills).unwrap();
        write_skill(&project_skills, "shared", "In both places.", "");

        let location = SavedLocation {
            id: "proj".into(),
            label: "App".into(),
            path: project.to_string_lossy().to_string(),
            notes: None,
            last_synced_at: None,
            kind: LocationKind::Project,
        };
        let out = resolve(&location, &home);

        assert_eq!(
            out.model_facing_count, 2,
            "shared must be counted once, not once per scope"
        );
        let global_group = out
            .groups
            .iter()
            .find(|g| g.origin == SkillOrigin::Global)
            .unwrap();
        let names: Vec<&str> = global_group
            .skills
            .iter()
            .map(|s| s.folder_name.as_str())
            .collect();
        assert_eq!(
            names,
            vec!["global-only"],
            "the project's copy is the nearer one and shadows Global"
        );

        fs::remove_dir_all(&home).ok();
    }

    /// Settings that exist but will not parse leave every veto unknown. Saying
    /// "no overrides" there is a claim Kit cannot support.
    #[test]
    fn unreadable_settings_are_reported_as_unknown_not_as_nothing_configured() {
        let home = fixture("bad-settings");
        write_skill(&home.join(".claude").join("skills"), "alpha", "A skill.", "");
        fs::write(home.join(".claude").join("settings.json"), "{ not json").unwrap();

        let out = resolve(&global_location(&home), &home);
        assert!(out.settings_unreadable);

        // A readable file is not flagged.
        fs::write(home.join(".claude").join("settings.json"), "{}").unwrap();
        assert!(!resolve(&global_location(&home), &home).settings_unreadable);

        fs::remove_dir_all(&home).ok();
    }

    /// A disabled plugin contributes nothing. Covers the cache fallback, which
    /// is what runs when `installed_plugins.json` is absent.
    #[test]
    fn disabled_plugins_are_excluded() {
        let home = fixture("plugins");
        let base = home.join(".claude").join("plugins").join("cache").join("market");
        for (plugin, enabled) in [("on", true), ("off", false)] {
            let dir = base.join(plugin).join("1.0.0").join("skills");
            fs::create_dir_all(&dir).unwrap();
            write_skill(&dir, &format!("{plugin}-skill"), "A plugin skill.", "");
            let _ = enabled;
        }
        fs::write(
            home.join(".claude").join("settings.json"),
            r#"{"enabledPlugins":{"on@market":true,"off@market":false}}"#,
        )
        .unwrap();

        let out = resolve(&global_location(&home), &home);
        let labels: Vec<&str> = out.groups.iter().map(|g| g.label.as_str()).collect();

        assert!(labels.contains(&"on"), "enabled plugin missing: {labels:?}");
        assert!(!labels.contains(&"off"), "disabled plugin included: {labels:?}");
        assert_eq!(
            out.groups.iter().find(|g| g.label == "on").unwrap().skills[0].id,
            "on:on-skill",
            "plugin skills are addressed as plugin:skill"
        );

        fs::remove_dir_all(&home).ok();
    }

    /// Several versions of one plugin are cached side by side; a skill present
    /// in each must be counted once, not once per version.
    #[test]
    fn repeated_plugin_versions_do_not_inflate_the_count() {
        let home = fixture("versions");
        let plugin = home.join(".claude").join("plugins").join("cache").join("market").join("dup");
        for version in ["1.0.0", "2.0.0", "abc123"] {
            let dir = plugin.join(version).join("skills");
            fs::create_dir_all(&dir).unwrap();
            write_skill(&dir, "same", "The same skill, cached thrice.", "");
        }

        let out = resolve(&global_location(&home), &home);
        let g = out.groups.iter().find(|g| g.label == "dup").unwrap();
        assert_eq!(g.skills.len(), 1);
        assert_eq!(g.model_facing_count, 1);

        fs::remove_dir_all(&home).ok();
    }

    /// Nothing on disk says which account packs are switched on, so they are
    /// listed but kept out of the totals rather than counted as loading.
    #[test]
    fn account_packs_are_listed_but_not_counted() {
        let home = fixture("account");
        let pack = home
            .join(".codex")
            .join("plugins")
            .join("cache")
            .join("claude-cowork")
            .join("small-business")
            .join("1.0.0")
            .join("skills");
        fs::create_dir_all(&pack).unwrap();
        write_skill(&pack, "invoicing", "An account-level skill.", "");

        let out = resolve(&global_location(&home), &home);
        let g = out
            .groups
            .iter()
            .find(|g| g.origin == SkillOrigin::Account)
            .expect("account pack should be listed");

        assert!(g.enablement_unknown);
        assert!(!g.controllable, "Kit cannot toggle these from local files");
        assert!(
            g.caveat.is_some(),
            "the listing is indicative, and the UI must be told so"
        );
        assert_eq!(g.model_facing_count, 1);
        assert_eq!(out.model_facing_count, 0, "must not inflate the real total");
        assert_eq!(out.token_estimate, 0);

        fs::remove_dir_all(&home).ok();
    }

    /// `~/.codex/plugins/cache` is the Codex CLI's own cache and holds many
    /// marketplaces. Only `claude-cowork` reaches a Claude session; counting the
    /// rest would report Codex's plugins as Claude skills.
    #[test]
    fn only_the_cowork_marketplace_counts_as_account_level() {
        let home = fixture("cowork-scope");
        for (marketplace, pack) in [
            ("claude-cowork", "small-business"),
            ("openai-curated", "gmail"),
            ("openai-bundled", "browser"),
        ] {
            let dir = home
                .join(".codex")
                .join("plugins")
                .join("cache")
                .join(marketplace)
                .join(pack)
                .join("1.0.0")
                .join("skills");
            fs::create_dir_all(&dir).unwrap();
            write_skill(&dir, &format!("{pack}-skill"), "A packaged skill.", "");
        }

        let out = resolve(&global_location(&home), &home);
        let account: Vec<&str> = out
            .groups
            .iter()
            .filter(|g| g.origin == SkillOrigin::Account)
            .map(|g| g.label.as_str())
            .collect();

        assert_eq!(
            account,
            vec!["small-business"],
            "only claude-cowork packs are account-level, got {account:?}"
        );

        fs::remove_dir_all(&home).ok();
    }

    /// Environment smoke test: run the resolver over the real home and print
    /// what it thinks loads. Ignored by default because the numbers are
    /// machine-specific. Run with:
    /// `cargo test -- --ignored loadout_on_this_machine --nocapture`
    #[test]
    #[ignore]
    fn loadout_on_this_machine() {
        let home = dirs::home_dir().expect("home");
        // Point KIT_LOADOUT_PROJECT at a project directory to resolve that
        // instead of Global, so the output can be compared with a real session
        // opened in the same place.
        let location = match std::env::var("KIT_LOADOUT_PROJECT") {
            Ok(path) if !path.trim().is_empty() => SavedLocation {
                id: "probe".into(),
                label: path.rsplit('/').next().unwrap_or("project").to_string(),
                path,
                notes: None,
                last_synced_at: None,
                kind: LocationKind::Project,
            },
            _ => global_location(&home),
        };
        let out = resolve(&location, &home);

        println!("model-facing: {}", out.model_facing_count);
        println!("command-only: {}", out.command_only_count);
        println!("tokens: ~{}", out.token_estimate);
        for g in &out.groups {
            println!(
                "  [{}] {:<28} {:>3} skills  ~{:>5} tokens{}",
                match g.origin {
                    SkillOrigin::Global => "global ",
                    SkillOrigin::Project => "project",
                    SkillOrigin::Plugin => "plugin ",
                    SkillOrigin::Account => "account",
                },
                g.label,
                g.model_facing_count,
                g.token_estimate,
                if g.enablement_unknown { "  (not counted)" } else { "" }
            );
        }
        println!("vetoed: {}", out.vetoed.len());
        println!("dead overrides: {:?}", out.dead_overrides);
        println!("unreachable overrides: {:?}", out.unreachable_overrides);
    }
}
