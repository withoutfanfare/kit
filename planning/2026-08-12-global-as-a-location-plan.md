# Kit Reshape: Global as a Location

**Goal:** Turn Kit from a per-project symlink assigner into a loadout manager that
answers "what loads into this session, where does it come from, and is it earning
its place" — without inventing a second product. One idea: skills flow from a
library into locations, and `~/.claude/skills` is simply the Global location.

**Architecture:** Keep the existing Vue 3 / Pinia / Tauri structure and the whole
symlink engine (`linker.rs`, `assignment.rs`, `sets.rs`). Add a resolver that
computes the effective session loadout from all four sources. Replace the dead
in-app usage counter with a reader over the existing `.skill-tracking` JSONL logs.

**Source review:** measured session on 12 Aug 2026 (see Evidence below).

---

## Evidence this plan rests on

Verified by running real headless sessions against controlled symlink set-ups:

| Skill | Set-up | Result |
| --- | --- | --- |
| `kit-loadtest-probe` | project-local dir, not overridden | PRESENT |
| `filament-admin` | in library, absent from `~/.claude/skills`, symlinked into project | PRESENT |
| `feature-sizing` | symlinked into project, `"off"` in `skillOverrides` | ABSENT |
| `data-seeder` | absent globally, not symlinked into project | ABSENT |

**The rule: symlink presence is additive and layers per-project;
`skillOverrides` is a global veto that project symlinks cannot beat.**

Consequence found in the wild: 31 deliberately created project symlinks across
scooda, stuntrocket, clipboard and grove were silently dead because the same
skill names had been switched off globally. Fixed on 12 Aug 2026 by lifting the
19 distinct overrides and removing their global symlinks instead — global
loadout unchanged at 42 model-facing skills, all 31 project links live again.

This is the bug class Kit exists to catch.

---

## Global constraints

- British English in all user-facing copy.
- Keep Rust types in `src-tauri/src/domain.rs` and TypeScript types in
  `src/types/index.ts` synchronised in the same task.
- Each frontend slice must pass `npm run build`; backend slices must also pass
  `cargo test` and `cargo clippy` from `src-tauri/`.
- Register every new command in `lib.rs`'s `invoke_handler`.
- Kit **writes symlinks only**. It reads `~/.claude/settings.json` and never
  writes to it. Turning a skill off globally means removing its symlink from
  `~/.claude/skills`, not adding a `skillOverrides` entry.

### Two safety rules that must not be skipped

1. **The Global location has no manifest.** Kit's manifest logic writes `skills`
   and `sets` arrays into `<location>/.claude/settings.json`. For the Global
   location that path resolves to the user's own Claude Code settings file, which
   governs every session. Writing to it would be destructive. `LocationKind::Global`
   must short-circuit every manifest read and write.
2. **Never unlink a skill referenced by a hook.** `~/.claude/settings.json` hooks
   invoke scripts by path, e.g.
   `python3 ~/.claude/skills/clio-hooks/scripts/session_start.py`. Removing that
   symlink breaks session start-up. Before any unlink from the Global location,
   grep the settings file for `skills/<name>` and refuse if matched.

---

## Phase 1 — Global as a location

> **Status, 12 Aug 2026:** Tasks 1 and 2 are implemented; Task 3 is not started.
> Backend verified (56 tests, clippy clean). The frontend slice is **unverified** —
> `npm install` cannot complete because `@stuntrocket/ui` is served from a
> Verdaccio registry on `localhost:4873` that is not running, so `npm run build`
> and `vue-tsc` could not be run.

### Task 1: Model the Global location

**Files:** `src-tauri/src/domain.rs`, `src-tauri/src/state.rs`,
`src-tauri/src/scanner.rs`, `src/types/index.ts`

- [x] Add `LocationKind { Global, Project }` to the domain, defaulting to
      `Project` on deserialise so existing `~/.kit/state.json` files still load.
- [x] Built as free functions `scanner::skills_dir_for(path, kind)` and
      `scanner::manifest_path_for(path, kind)` rather than methods, because most
      call sites hold a bare `&Path` rather than the `SavedLocation`.
- [x] Synthesise the Global location at state load so it always exists and cannot
      be deleted by the user.
- [x] Route every `.claude/skills` join in `scanner.rs` through `skills_dir()`.
- [x] Make `discover_manifest` return `None` for `LocationKind::Global`.
- [x] Mirror `LocationKind` in `src/types/index.ts`.

### Task 2: Surface it in the UI

**Files:** `src/views/LocationsView.vue`, `src/views/LocationDetailView.vue`,
`src/components/domain/LocationHeader.vue`, `src/stores/locationsStore.ts`

- [x] Pin Global to the top of the locations list with a distinct badge and a
      subtitle naming what it means ("loads in every session").
- [x] Hide the manifest field and the Remove action when the location is Global.
- [ ] Set-assignment UI is still shown for Global. Not harmful — the symlinks are
      still created and the manifest write is skipped, which is correct — but the
      set count will always read zero, so the control should be hidden.
- [x] Assign/unassign works identically — that is the point of the change.

### Task 1a: Safety rules (added during implementation)

- [x] `scanner::writable_manifest_path` is the single choke point for manifest
      writes and returns `None` for Global. Every hand-rolled
      `join(".claude").join("settings.json")` in the command layer now goes
      through it.
