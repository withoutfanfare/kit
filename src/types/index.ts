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
  defaultView: "locations" | "skills";
  showArchived: boolean;
  locations: SavedLocationSummary[];
  counts: {
    skills: number;
    sets: number;
    archivedSkills: number;
    brokenLinks: number;
  };
};

export type SavedLocationSummary = {
  id: LocationId;
  label: string;
  path: string;
  issueCount: number;
  installedSkillCount: number;
  installedSetCount: number;
  lastSyncedAt: string | null;
};

export type DetectedProjectType = {
  name: string;
  markerFile: string;
};

export type SkillRecommendation = {
  skillId: SkillId;
  skillName: string;
  reason: string;
};

export type LocationDetail = {
  id: LocationId;
  label: string;
  path: string;
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
  recentlyUsed: Array<{ id: SkillId; name: string; lastUsedAt: string }>;
  unused: Array<{ id: SkillId; name: string }>;
  suggestions: string[];
};

export type Preferences = {
  libraryRoot: string;
  editorCommand: string;
  defaultView: "locations" | "skills";
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

export type HealthCheckResult = {
  issues: HealthIssue[];
  locationCount: number;
  healthyCount: number;
  warningCount: number;
  errorCount: number;
  scannedAt: string;
};

// Export/import types
export type ImportPreview = {
  skills: Array<{ id: SkillId; name: string; alreadyExists: boolean }>;
  setDefinition: { name: string; description: string | null; skills: string[] } | null;
  conflictCount: number;
};

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
  assignedLocationCount: number;
};

// Watcher status
export type WatcherStatus = "active" | "paused" | "error" | "stopped";

export type WatcherStatusResponse = {
  status: WatcherStatus;
  watchedPath: string | null;
};
