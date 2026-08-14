/**
 * A stand-in for the Rust backend, for designing the interface without it.
 *
 * Vite swaps `@tauri-apps/api/core` for this module only when `KIT_UI_FIXTURES`
 * is set, so it cannot reach a production build. Run it with:
 *
 *   KIT_UI_FIXTURES=1 npm run dev
 *
 * The data is shaped like a real machine rather than a tidy demo: a Global
 * location holding one hook-protected skill, projects at different states of
 * repair, a broken link, a skill switched off globally while a project still
 * links it, account packs Kit cannot verify, and usage logs that start
 * part-way through the library's life. Designing against tidy data is how
 * interfaces end up unable to show the truth.
 */

import type {
  AppBootstrap,
  HealthCheckResult,
  LibraryListItem,
  LocationComparison,
  LocationDetail,
  LocationUsage,
  Preferences,
  SavedLocationSummary,
  SessionLoadout,
  SetDetail,
  SetSummary,
  SkillDetail,
  UsageReport,
  UsageSummary,
} from "@/types";

const HOME = "/Users/dannyharding";
const LIBRARY = `${HOME}/Ai/Assets/Claude/Skills`;

const iso = (daysAgo: number) =>
  new Date(Date.now() - daysAgo * 86_400_000).toISOString();

// ── Locations ──────────────────────────────────────────────────────────────

const locations: SavedLocationSummary[] = [
  {
    id: "__global__",
    label: "Global",
    path: `${HOME}/.claude/skills`,
    kind: "global",
    pathExists: true,
    issueCount: 0,
    installedSkillCount: 1,
    installedSetCount: 0,
    lastSyncedAt: iso(0),
  },
  {
    id: "loc-kit",
    label: "Kit",
    path: `${HOME}/Development/Code/Project/kit`,
    kind: "project",
    pathExists: true,
    issueCount: 2,
    installedSkillCount: 9,
    installedSetCount: 1,
    lastSyncedAt: iso(0),
  },
  {
    id: "loc-scooda",
    label: "Scooda",
    path: `${HOME}/Herd/scooda-worktrees/develop`,
    kind: "project",
    pathExists: true,
    issueCount: 0,
    installedSkillCount: 12,
    installedSetCount: 2,
    lastSyncedAt: iso(1),
  },
  {
    id: "loc-houston",
    label: "Houston",
    path: `${HOME}/Herd/houston-worktrees/main`,
    kind: "project",
    pathExists: true,
    issueCount: 1,
    installedSkillCount: 7,
    installedSetCount: 1,
    lastSyncedAt: iso(4),
  },
  {
    id: "loc-stuntrocket",
    label: "Stuntrocket",
    path: `${HOME}/Herd/stuntrocket-worktrees/develop`,
    kind: "project",
    pathExists: true,
    issueCount: 0,
    installedSkillCount: 6,
    installedSetCount: 0,
    lastSyncedAt: iso(9),
  },
  {
    id: "loc-knotbook",
    label: "KnotBook",
    path: `${HOME}/Herd/knotbook-worktrees/main`,
    kind: "project",
    pathExists: false,
    issueCount: 0,
    installedSkillCount: 4,
    installedSetCount: 0,
    lastSyncedAt: iso(41),
  },
];

// ── The library ────────────────────────────────────────────────────────────

type Seed = {
  id: string;
  summary: string;
  runs: number;
  last: number | null;
  locs: number;
  tags?: string[];
};