- [x] `remove_location` refuses the Global id.
- [x] `linker::remove_skill_link` refuses to unlink a skill that a hook in
      `~/.claude/settings.json` runs a script from, matching both `~/…` and
      absolute command paths. Verified against the real settings file: it
      protects `clio-hooks`.
- [x] `linker::ensure_skills_dir` returns the location path itself for Global
      rather than nesting `.claude/skills` inside it.

### Task 3: Fix location drift

**Files:** `src-tauri/src/commands/locations.rs`, `src/views/LocationsView.vue`

- [ ] Flag saved locations whose path no longer exists, with a one-click remove
      (3 currently dead).
- [ ] Offer discovered-but-unregistered projects — any directory under the known
      roots with a `.claude/skills` folder — as one-click adds (8 currently,
      including KnotBook).

---

## Phase 2 — What actually loads

### Task 4: The resolver

**Files:** new `src-tauri/src/resolver.rs`, `src-tauri/src/commands/mod.rs`,
`src-tauri/src/lib.rs`, `src-tauri/src/domain.rs`, `src/types/index.ts`

- [ ] `ResolvedSkill { name, source, model_facing, vetoed_by, token_estimate }`
      where `source` is `Global | Project | Plugin(String) | Account(String) | BuiltIn`.
- [ ] Read all four sources:
      - Global: `~/.claude/skills`
      - Project: `<location>/.claude/skills`
      - Local plugins: `~/.claude/plugins/cache/*/*/*/skills/*/SKILL.md`, gated on
        `enabledPlugins` in `~/.claude/settings.json`
      - Account-level packs: `~/.codex/plugins/cache/claude-cowork/*/*/skills/*/SKILL.md`
- [ ] Apply `skillOverrides` as a veto and record it in `vetoed_by` rather than
      dropping the row — the conflict is the interesting part.
- [ ] Honour `disable-model-invocation: true`: those skills are command-only and
      must be excluded from the model-facing count and token estimate.
- [ ] `token_estimate` = `(len(name) + len(description)) / 4`. Parse multi-line
      YAML descriptions properly; a first-line-only parser undercounts by ~5×.
- [ ] Command `resolve_session(location_id) -> SessionLoadout` with per-source
      counts and token totals.

### Task 5: The loadout view

**Files:** new `src/views/LoadoutView.vue`, new `src/stores/loadoutStore.ts`,
`src/components/layout/SidebarNav.vue`

- [ ] For the selected location, show what loads grouped by source, with token
      cost per group and a total.
- [ ] Show account-level packs as read-only with a note that they are toggled in
      the desktop app, not by Kit.

### Task 6: Health checks for the real failure modes

**Files:** `src-tauri/src/commands/health.rs`, `src/views/HealthView.vue`

- [ ] **Override vetoes a symlink** — a skill linked into a location but switched
      off globally. This is the 31-link bug; it is the headline check. Offer the
      fix: lift the override and remove the global symlink instead.
- [ ] **Dead override** — a `skillOverrides` entry naming a skill that no longer
      exists on disk (10 found on 12 Aug 2026).
- [ ] **Override cannot reach a namespaced skill** — `skillOverrides` keys match
      bare names only, so a `plugin:skill` copy keeps loading. Four skills were
      switched off yet still loading from the `anthropic-skills` pack.
- [ ] **Hook-referenced skill** — warn before unlinking, per safety rule 2.

---

## Phase 3 — Real usage

### Task 7: Read the tracking logs

**Files:** rewrite `src-tauri/src/commands/usage.rs`, `src-tauri/src/state.rs`,
`src/views/UsageView.vue`, `src/stores/usageStore.ts`

- [ ] Read `<libraryRoot>/.skill-tracking/logs/skill-usage/*.jsonl`. Each line has
      `event`, `skill`, `timestamp`, `project`, `cwd`, `session` — so usage can be
      grouped per location, not just globally.
- [ ] Delete `AppState.usage` and the only write to it, the restore path at
      `commands/backup.rs:300`. Nothing has ever incremented this counter, so every
      "N skills never used" figure the app has shown was derived from an empty map.
- [ ] Join usage against the resolver per location: "linked here, never used here"
      is the recommendation that earns its place.
- [ ] Handle a missing or unreadable log directory as "no data yet", never as zero
      uses — the old counter's failure mode was to present absence as evidence.

---

## Phase 4 — Removals

- [ ] Delete `commands/changelog.rs` and `ChangelogView.vue` (~378 lines). Kit has
      no edit event log, so the view cannot claim history.
- [ ] Delete `commands/sharing.rs` (~189 lines) unless set export is still wanted.
- [ ] Repurpose `commands/comparison.rs` and `CompareLocationsView.vue` (~682 lines)
      to "compare this project against Global", or delete them.
- [ ] Remove the routes and nav entries for anything deleted.

Sets, assignment, the linker and the manifest all stay. They are the engine.

---

## Verification

Automated checks are not sufficient here — a clean build does not prove a skill
loads. Each phase must be verified by driving the real behaviour:

- [ ] After Phase 1: unassign a skill from Global in Kit, then confirm with
      `claude -p` from a bare directory that it no longer appears.
- [ ] After Phase 2: compare Kit's resolved list for a location against the skills
      a real session in that directory reports. They must match exactly.
- [ ] After Phase 3: compare Kit's per-project usage counts against
      `jq` over the JSONL logs for the same project.
