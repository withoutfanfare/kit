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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LibraryItemKind {
    Skill,
    Set,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DefaultView {
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
pub struct HealthCheckResult {
    pub issues: Vec<HealthIssue>,
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
    pub reason: String,
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
pub struct ImportPreview {
    pub skills: Vec<ImportSkillEntry>,
    pub set_definition: Option<SetDefinition>,
    pub conflict_count: usize,
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
    pub assigned_location_count: usize,
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