const skillSeeds: Seed[] = [
  { id: "clio-hooks", summary: "Install, troubleshoot and maintain Clio hook scripts, and clean up memories.", runs: 96, last: 0, locs: 1, tags: ["memory"] },
  { id: "commit-message", summary: "Generate a structured commit message or PR description from the current changes.", runs: 141, last: 0, locs: 5, tags: ["git"] },
  { id: "pre-pr-review", summary: "Bug check and code review of staged changes before creating a PR.", runs: 88, last: 1, locs: 5, tags: ["git", "review"] },
  { id: "tauri-ship", summary: "Commit, merge into develop, build the release Tauri app and install it.", runs: 34, last: 0, locs: 1, tags: ["build"] },
  { id: "uat", summary: "Run a browser-driven end-to-end UAT of a feature, then report findings with evidence.", runs: 61, last: 2, locs: 4, tags: ["testing"] },
  { id: "roadmap", summary: "Create a roadmap or spec document from the current plan.", runs: 27, last: 6, locs: 4 },
  { id: "laravel-verifier", summary: "Prove a Laravel change works by running it, not by reading it.", runs: 73, last: 1, locs: 3, tags: ["laravel"] },
  { id: "fix-herd-ssl", summary: "Diagnose and fix Herd SSL certificate issues for .test domains.", runs: 12, last: 22, locs: 3, tags: ["laravel"] },
  { id: "queue-task", summary: "Create a task file in the TaskQueue for dispatching to an agent.", runs: 19, last: 8, locs: 2 },
  { id: "reflect", summary: "Analyse session learnings and suggest improvements to existing skills.", runs: 41, last: 3, locs: 6 },
  { id: "sync-skills-guide", summary: "Sync SKILLS_GUIDE.md with the actual skills folders.", runs: 9, last: 31, locs: 1 },
  { id: "morning", summary: "Morning briefing — reconcile the week plan with Linear and propose today's list.", runs: 52, last: 1, locs: 1 },
  { id: "loop-triage", summary: "Triage the Linear board — priority, cycle, type label and acceptance stubs.", runs: 23, last: 5, locs: 1 },
  { id: "close-chat", summary: "Close the current session — decide defaults, batch remaining questions.", runs: 67, last: 0, locs: 6 },
  { id: "create-prompt", summary: "Create optimised, XML-structured prompts with intelligent depth selection.", runs: 15, last: 14, locs: 2 },
  { id: "screenshot-doctor", summary: "Diagnose failing screenshot comparisons in visual regression suites.", runs: 0, last: null, locs: 0 },
  { id: "release-notes", summary: "Draft release notes from merged pull requests since the last tag.", runs: 0, last: null, locs: 0 },
  { id: "db-snapshot", summary: "Take and restore local database snapshots for a Herd site.", runs: 0, last: null, locs: 1, tags: ["laravel"] },
];

const libraryItems: LibraryListItem[] = skillSeeds.map((s) => ({
  id: s.id,
  name: s.id,
  kind: "skill",
  archived: false,
  summary: s.summary,
  linkedLocationCount: s.locs,
  useCount30d: s.runs > 0 ? Math.max(1, Math.round(s.runs / 4)) : 0,
  lastUsedAt: s.last === null ? null : iso(s.last),
  isUnusedEverywhere: s.runs === 0,
  tags: s.tags ?? [],
  validationIssues:
    s.id === "sync-skills-guide"
      ? [
          {
            field: "description",
            message: "Description doesn't say when to use the skill.",
            suggestion: "Open with the trigger: \"Use when…\"",
            severity: "warning" as const,
          },
        ]
      : [],
  brokenSkillCount: 0,
}));

// ── Sets ───────────────────────────────────────────────────────────────────

const sets: SetSummary[] = [
  {
    id: "shipping",
    name: "Shipping",
    description: "Everything needed to get a change reviewed, merged and out.",
    scope: "global",
    ownerLocationId: null,
    skillCount: 4,
    assignedLocationCount: 4,
    path: `${LIBRARY}/sets/shipping.set.json`,
  },
  {
    id: "laravel",
    name: "Laravel",
    description: "Herd, verification and the Laravel-specific helpers.",
    scope: "global",
    ownerLocationId: null,
    skillCount: 3,
    assignedLocationCount: 3,
    path: `${LIBRARY}/sets/laravel.set.json`,
  },
  {
    id: "desktop",
    name: "Desktop apps",
    description: null,
    scope: "project",
    ownerLocationId: "loc-kit",
    skillCount: 2,
    assignedLocationCount: 1,
    path: `${HOME}/Development/Code/Project/kit/.claude/sets/desktop.set.json`,
  },
];

