export type LocationId = string;
export type SkillId = string;
export type SetId = string;

export type LinkState =
  | "linked"
  | "declared_only"
  | "local_only"
  | "broken_link";

export type AppBootstrap = {
  libraryRoot: string;
  editorCommand: string;
  defaultView: "panel" | "locations" | "skills";
  showArchived: boolean;
  locations: SavedLocationSummary[];
  counts: {
    skills: number;
    sets: number;
    archivedSkills: number;
    brokenLinks: number;
  };
};

/**
 * `global` is `~/.claude/skills` — the folder Claude Code reads for skills that
 * load in every session. It holds its skills directly rather than under
 * `.claude/skills`, and it has no manifest.
 */
export type LocationKind = "global" | "project";

export type SavedLocationSummary = {
  id: LocationId;
  label: string;
  path: string;
  kind: LocationKind;
  /** False when the directory has been moved or deleted since it was saved. */
  pathExists: boolean;
  issueCount: number;
  installedSkillCount: number;
  installedSetCount: number;
  lastSyncedAt: string | null;
};

/** Where a skill reaching a session came from. */
export type SkillOrigin = "global" | "project" | "plugin" | "account";

export type ResolvedSkill = {
  /** How the skill is addressed in a session: bare, or `plugin:skill`. */
  id: string;
  /** The folder name — what a `skillOverrides` key must match. */
  folderName: string;
  origin: SkillOrigin;
  /** Plugin or pack it came from; empty for global and project. */
  sourceLabel: string;
  /** False for command-only skills, which never enter the model's list. */
  modelFacing: boolean;
  /** The `skillOverrides` key that switched this off, if any. */
  vetoedBy: string | null;
  tokenEstimate: number;
  path: string;
};

export type LoadoutGroup = {
  origin: SkillOrigin;
  label: string;
  modelFacingCount: number;
  commandOnlyCount: number;
  tokenEstimate: number;
  /** False when Kit can't change this by moving symlinks. */
  controllable: boolean;
  /** True when nothing on disk says whether these are switched on. */
  enablementUnknown: boolean;
  caveat: string | null;
  skills: ResolvedSkill[];
};

export type SessionLoadout = {
  locationId: LocationId;
  locationLabel: string;
  groups: LoadoutGroup[];
  modelFacingCount: number;
  commandOnlyCount: number;
  tokenEstimate: number;
  /** Linked here but switched off globally — looks active, isn't. */
  vetoed: ResolvedSkill[];
  /** Overrides naming a skill found in none of the places Kit looked. */
  deadOverrides: string[];
  /** Overrides that can't bite because the skill is only `plugin:skill`. */
  unreachableOverrides: string[];
  /** `~/.claude/settings.json` is there but unreadable, so vetoes are unknown. */
  settingsUnreadable: boolean;
};

export type UsageProjectCount = { name: string; runs: number };

export type UsageSkillRow = {
  skill: string;
  runs: number;
  lastUsedAt: string | null;
  /** False for plugin and account skills, which the library doesn't hold. */
  inLibrary: boolean;
  projects: UsageProjectCount[];
};

export type UsageReport = {
  /** False when no logs were found — not the same as nothing being used. */
  available: boolean;
  eventCount: number;
  recordedSince: string | null;
  distinctSkills: number;
  rows: UsageSkillRow[];
  /** Library skills with no recorded run at all. */
  neverUsed: string[];
};

export type LocationUsageRow = {
  skillId: SkillId;
  name: string;
  /** Invocations recorded inside this location's directory. */
  usesHere: number;
  /** Invocations anywhere in the last 30 days. */
  usesAnywhere: number;
  lastUsedAt: string | null;
};

export type LocationUsage = {
  locationId: LocationId;
  locationLabel: string;
  /** False when no logs were found — not the same as nothing being used. */
  available: boolean;
  recordedSince: string | null;
  eventCount: number;
  linkedCount: number;
  usedHereCount: number;
  rows: LocationUsageRow[];
};

/** A project on disk that keeps Claude skills but isn't tracked yet. */
export type DiscoveredLocation = {
  path: string;
  label: string;
  skillCount: number;
};

export type DetectedProjectType = {
  name: string;
  markerFile: string;
};

export type SkillRecommendation = {
  skillId: SkillId;
  skillName: string;
  projectType: string;
  reason: string | null;
};

export type LocationDetail = {
  id: LocationId;
  label: string;
  path: string;
  kind: LocationKind;
  manifestPath: string | null;
  notes: string | null;
  sets: SetAssignment[];
  skills: SkillAssignment[];
  issues: LocationIssue[];
  stats: {
    linkedCount: number;
    localOnlyCount: number;
    brokenCount: number;
  };
  detectedProjectTypes: DetectedProjectType[];
  skillRecommendations: SkillRecommendation[];
  lastScannedAt: string | null;
};

export type SetAssignment = {
  setId: SetId;
  name: string;
  skillCount: number;
  path: string;
};

export type SkillAssignment = {
  skillId: SkillId;
  name: string;
  path: string;
  linkState: LinkState;
  declaredInManifest: boolean;
  archived: boolean;
  source: "library" | "local";
  disabled: boolean;
};

export type LocationIssue = {
  kind: "broken_link" | "declared_missing" | "linked_undeclared" | "stale" | "missing_set";
  skillName: string;
  skillId: SkillId | null;
  message: string;
};

export type SkillDetail = {
  id: SkillId;
  name: string;
  path: string;
  archived: boolean;
  summary: string | null;
  linkedLocations: SavedLocationSummary[];
  includedInSets: Array<{ id: SetId; name: string }>;
  usage: {
    lastUsedAt: string | null;
    useCount30d: number;
  };
};

export type ValidationSeverity = "error" | "warning";

