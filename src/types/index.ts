export type LocationId = string;
export type SkillId = string;
export type SetId = string;

export type LinkState =
  | "linked"
  | "declared_only"
  | "local_only"
  | "broken_link";

export type AppBootstrap = {
  libraryRoot: string | null;
  editorCommand: string | null;
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
  kind: "broken_link" | "declared_missing" | "linked_undeclared" | "stale";
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

export type LibraryListItem = {
  id: string;
  name: string;
  kind: "skill" | "set";
  archived: boolean;
  summary: string | null;
  linkedLocationCount: number;
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
  skills: Array<{ id: SkillId; name: string; archived: boolean }>;
  assignedLocations: SavedLocationSummary[];
};

export type UsageSummary = {
  mostUsed: Array<{ id: SkillId; name: string; count: number }>;
  recentlyUsed: Array<{ id: SkillId; name: string; lastUsedAt: string }>;
  unused: Array<{ id: SkillId; name: string }>;
  suggestions: string[];
};

export type Preferences = {
  libraryRoot: string | null;
  editorCommand: string | null;
  defaultView: "locations" | "skills";
  showArchived: boolean;
};

export type UpdatePreferencesInput = Partial<Preferences>;