// ── The loadout: the app's central claim ───────────────────────────────────

function resolved(
  id: string,
  origin: SessionLoadout["groups"][number]["origin"],
  tokens: number,
  opts: { sourceLabel?: string; modelFacing?: boolean; vetoedBy?: string | null } = {}
) {
  return {
    id,
    folderName: id.includes(":") ? id.split(":")[1] : id,
    origin,
    sourceLabel: opts.sourceLabel ?? "",
    modelFacing: opts.modelFacing ?? true,
    vetoedBy: opts.vetoedBy ?? null,
    tokenEstimate: tokens,
    path: `${LIBRARY}/${id}`,
  };
}

function loadoutFor(locationId: string): SessionLoadout {
  const label =
    locations.find((l) => l.id === locationId)?.label ?? "Global";
  const isProject = locationId !== "__global__";

  const globalSkills = [resolved("clio-hooks", "global", 64)];

  const projectSkills = isProject
    ? [
        resolved("commit-message", "project", 71),
        resolved("pre-pr-review", "project", 82),
        resolved("tauri-ship", "project", 66),
        resolved("uat", "project", 94),
        resolved("roadmap", "project", 48),
        resolved("close-chat", "project", 57),
        resolved("reflect", "project", 61),
        resolved("queue-task", "project", 44, { modelFacing: false }),
        resolved("sync-skills-guide", "project", 52, { vetoedBy: "sync-skills-guide" }),
      ]
    : [];

  const groups: SessionLoadout["groups"] = [
    {
      origin: "global",
      label: "Global",
      modelFacingCount: 1,
      commandOnlyCount: 0,
      tokenEstimate: 64,
      controllable: true,
      enablementUnknown: false,
      caveat: null,
      skills: globalSkills,
    },
  ];

  if (isProject) {
    groups.push({
      origin: "project",
      label,
      modelFacingCount: 7,
      commandOnlyCount: 1,
      tokenEstimate: 479,
      controllable: true,
      enablementUnknown: false,
      caveat: null,
      skills: projectSkills,
    });
  }

  const plugins: Array<[string, string[], number]> = [
    ["impeccable", ["impeccable"], 229],
    ["ponytail", ["ponytail", "ponytail-audit", "ponytail-debt", "ponytail-gain", "ponytail-help", "ponytail-review"], 618],
    ["skill-creator", ["skill-creator"], 86],
    ["frontend-design", ["frontend-design"], 58],
  ];

  for (const [plugin, skills, tokens] of plugins) {
    groups.push({
      origin: "plugin",
      label: plugin,
      modelFacingCount: skills.length,
      commandOnlyCount: 0,
      tokenEstimate: tokens,
      controllable: true,
      enablementUnknown: false,
      caveat: null,
      skills: skills.map((s) =>
        resolved(`${plugin}:${s}`, "plugin", Math.round(tokens / skills.length), {
          sourceLabel: plugin,
        })
      ),
    });
  }

  const packs: Array<[string, number, number]> = [
    ["anthropic-skills", 15, 1827],
    ["small-business", 31, 2703],
    ["engineering", 10, 690],
    ["data", 10, 688],
    ["operations", 9, 671],
    ["human-resources", 9, 641],
    ["product-management", 8, 634],
    ["design", 7, 490],
    ["brand-voice", 3, 463],
    ["productivity", 4, 268],
  ];

  for (const [pack, count, tokens] of packs) {
    groups.push({
      origin: "account",
      label: pack,
      modelFacingCount: count,
      commandOnlyCount: 0,
      tokenEstimate: tokens,
      controllable: false,
      enablementUnknown: true,
      caveat:
        "Delivered by the desktop app, where they are also switched on and off. Kit reads the local cache, which can be stale or incomplete, so treat this as indicative rather than the full list.",
      skills: Array.from({ length: count }, (_, i) =>
        resolved(`${pack}:skill-${i + 1}`, "account", Math.round(tokens / count), {
          sourceLabel: pack,
        })
      ),
    });
  }

  const counted = groups.filter((g) => !g.enablementUnknown);

  return {
    locationId,
    locationLabel: label,
    groups,
    modelFacingCount: counted.reduce((n, g) => n + g.modelFacingCount, 0),
    commandOnlyCount: counted.reduce((n, g) => n + g.commandOnlyCount, 0),
    tokenEstimate: counted.reduce((n, g) => n + g.tokenEstimate, 0),
    vetoed: projectSkills.filter((s) => s.vetoedBy),
    deadOverrides: ["superpowers", "feature-dev"],
    unreachableOverrides: ["frontend-design"],
    settingsUnreadable: false,
  };
}