export type ValidationIssue = {
  field: string;
  message: string;
  suggestion: string;
  severity: ValidationSeverity;
};

export type LibraryListItem = {
  id: string;
  name: string;
  kind: "skill" | "set";
  archived: boolean;
  summary: string | null;
  linkedLocationCount: number;
  useCount30d: number;
  lastUsedAt: string | null;
  isUnusedEverywhere: boolean;
  tags: string[];
  validationIssues: ValidationIssue[];
  brokenSkillCount: number;
};

export type PreviewChange = {
  kind: "add_link" | "remove_link" | "manifest_add" | "manifest_remove";
  skillName: string;
  detail: string;
};

export type AssignmentPreview = {
  locationId: LocationId;
  adds: PreviewChange[];
  removes: PreviewChange[];
  manifestUpdates: PreviewChange[];
  warnings: string[];
};

export type SetScope = "global" | "project";

export type SetSummary = {
  id: SetId;
  name: string;
  description: string | null;
  scope: SetScope;
  ownerLocationId: LocationId | null;
  skillCount: number;
  assignedLocationCount: number;
  path: string;
};

export type SetDetail = {
  id: SetId;
  name: string;
  description: string | null;
  scope: SetScope;
  ownerLocationId: LocationId | null;
  path: string;
  skills: Array<{ id: SkillId; name: string; archived: boolean; missing: boolean }>;
  assignedLocations: SavedLocationSummary[];
};

export type UsageSummary = {
  mostUsed: Array<{ id: SkillId; name: string; count: number }>;
  recentlyUsed: Array<{ id: SkillId; name: string; lastUsedAt: string | null }>;
  unused: Array<{ id: SkillId; name: string }>;
  suggestions: string[];
};

export type Preferences = {
  libraryRoot: string;
  editorCommand: string;
  defaultView: "panel" | "locations" | "skills";
  showArchived: boolean;
  trackSkillVersions: boolean;
};

export type UpdatePreferencesInput = Partial<Preferences>;

export type SkillsRepoValidation = {
  valid: boolean;
  path: string;
  isGitRepo: boolean;
  detectedBranch: string | null;
  skillCount: number;
  issues: string[];
};

export type RepoState = "up_to_date" | "behind" | "ahead" | "diverged" | "dirty" | "unavailable";

export type SkillsRepoStatus = {
  path: string;
  branch: string | null;
  upstream: string | null;
  state: RepoState;
  aheadBy: number;
  behindBy: number;
  hasUncommittedChanges: boolean;
  lastCheckedAt: string | null;
  message: string;
};

// Health check types
export type HealthIssueSeverity = "error" | "warning" | "info";

export type HealthIssue = {
  severity: HealthIssueSeverity;
  locationId: LocationId;
  locationLabel: string;
  description: string;
  suggestion: string;
  autoFixable: boolean;
  skillId: SkillId | null;
  kind: LocationIssue["kind"];
};

export type HealthLocationSummary = {
  locationId: LocationId;
  locationLabel: string;
  errorCount: number;
  warningCount: number;
  infoCount: number;
  brokenLinkCount: number;
};

export type BrokenLinkRemovalPreview = {
  locationId: LocationId;
  locationLabel: string;
  paths: string[];
};

export type BrokenLinkRemovalResult = {
  removedCount: number;
  failures: Array<{ path: string; error: string }>;
  health: HealthCheckResult;
};

export type HealthCheckResult = {
  issues: HealthIssue[];
  locations: HealthLocationSummary[];
  locationCount: number;
  healthyCount: number;
  warningCount: number;
  errorCount: number;
  scannedAt: string;
};

// Export/import types
// Skill version tracking
export type SkillVersionInfo = {
  skillId: SkillId;
  assignedHash: string | null;
  currentHash: string | null;
  hasChanged: boolean;
  assignedAt: string | null;
};

// Bulk assignment types
export type BulkAssignResult = {
  locationId: LocationId;
  locationLabel: string;
  success: boolean;
  error: string | null;
};

// Changelog types
export type ChangelogEntry = {
  skillId: SkillId;
  name: string;
  modifiedAt: string;
  sizeBytes: number;
  assignedLocations: Array<{
    id: LocationId;
    label: string;
  }>;
};

// Skill content diff
export type SkillContentDiff = {
  skillId: SkillId;
  assignedContent: string | null;
  currentContent: string | null;
  hasChanged: boolean;
};

// Location comparison types
export type ComparisonSide = {
  id: LocationId;
  label: string;
  path: string;
  totalSkills: number;
};

export type ComparedSkill = {
  skillId: SkillId;
  name: string;
  linkState: LinkState;
  source: "library" | "local";
};

export type SharedSkill = {
  skillId: SkillId;
  name: string;
  linkStateA: LinkState;
  linkStateB: LinkState;
  versionDiffers: boolean;
};

export type LocationComparison = {
  locationA: ComparisonSide;
  locationB: ComparisonSide;
  onlyInA: ComparedSkill[];
  onlyInB: ComparedSkill[];
  shared: SharedSkill[];
};

// Library backup types
export type BackupResult = {
  path: string;
  skillCount: number;
  setCount: number;
  sizeBytes: number;
};

export type RestoreConflictKind = "skill_exists" | "set_exists";

export type RestoreConflict = {
  name: string;
  kind: RestoreConflictKind;
};

export type RestorePreview = {
  skillCount: number;
  setCount: number;
  hasState: boolean;
  conflicts: RestoreConflict[];
};

export type RestoreResult = {
  skillsRestored: number;
  setsRestored: number;
  skillsSkipped: number;
  setsSkipped: number;
};

// Watcher status
export type WatcherStatus = "active" | "paused" | "error" | "stopped";

export type WatcherStatusResponse = {
  status: WatcherStatus;
  watchedPath: string | null;
};
