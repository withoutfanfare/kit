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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferencesUpdate {
    pub library_root: Option<String>,
    pub editor_command: Option<String>,
    pub default_view: Option<DefaultView>,
    pub show_archived: Option<bool>,
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
    pub folder_name: String,
    pub path: String,
    /// Canonical (fully resolved, symlink-followed) path to the skill folder.
    pub canonical_path: Option<String>,
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
}