// ── Location detail ────────────────────────────────────────────────────────

function detailFor(locationId: string): LocationDetail {
  const loc = locations.find((l) => l.id === locationId) ?? locations[0];
  const isGlobal = loc.kind === "global";

  const skills = isGlobal
    ? [
        {
          skillId: "clio-hooks",
          name: "clio-hooks",
          path: `${HOME}/.claude/skills/clio-hooks`,
          linkState: "linked" as const,
          declaredInManifest: false,
          archived: false,
          source: "library" as const,
          disabled: false,
        },
      ]
    : [
        ["commit-message", "linked", true],
        ["pre-pr-review", "linked", true],
        ["tauri-ship", "linked", true],
        ["uat", "linked", true],
        ["roadmap", "linked", true],
        ["close-chat", "linked", true],
        ["reflect", "linked", true],
        ["queue-task", "linked", false],
        ["sync-skills-guide", "broken_link", true],
        ["scratch-helper", "local_only", false],
        ["db-snapshot", "declared_only", true],
      ].map(([id, state, declared]) => ({
        skillId: id as string,
        name: id as string,
        path: `${loc.path}/.claude/skills/${id}`,
        linkState: state as LocationDetail["skills"][number]["linkState"],
        declaredInManifest: declared as boolean,
        archived: false,
        source: (state === "local_only" ? "local" : "library") as "library" | "local",
        disabled: id === "queue-task",
      }));

  return {
    id: loc.id,
    label: loc.label,
    path: loc.path,
    kind: loc.kind,
    manifestPath: isGlobal ? null : `${loc.path}/.claude/settings.json`,
    notes: null,
    sets: isGlobal
      ? []
      : [
          { setId: "shipping", name: "Shipping", skillCount: 4, path: sets[0].path },
        ],
    skills,
    issues: isGlobal
      ? []
      : [
          {
            kind: "broken_link" as const,
            skillName: "sync-skills-guide",
            skillId: "sync-skills-guide",
            message: "Symlink target no longer exists in the library.",
          },
          {
            kind: "declared_missing" as const,
            skillName: "db-snapshot",
            skillId: "db-snapshot",
            message: "Declared in the manifest but no folder is linked.",
          },
        ],
    stats: {
      linkedCount: isGlobal ? 1 : 8,
      localOnlyCount: isGlobal ? 0 : 1,
      brokenCount: isGlobal ? 0 : 1,
    },
    detectedProjectTypes: isGlobal
      ? []
      : [
          { name: "Rust", markerFile: "Cargo.toml" },
          { name: "Node", markerFile: "package.json" },
        ],
    skillRecommendations: isGlobal
      ? []
      : [
          {
            skillId: "release-notes",
            skillName: "release-notes",
            projectType: "Node",
            reason: "Other Node projects here use it.",
          },
        ],
    lastScannedAt: iso(0),
  };
}

// ── Usage ──────────────────────────────────────────────────────────────────

