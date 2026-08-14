use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LinkState {
    Linked,
    DeclaredOnly,
    LocalOnly,
    BrokenLink,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SkillSource {
    Library,
    Local,
}

/// What kind of place a location is.
///
/// `Global` is `~/.claude/skills` — the folder Claude Code reads for skills that
/// load in *every* session. Its skills live directly in the location path rather
/// than under `.claude/skills`, and it has no manifest: the `.claude/settings.json`
/// beside it is the user's own Claude Code settings file, not something Kit may
/// write to.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum LocationKind {
    Global,
    #[default]
    Project,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LibraryItemKind {
    Skill,
    Set,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DefaultView {
    /// The board at a glance. The default: it answers "what is the state of
    /// things" before you have to decide where to look.
    Panel,
    Locations,
    Skills,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IssueKind {
    BrokenLink,
    DeclaredMissing,
    LinkedUndeclared,
    Stale,
    MissingSet,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationIssue {
    pub field: String,
    pub message: String,
    pub suggestion: String,
    pub severity: ValidationSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PreviewChangeKind {
    AddLink,
    RemoveLink,
    ManifestAdd,
    ManifestRemove,
}

// ---------------------------------------------------------------------------
// Bootstrap
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBootstrap {
    pub library_root: String,
    pub editor_command: String,
    pub default_view: DefaultView,
    pub show_archived: bool,
    pub locations: Vec<SavedLocationSummary>,
    pub counts: BootstrapCounts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapCounts {
    pub skills: usize,
    pub sets: usize,
    pub archived_skills: usize,
    pub broken_links: usize,
}

// ---------------------------------------------------------------------------
// Preferences
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub library_root: String,
    pub editor_command: String,
    pub default_view: DefaultView,
    pub show_archived: bool,
    #[serde(default = "default_true")]
    pub track_skill_versions: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferencesUpdate {
    pub library_root: Option<String>,
    pub editor_command: Option<String>,
    pub default_view: Option<DefaultView>,
    pub show_archived: Option<bool>,
    pub track_skill_versions: Option<bool>,
}

// ---------------------------------------------------------------------------
// Location types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedLocationSummary {
    pub id: String,
    pub label: String,
    pub path: String,
    #[serde(default)]
    pub kind: LocationKind,
    /// `false` when the directory has been moved or deleted since it was saved.
    #[serde(default)]
    pub path_exists: bool,
    pub issue_count: usize,
    pub installed_skill_count: usize,
    pub installed_set_count: usize,
    pub last_synced_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationDetail {
    pub id: String,
    pub label: String,
    pub path: String,
    #[serde(default)]
    pub kind: LocationKind,
    pub manifest_path: Option<String>,
    pub notes: Option<String>,
    pub sets: Vec<SetAssignment>,
    pub skills: Vec<SkillAssignment>,
    pub issues: Vec<LocationIssue>,
    pub stats: LocationStats,
    pub detected_project_types: Vec<DetectedProjectType>,
    pub skill_recommendations: Vec<SkillRecommendation>,
    pub last_scanned_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationStats {
    pub linked_count: usize,
    pub local_only_count: usize,
    pub broken_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillAssignment {
    pub skill_id: String,
    pub name: String,
    pub path: String,
    pub link_state: LinkState,
    pub declared_in_manifest: bool,
    pub archived: bool,
    pub source: SkillSource,
    /// Whether this skill is temporarily disabled at this location.
    /// Disabled skills keep their symlink but are removed from the manifest.
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAssignment {
    pub set_id: String,
    pub name: String,
    pub skill_count: usize,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationIssue {
    pub kind: IssueKind,
    pub skill_name: String,
    pub skill_id: Option<String>,
    pub message: String,
}

// ---------------------------------------------------------------------------
// Library / Skill types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillDetail {
    pub id: String,
    pub name: String,
    pub path: String,
    pub archived: bool,
    pub summary: Option<String>,
    pub linked_locations: Vec<SavedLocationSummary>,
    pub included_in_sets: Vec<SetRef>,
    pub usage: SkillUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRef {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillUsage {
    pub last_used_at: Option<DateTime<Utc>>,
    pub use_count_30d: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryListItem {
    pub id: String,
    pub name: String,
    pub kind: LibraryItemKind,
    pub archived: bool,
    pub summary: Option<String>,
    pub linked_location_count: usize,
    pub use_count_30d: usize,
    pub last_used_at: Option<DateTime<Utc>>,
    pub is_unused_everywhere: bool,
    pub tags: Vec<String>,
    pub validation_issues: Vec<ValidationIssue>,
    pub broken_skill_count: usize,
}

// ---------------------------------------------------------------------------
// Assignment / Preview types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewChange {
    pub kind: PreviewChangeKind,
    pub skill_name: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentPreview {
    pub location_id: String,
    pub adds: Vec<PreviewChange>,
    pub removes: Vec<PreviewChange>,
    pub manifest_updates: Vec<PreviewChange>,
    pub warnings: Vec<String>,
}

// ---------------------------------------------------------------------------
// Usage types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageSummary {
    pub most_used: Vec<UsageEntry>,
    pub recently_used: Vec<RecentEntry>,
    pub unused: Vec<UnusedEntry>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageEntry {
    pub id: String,
    pub name: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentEntry {
    pub id: String,
    pub name: String,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnusedEntry {
    pub id: String,
    pub name: String,
}

// ---------------------------------------------------------------------------
// Skills repository validation / status
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillsRepoValidation {
    pub valid: bool,
    pub path: String,
    pub is_git_repo: bool,
    pub detected_branch: Option<String>,
    pub skill_count: usize,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillsRepoStatus {
    pub path: String,
    pub branch: Option<String>,
    pub upstream: Option<String>,
    pub state: RepoState,
    pub ahead_by: usize,
    pub behind_by: usize,
    pub has_uncommitted_changes: bool,
    pub last_checked_at: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepoState {
    UpToDate,
    Behind,
    Ahead,
    Diverged,
    Dirty,
    Unavailable,
}

// ---------------------------------------------------------------------------
// Health check types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HealthIssueSeverity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthIssue {
    pub severity: HealthIssueSeverity,
    pub location_id: String,
    pub location_label: String,
    pub description: String,
    pub suggestion: String,
    pub auto_fixable: bool,
    pub skill_id: Option<String>,
    pub kind: IssueKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthLocationSummary {
    pub location_id: String,
    pub location_label: String,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub broken_link_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrokenLinkRemovalPreview {
    pub location_id: String,
    pub location_label: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrokenLinkRemovalFailure {
    pub path: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrokenLinkRemovalResult {
    pub removed_count: usize,
    pub failures: Vec<BrokenLinkRemovalFailure>,
    pub health: HealthCheckResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResult {
    pub issues: Vec<HealthIssue>,
    pub locations: Vec<HealthLocationSummary>,
    pub location_count: usize,
    pub healthy_count: usize,
    pub warning_count: usize,
    pub error_count: usize,
    pub scanned_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Project-type detection types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedProjectType {
    pub name: String,
    pub marker_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillRecommendation {
    pub skill_id: String,
    pub skill_name: String,
    pub project_type: String,
    pub reason: Option<String>,
}

// ---------------------------------------------------------------------------
// Export/import types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportManifest {
    pub name: String,
    pub description: Option<String>,
    pub exported_at: DateTime<Utc>,
    pub skills: Vec<String>,
    pub set_definition: Option<SetDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportSkillEntry {
    pub id: String,
    pub name: String,
    pub already_exists: bool,
}

// ---------------------------------------------------------------------------
// Skill version tracking types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillVersionInfo {
    pub skill_id: String,
    pub assigned_hash: Option<String>,
    pub current_hash: Option<String>,
    pub has_changed: bool,
    pub assigned_at: Option<DateTime<Utc>>,
}

// ---------------------------------------------------------------------------
// Changelog types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangelogEntry {
    pub skill_id: String,
    pub name: String,
    pub modified_at: DateTime<Utc>,
    pub size_bytes: u64,
    pub assigned_locations: Vec<ChangelogAssignedLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangelogAssignedLocation {
    pub id: String,
    pub label: String,
}

// ---------------------------------------------------------------------------
// Internal / persistence types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedLocation {
    pub id: String,
    pub label: String,
    pub path: String,
    pub notes: Option<String>,
    pub last_synced_at: Option<DateTime<Utc>>,
    /// Defaults to `Project` so state files written before Global existed still load.
    #[serde(default)]
    pub kind: LocationKind,
}

impl SavedLocation {
    pub fn is_global(&self) -> bool {
        self.kind == LocationKind::Global
    }
}

// ---------------------------------------------------------------------------
// Session loadout — what actually reaches a Claude Code session
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SkillOrigin {
    /// `~/.claude/skills` — loads in every session.
    Global,
    /// `<location>/.claude/skills` — loads only in that project.
    Project,
    /// A plugin cached under `~/.claude/plugins`, gated on `enabledPlugins`.
    Plugin,
    /// An account-level pack under `~/.codex/plugins/cache/claude-cowork`.
    Account,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedSkill {
    /// How the skill is addressed in a session — bare for Global and Project,
    /// `plugin:skill` for the namespaced ones.
    pub id: String,
    /// The folder name, which is what a `skillOverrides` key must match.
    pub folder_name: String,
    pub origin: SkillOrigin,
    /// Plugin or pack this came from. Empty for Global and Project.
    pub source_label: String,
    /// `false` for `disable-model-invocation: true` — those are slash-command
    /// only, so they cost nothing in the model's skill list.
    pub model_facing: bool,
    /// The `skillOverrides` key that switched this off, if any.
    pub vetoed_by: Option<String>,
    pub token_estimate: usize,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadoutGroup {
    pub origin: SkillOrigin,
    pub label: String,
    pub model_facing_count: usize,
    pub command_only_count: usize,
    pub token_estimate: usize,
    /// `false` when Kit cannot change this by moving symlinks — account-level
    /// packs are toggled in the desktop app.
    pub controllable: bool,
    /// `true` when nothing on disk records whether these are switched on, so the
    /// listing is "present" rather than "loading".
    pub enablement_unknown: bool,
    /// Shown alongside the group when its listing cannot be taken at face value.
    pub caveat: Option<String>,
    pub skills: Vec<ResolvedSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionLoadout {
    pub location_id: String,
    pub location_label: String,
    pub groups: Vec<LoadoutGroup>,
    /// Counts and cost for the groups Kit can vouch for — excludes any group
    /// whose enablement is unknown.
    pub model_facing_count: usize,
    pub command_only_count: usize,
    pub token_estimate: usize,
    /// Skills linked into this location but switched off globally. A symlink
    /// that looks active and is not.
    pub vetoed: Vec<ResolvedSkill>,
    /// `skillOverrides` entries naming a skill found in none of the places this
    /// resolution looked: Global, your installed plugins, and — when a project
    /// is selected — that project. A skill in some *other* project is not
    /// visible here, so the UI says what was actually checked.
    pub dead_overrides: Vec<String>,
    /// Overrides that cannot bite because the skill only exists under a
    /// `plugin:skill` name, which bare-name keys never match.
    pub unreachable_overrides: Vec<String>,
    /// `~/.claude/settings.json` exists but could not be read or parsed, so the
    /// vetoes and plugin states below are unknown rather than empty.
    pub settings_unreadable: bool,
}

/// How often one skill ran inside one project.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageProjectCount {
    pub name: String,
    pub runs: usize,
}

/// One skill's whole history, reduced to what a keep-or-drop decision needs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageSkillRow {
    pub skill: String,
    pub runs: usize,
    pub last_used_at: Option<DateTime<Utc>>,
    /// `false` for plugin and account skills, which the library does not hold.
    pub in_library: bool,
    pub projects: Vec<UsageProjectCount>,
}

/// The usage log, ranked and summarised.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageReport {
    /// `false` when no logs were found — not the same as nothing being used.
    pub available: bool,
    pub event_count: usize,
    pub recorded_since: Option<DateTime<Utc>>,
    pub distinct_skills: usize,
    pub rows: Vec<UsageSkillRow>,
    /// Library skills with no recorded run at all.
    pub never_used: Vec<String>,
}

/// One linked skill at a location, with how often it has actually been used.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationUsageRow {
    pub skill_id: String,
    pub name: String,
    /// Invocations recorded inside this location's directory.
    pub uses_here: usize,
    /// Invocations anywhere in the last 30 days.
    pub uses_anywhere: usize,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// What is linked at a location set against what has been used there.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationUsage {
    pub location_id: String,
    pub location_label: String,
    /// `false` when no logs were found. Distinct from "found, but nothing used":
    /// absence of a record is not evidence of no use.
    pub available: bool,
    pub recorded_since: Option<DateTime<Utc>>,
    pub event_count: usize,
    pub linked_count: usize,
    pub used_here_count: usize,
    pub rows: Vec<LocationUsageRow>,
}

/// A project on disk that keeps Claude skills but is not yet tracked by Kit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveredLocation {
    pub path: String,
    pub label: String,
    pub skill_count: usize,
}

/// Parsed skill metadata from SKILL.md frontmatter.
#[derive(Debug, Clone)]
pub struct SkillMeta {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub archived: bool,
    pub tags: Vec<String>,
    pub folder_name: String,
    pub path: String,
    /// Canonical (fully resolved, symlink-followed) path to the skill folder.
    pub canonical_path: Option<String>,
    pub validation_issues: Vec<ValidationIssue>,
}

/// A set definition loaded from a `.set.json` file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDefinition {
    pub name: String,
    pub description: Option<String>,
    pub skills: Vec<String>,
}

// ---------------------------------------------------------------------------
// Set types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SetScope {
    Global,
    Project,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSummary {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub scope: SetScope,
    pub owner_location_id: Option<String>,
    pub skill_count: usize,
    pub assigned_location_count: usize,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDetail {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub scope: SetScope,
    pub owner_location_id: Option<String>,
    pub path: String,
    pub skills: Vec<SetSkillEntry>,
    pub assigned_locations: Vec<SavedLocationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSkillEntry {
    pub id: String,
    pub name: String,
    pub archived: bool,
    pub missing: bool,
}

// ---------------------------------------------------------------------------
// Skill content diff types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillContentDiff {
    pub skill_id: String,
    pub assigned_content: Option<String>,
    pub current_content: Option<String>,
    pub has_changed: bool,
}

// ---------------------------------------------------------------------------
// Library backup types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupManifest {
    pub version: u32,
    pub created_at: DateTime<Utc>,
    pub library_root: String,
    pub skill_count: usize,
    pub set_count: usize,
    pub checksums: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupResult {
    pub path: String,
    pub skill_count: usize,
    pub set_count: usize,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestorePreview {
    pub skill_count: usize,
    pub set_count: usize,
    pub has_state: bool,
    pub conflicts: Vec<RestoreConflict>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreConflict {
    pub name: String,
    pub kind: RestoreConflictKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RestoreConflictKind {
    SkillExists,
    SetExists,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreResult {
    pub skills_restored: usize,
    pub sets_restored: usize,
    pub skills_skipped: usize,
    pub sets_skipped: usize,
}

// ---------------------------------------------------------------------------
// Location comparison types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationComparison {
    pub location_a: ComparisonSide,
    pub location_b: ComparisonSide,
    pub only_in_a: Vec<ComparedSkill>,
    pub only_in_b: Vec<ComparedSkill>,
    pub shared: Vec<SharedSkill>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComparisonSide {
    pub id: String,
    pub label: String,
    pub path: String,
    pub total_skills: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComparedSkill {
    pub skill_id: String,
    pub name: String,
    pub link_state: LinkState,
    pub source: SkillSource,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedSkill {
    pub skill_id: String,
    pub name: String,
    pub link_state_a: LinkState,
    pub link_state_b: LinkState,
    pub version_differs: bool,
}