const usageReport: UsageReport = {
  available: true,
  eventCount: 733,
  recordedSince: iso(67),
  distinctSkills: 15,
  rows: skillSeeds
    .filter((s) => s.runs > 0)
    .sort((a, b) => b.runs - a.runs)
    .map((s) => ({
      skill: s.id,
      runs: s.runs,
      lastUsedAt: s.last === null ? null : iso(s.last),
      inLibrary: true,
      projects: [
        { name: "kit", runs: Math.round(s.runs * 0.42) },
        { name: "scooda", runs: Math.round(s.runs * 0.31) },
        { name: "stuntrocket", runs: Math.round(s.runs * 0.18) },
        { name: "elsewhere", runs: Math.round(s.runs * 0.09) },
      ].filter((p) => p.runs > 0),
    })),
  neverUsed: skillSeeds.filter((s) => s.runs === 0).map((s) => s.id),
};

function usageFor(locationId: string): LocationUsage {
  const loc = locations.find((l) => l.id === locationId) ?? locations[0];
  const rows = skillSeeds.slice(0, 9).map((s) => ({
    skillId: s.id,
    name: s.id,
    usesHere: Math.max(0, Math.round(s.runs * 0.42)),
    usesAnywhere: s.runs,
    lastUsedAt: s.last === null ? null : iso(s.last),
  }));
  return {
    locationId: loc.id,
    locationLabel: loc.label,
    available: true,
    recordedSince: iso(67),
    eventCount: 733,
    linkedCount: loc.installedSkillCount,
    usedHereCount: Math.max(1, Math.round(loc.installedSkillCount * 0.55)),
    rows,
  };
}

// ── Health ─────────────────────────────────────────────────────────────────

const health: HealthCheckResult = {
  issues: [
    {
      severity: "error",
      locationId: "loc-kit",
      locationLabel: "Kit",
      description: "sync-skills-guide points at a folder that no longer exists.",
      suggestion: "Remove the broken link, or restore the skill in the library.",
      autoFixable: true,
      skillId: "sync-skills-guide",
      kind: "broken_link",
    },
    {
      severity: "warning",
      locationId: "loc-kit",
      locationLabel: "Kit",
      description: "db-snapshot is declared in the manifest but nothing is linked.",
      suggestion: "Link the skill, or drop it from the manifest.",
      autoFixable: false,
      skillId: "db-snapshot",
      kind: "declared_missing",
    },
    {
      severity: "warning",
      locationId: "loc-houston",
      locationLabel: "Houston",
      description: "scratch-helper is linked but never declared.",
      suggestion: "Add it to the manifest so the loadout is reproducible.",
      autoFixable: true,
      skillId: null,
      kind: "linked_undeclared",
    },
  ],
  locations: [
    { locationId: "__global__", locationLabel: "Global", errorCount: 0, warningCount: 0, infoCount: 0, brokenLinkCount: 0 },
    { locationId: "loc-kit", locationLabel: "Kit", errorCount: 1, warningCount: 1, infoCount: 0, brokenLinkCount: 1 },
    { locationId: "loc-scooda", locationLabel: "Scooda", errorCount: 0, warningCount: 0, infoCount: 0, brokenLinkCount: 0 },
    { locationId: "loc-houston", locationLabel: "Houston", errorCount: 0, warningCount: 1, infoCount: 0, brokenLinkCount: 0 },
    { locationId: "loc-stuntrocket", locationLabel: "Stuntrocket", errorCount: 0, warningCount: 0, infoCount: 0, brokenLinkCount: 0 },
  ],
  locationCount: 6,
  healthyCount: 4,
  warningCount: 2,
  errorCount: 1,
  scannedAt: iso(0),
};

// ── Dispatch ───────────────────────────────────────────────────────────────

const bootstrap: AppBootstrap = {
  libraryRoot: LIBRARY,
  editorCommand: "code",
  defaultView: "panel",
  showArchived: false,
  locations,
  counts: { skills: skillSeeds.length, sets: sets.length, archivedSkills: 2, brokenLinks: 1 },
};

/** The preferences as fixture mode currently holds them. Mutated by
 *  `update_preferences`, so Settings behaves rather than throwing. */
const preferences: Preferences = {
  libraryRoot: LIBRARY,
  editorCommand: bootstrap.editorCommand,
  defaultView: bootstrap.defaultView,
  showArchived: bootstrap.showArchived,
  trackSkillVersions: false,
};

const handlers: Record<string, (a: any) => unknown> = {
  get_app_bootstrap: () => bootstrap,
  // Not a no-op: the store reads the updated preferences straight off the
  // result, so returning nothing threw the moment anything was changed.
  update_preferences: (a): Preferences => {
    Object.assign(preferences, a?.prefs ?? {});
    // A real backend persists, so the next bootstrap has to agree — otherwise
    // anything that reads the saved preference on start-up, like the default
    // view, silently reverts.
    bootstrap.libraryRoot = preferences.libraryRoot;
    bootstrap.editorCommand = preferences.editorCommand;
    bootstrap.defaultView = preferences.defaultView;
    bootstrap.showArchived = preferences.showArchived;
    return { ...preferences };
  },
  list_locations: () => locations,
  // The real command takes `id`; the loadout and usage ones take `locationId`.
  get_location_detail: (a) => detailFor(a?.id ?? a?.locationId ?? "__global__"),
  list_library_items: () => libraryItems,
  list_sets: () => sets,
  get_set_detail: (a): SetDetail => {
    const s = sets.find((x) => x.id === a?.setId) ?? sets[0];
    return {
      id: s.id,
      name: s.name,
      description: s.description,
      scope: s.scope,
      ownerLocationId: s.ownerLocationId,
      path: s.path,
      skills: ["commit-message", "pre-pr-review", "uat", "close-chat"]
        .slice(0, s.skillCount)
        .map((id) => ({ id, name: id, archived: false, missing: id === "uat" && s.id === "laravel" })),
      assignedLocations: locations.slice(1, 1 + s.assignedLocationCount),
    };
  },
  get_skill_detail: (a): SkillDetail => {
    const seed = skillSeeds.find((s) => s.id === a?.skillId) ?? skillSeeds[0];
    return {
      id: seed.id,
      name: seed.id,
      path: `${LIBRARY}/${seed.id}`,
      archived: false,
      summary: seed.summary,
      linkedLocations: locations.slice(1, 1 + seed.locs),
      includedInSets: seed.id === "commit-message" ? [{ id: "shipping", name: "Shipping" }] : [],
      usage: {
        lastUsedAt: seed.last === null ? null : iso(seed.last),
        useCount30d: seed.runs > 0 ? Math.max(1, Math.round(seed.runs / 4)) : 0,
      },
    };
  },
  resolve_session_loadout: (a) => loadoutFor(a?.locationId ?? "__global__"),
  get_location_usage: (a) => usageFor(a?.locationId ?? "__global__"),
  get_usage_report: () => usageReport,
  get_usage_summary: (): UsageSummary => ({
    mostUsed: skillSeeds.slice(0, 5).map((s) => ({ id: s.id, name: s.id, count: s.runs })),
    recentlyUsed: skillSeeds.slice(0, 5).map((s) => ({ id: s.id, name: s.id, lastUsedAt: s.last === null ? null : iso(s.last) })),
    unused: skillSeeds.filter((s) => s.runs === 0).map((s) => ({ id: s.id, name: s.id })),
    suggestions: [
      "3 skills have never run. Unlinking them would save about 180 tokens a session.",
      "sync-skills-guide is switched off globally but still linked in Kit.",
    ],
  }),
  run_health_check: () => health,
  compare_locations: (): LocationComparison => ({
    locationA: { id: "loc-kit", label: "Kit", path: locations[1].path, totalSkills: 9 },
    locationB: { id: "loc-scooda", label: "Scooda", path: locations[2].path, totalSkills: 12 },
    onlyInA: [
      { skillId: "tauri-ship", name: "tauri-ship", linkState: "linked", source: "library" },
      { skillId: "scratch-helper", name: "scratch-helper", linkState: "local_only", source: "local" },
    ],
    onlyInB: [
      { skillId: "laravel-verifier", name: "laravel-verifier", linkState: "linked", source: "library" },
      { skillId: "fix-herd-ssl", name: "fix-herd-ssl", linkState: "linked", source: "library" },
      { skillId: "loop-triage", name: "loop-triage", linkState: "linked", source: "library" },
    ],
    shared: [
      { skillId: "commit-message", name: "commit-message", linkStateA: "linked", linkStateB: "linked", versionDiffers: false },
      { skillId: "pre-pr-review", name: "pre-pr-review", linkStateA: "linked", linkStateB: "linked", versionDiffers: true },
      { skillId: "uat", name: "uat", linkStateA: "linked", linkStateB: "linked", versionDiffers: false },
      { skillId: "close-chat", name: "close-chat", linkStateA: "linked", linkStateB: "linked", versionDiffers: false },
    ],
  }),
  get_watcher_status: () => ({ status: "active", watchedPath: LIBRARY }),
  get_skills_repo_status: () => ({
    path: LIBRARY,
    branch: "main",
    upstream: "origin/main",
    state: "behind",
    aheadBy: 0,
    behindBy: 3,
    hasUncommittedChanges: false,
    lastCheckedAt: iso(0),
    message: "3 commits behind origin/main.",
  }),
  get_skill_changelog: () =>
    skillSeeds.slice(0, 8).map((s, i) => ({
      skillId: s.id,
      name: s.id,
      modifiedAt: iso(i),
      sizeBytes: 2400 + i * 380,
      assignedLocations: locations.slice(1, 3).map((l) => ({ id: l.id, label: l.label })),
    })),
  read_skill_content: () =>
    "---\nname: commit-message\ndescription: Generate a structured commit message.\n---\n\n# Commit message\n\nRead the staged diff, then write the message.\n",
  get_skill_body_validation: () => [],
  get_app_data_path: () => `${HOME}/.kit`,
  resolve_skill_path: (a) => `${LIBRARY}/${a?.skillId ?? ""}`,
  preview_assignment: (a) => ({
    locationId: a?.locationId ?? "loc-kit",
    adds: [{ kind: "add_link", skillName: "release-notes", detail: "Link into .claude/skills/" }],
    removes: [],
    manifestUpdates: [{ kind: "manifest_add", skillName: "release-notes", detail: "Add to settings.json" }],
    warnings: [],
  }),
};

/** Commands that only mutate; the UI just needs them not to throw. */
const noop = new Set([
  "add_location", "remove_location", "update_location", "sync_location",
  "start_library_watcher", "stop_library_watcher",
  "open_path_in_editor", "reveal_in_finder", "archive_skill", "unarchive_skill",
  "create_set", "update_set", "delete_set", "add_skill_to_set",
  "remove_skill_from_set", "update_manifest_entry", "toggle_skill_activation",
  "apply_assignment", "bulk_assign_skills", "recheck_skills_repo_status",
  "copy_repo_pull_command", "backup_library", "restore_library",
  "remove_broken_links", "remove_missing_locations", "discover_unregistered_locations",
]);

export async function invoke<T>(command: string, args?: any): Promise<T> {
  // A touch of latency, so loading and skeleton states are designed against
  // something rather than never appearing.
  await new Promise((r) => setTimeout(r, 90));

  const handler = handlers[command];
  if (handler) return handler(args) as T;
  if (noop.has(command)) {
    if (command === "remove_missing_locations") return locations as T;
    if (command === "discover_unregistered_locations") return [] as T;
    return undefined as T;
  }
  console.warn(`[fixtures] no handler for "${command}"`);
  return undefined as T;
}

export const convertFileSrc = (p: string) => p;
